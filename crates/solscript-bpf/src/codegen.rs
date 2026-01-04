//! Code generation from SolScript AST to LLVM IR

use crate::intrinsics::Intrinsics;
use crate::types::TypeMapper;
use crate::{BpfError, Result};
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, PointerValue};
use inkwell::AddressSpace;
use inkwell::IntPredicate;
use solscript_ast::*;
use std::collections::HashMap;

/// The main compiler that generates LLVM IR from SolScript AST
pub struct Compiler<'a, 'ctx> {
    context: &'ctx Context,
    module: &'a Module<'ctx>,
    builder: Builder<'ctx>,
    type_mapper: TypeMapper<'ctx>,
    intrinsics: Intrinsics<'ctx>,

    /// Current function being compiled
    current_function: Option<FunctionValue<'ctx>>,

    /// Local variables in the current scope
    variables: HashMap<String, PointerValue<'ctx>>,

    /// State variables (contract storage)
    state_vars: HashMap<String, (PointerValue<'ctx>, BasicTypeEnum<'ctx>)>,

    /// Current contract name
    current_contract: Option<String>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(context: &'ctx Context, module: &'a Module<'ctx>) -> Self {
        let builder = context.create_builder();
        let type_mapper = TypeMapper::new(context);
        let intrinsics = Intrinsics::new(context);

        // Declare Solana intrinsics
        intrinsics.declare_all(module);

        Self {
            context,
            module,
            builder,
            type_mapper,
            intrinsics,
            current_function: None,
            variables: HashMap::new(),
            state_vars: HashMap::new(),
            current_contract: None,
        }
    }

    /// Compile an entire program
    pub fn compile_program(&mut self, program: &Program) -> Result<()> {
        // First pass: declare all types and functions
        for item in &program.items {
            self.declare_item(item)?;
        }

        // Second pass: compile function bodies
        for item in &program.items {
            self.compile_item(item)?;
        }

        Ok(())
    }

    /// Declare an item (first pass)
    fn declare_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Contract(contract) => self.declare_contract(contract),
            Item::Struct(s) => self.declare_struct(s),
            Item::Function(f) => {
                self.declare_function(f)?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Compile an item (second pass)
    fn compile_item(&mut self, item: &Item) -> Result<()> {
        match item {
            Item::Contract(contract) => self.compile_contract(contract),
            Item::Function(f) => self.compile_function(f),
            _ => Ok(()),
        }
    }

    /// Declare a contract
    fn declare_contract(&mut self, contract: &ContractDef) -> Result<()> {
        self.current_contract = Some(contract.name.name.to_string());

        // Declare state variables as global storage
        for member in &contract.members {
            if let ContractMember::StateVar(var) = member {
                self.declare_state_var(var)?;
            }
        }

        // Declare all functions
        for member in &contract.members {
            match member {
                ContractMember::Function(f) => {
                    self.declare_function(f)?;
                }
                ContractMember::Constructor(c) => {
                    self.declare_constructor(c)?;
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Compile a contract
    fn compile_contract(&mut self, contract: &ContractDef) -> Result<()> {
        self.current_contract = Some(contract.name.name.to_string());

        // Compile all functions
        for member in &contract.members {
            match member {
                ContractMember::Function(f) => {
                    self.compile_function(f)?;
                }
                ContractMember::Constructor(c) => {
                    self.compile_constructor(c)?;
                }
                _ => {}
            }
        }

        // Generate the entrypoint function
        self.generate_entrypoint(contract)?;

        Ok(())
    }

    /// Declare a state variable
    fn declare_state_var(&mut self, var: &StateVar) -> Result<()> {
        let ty = self.type_mapper.get_type(&var.ty);
        let name = format!("state_{}", var.name.name);

        let global = self.module.add_global(ty, Some(AddressSpace::default()), &name);
        global.set_initializer(&ty.const_zero());

        self.state_vars.insert(
            var.name.name.to_string(),
            (global.as_pointer_value(), ty),
        );

        Ok(())
    }

    /// Declare a struct type
    fn declare_struct(&mut self, s: &StructDef) -> Result<()> {
        let field_types: Vec<BasicTypeEnum> = s
            .fields
            .iter()
            .map(|f| self.type_mapper.get_type(&f.ty))
            .collect();

        self.type_mapper.register_struct(&s.name.name, &field_types);
        Ok(())
    }

    /// Declare a function
    fn declare_function(&mut self, f: &FnDef) -> Result<FunctionValue<'ctx>> {
        let param_types: Vec<BasicTypeEnum> = f
            .params
            .iter()
            .map(|p| self.type_mapper.get_type(&p.ty).into())
            .collect();

        let param_types_ref: Vec<_> = param_types.iter().map(|t| (*t).into()).collect();

        let fn_type = if f.return_params.is_empty() {
            self.context.void_type().fn_type(&param_types_ref, false)
        } else {
            let ret_type = self.type_mapper.get_type(&f.return_params[0].ty);
            ret_type.fn_type(&param_types_ref, false)
        };

        let fn_name = self.mangle_function_name(&f.name.name);
        let function = self.module.add_function(&fn_name, fn_type, None);

        Ok(function)
    }

    /// Declare a constructor
    fn declare_constructor(&mut self, c: &ConstructorDef) -> Result<FunctionValue<'ctx>> {
        let param_types: Vec<BasicTypeEnum> = c
            .params
            .iter()
            .map(|p| self.type_mapper.get_type(&p.ty).into())
            .collect();

        let param_types_ref: Vec<_> = param_types.iter().map(|t| (*t).into()).collect();
        let fn_type = self.context.void_type().fn_type(&param_types_ref, false);

        let fn_name = self.mangle_function_name("constructor");
        let function = self.module.add_function(&fn_name, fn_type, None);

        Ok(function)
    }

    /// Compile a function
    fn compile_function(&mut self, f: &FnDef) -> Result<()> {
        let fn_name = self.mangle_function_name(&f.name.name);
        let function = self.module.get_function(&fn_name)
            .ok_or_else(|| BpfError::CodegenError(format!("Function {} not declared", f.name.name)))?;

        self.current_function = Some(function);
        self.variables.clear();

        // Create entry block
        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Allocate parameters
        for (i, param) in f.params.iter().enumerate() {
            let ty = self.type_mapper.get_type(&param.ty);
            let alloca = self.builder.build_alloca(ty, &param.name.name)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;

            let param_value = function.get_nth_param(i as u32)
                .ok_or_else(|| BpfError::CodegenError("Missing parameter".to_string()))?;

            self.builder.build_store(alloca, param_value)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;

            self.variables.insert(param.name.name.to_string(), alloca);
        }

        // Compile function body
        if let Some(body) = &f.body {
            self.compile_block(body)?;
        }

        // Add implicit return if needed
        if f.return_params.is_empty() {
            if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
                self.builder.build_return(None)
                    .map_err(|e| BpfError::LlvmError(e.to_string()))?;
            }
        }

        self.current_function = None;
        Ok(())
    }

    /// Compile a constructor
    fn compile_constructor(&mut self, c: &ConstructorDef) -> Result<()> {
        let fn_name = self.mangle_function_name("constructor");
        let function = self.module.get_function(&fn_name)
            .ok_or_else(|| BpfError::CodegenError("Constructor not declared".to_string()))?;

        self.current_function = Some(function);
        self.variables.clear();

        let entry = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(entry);

        // Allocate parameters
        for (i, param) in c.params.iter().enumerate() {
            let ty = self.type_mapper.get_type(&param.ty);
            let alloca = self.builder.build_alloca(ty, &param.name.name)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;

            let param_value = function.get_nth_param(i as u32)
                .ok_or_else(|| BpfError::CodegenError("Missing parameter".to_string()))?;

            self.builder.build_store(alloca, param_value)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;

            self.variables.insert(param.name.name.to_string(), alloca);
        }

        // Compile constructor body
        self.compile_block(&c.body)?;

        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_return(None)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        self.current_function = None;
        Ok(())
    }

    /// Compile a block of statements
    fn compile_block(&mut self, block: &Block) -> Result<()> {
        for stmt in &block.stmts {
            self.compile_statement(stmt)?;
        }
        Ok(())
    }

    /// Compile a statement
    fn compile_statement(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::VarDecl(decl) => self.compile_var_decl(decl),
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                Ok(())
            }
            Stmt::Return(ret) => self.compile_return(ret),
            Stmt::If(if_stmt) => self.compile_if(if_stmt),
            Stmt::While(while_stmt) => self.compile_while(while_stmt),
            Stmt::For(for_stmt) => self.compile_for(for_stmt),
            Stmt::Block(block) => self.compile_block(block),
            Stmt::Emit(emit) => self.compile_emit(emit),
            Stmt::Require(req) => self.compile_require(req),
            Stmt::Revert(rev) => self.compile_revert(rev),
            Stmt::Assignment(assign) => self.compile_assignment(assign),
            _ => Ok(()), // Skip unsupported statements for now
        }
    }

    /// Compile a variable declaration
    fn compile_var_decl(&mut self, decl: &VarDecl) -> Result<()> {
        let ty = self.type_mapper.get_type(&decl.ty);
        let alloca = self.builder.build_alloca(ty, &decl.name.name)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        if let Some(init) = &decl.init {
            let value = self.compile_expr(init)?;
            self.builder.build_store(alloca, value)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        self.variables.insert(decl.name.name.to_string(), alloca);
        Ok(())
    }

    /// Compile a return statement
    fn compile_return(&mut self, ret: &ReturnStmt) -> Result<()> {
        if let Some(value) = &ret.value {
            let val = self.compile_expr(value)?;
            self.builder.build_return(Some(&val))
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        } else {
            self.builder.build_return(None)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }
        Ok(())
    }

    /// Compile an if statement
    fn compile_if(&mut self, if_stmt: &IfStmt) -> Result<()> {
        let function = self.current_function
            .ok_or_else(|| BpfError::CodegenError("No current function".to_string()))?;

        let cond = self.compile_expr(&if_stmt.condition)?;
        let cond_bool = self.builder.build_int_compare(
            IntPredicate::NE,
            cond.into_int_value(),
            self.context.bool_type().const_zero(),
            "ifcond",
        ).map_err(|e| BpfError::LlvmError(e.to_string()))?;

        let then_bb = self.context.append_basic_block(function, "then");
        let else_bb = self.context.append_basic_block(function, "else");
        let merge_bb = self.context.append_basic_block(function, "ifcont");

        self.builder.build_conditional_branch(cond_bool, then_bb, else_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Then branch
        self.builder.position_at_end(then_bb);
        self.compile_block(&if_stmt.then_branch)?;
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        // Else branch
        self.builder.position_at_end(else_bb);
        if let Some(else_branch) = &if_stmt.else_branch {
            match else_branch.as_ref() {
                ElseBranch::Block(block) => self.compile_block(block)?,
                ElseBranch::If(nested_if) => self.compile_if(nested_if)?,
            }
        }
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(merge_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        self.builder.position_at_end(merge_bb);
        Ok(())
    }

    /// Compile a while loop
    fn compile_while(&mut self, while_stmt: &WhileStmt) -> Result<()> {
        let function = self.current_function
            .ok_or_else(|| BpfError::CodegenError("No current function".to_string()))?;

        let cond_bb = self.context.append_basic_block(function, "while.cond");
        let body_bb = self.context.append_basic_block(function, "while.body");
        let end_bb = self.context.append_basic_block(function, "while.end");

        self.builder.build_unconditional_branch(cond_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Condition
        self.builder.position_at_end(cond_bb);
        let cond = self.compile_expr(&while_stmt.condition)?;
        let cond_bool = self.builder.build_int_compare(
            IntPredicate::NE,
            cond.into_int_value(),
            self.context.bool_type().const_zero(),
            "whilecond",
        ).map_err(|e| BpfError::LlvmError(e.to_string()))?;
        self.builder.build_conditional_branch(cond_bool, body_bb, end_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Body
        self.builder.position_at_end(body_bb);
        self.compile_block(&while_stmt.body)?;
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(cond_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        self.builder.position_at_end(end_bb);
        Ok(())
    }

    /// Compile a for loop
    fn compile_for(&mut self, for_stmt: &ForStmt) -> Result<()> {
        let function = self.current_function
            .ok_or_else(|| BpfError::CodegenError("No current function".to_string()))?;

        // Initialize
        if let Some(init) = &for_stmt.init {
            self.compile_statement(init)?;
        }

        let cond_bb = self.context.append_basic_block(function, "for.cond");
        let body_bb = self.context.append_basic_block(function, "for.body");
        let incr_bb = self.context.append_basic_block(function, "for.incr");
        let end_bb = self.context.append_basic_block(function, "for.end");

        self.builder.build_unconditional_branch(cond_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Condition
        self.builder.position_at_end(cond_bb);
        if let Some(cond_expr) = &for_stmt.condition {
            let cond = self.compile_expr(cond_expr)?;
            let cond_bool = self.builder.build_int_compare(
                IntPredicate::NE,
                cond.into_int_value(),
                self.context.bool_type().const_zero(),
                "forcond",
            ).map_err(|e| BpfError::LlvmError(e.to_string()))?;
            self.builder.build_conditional_branch(cond_bool, body_bb, end_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        } else {
            self.builder.build_unconditional_branch(body_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        // Body
        self.builder.position_at_end(body_bb);
        self.compile_block(&for_stmt.body)?;
        if self.builder.get_insert_block().unwrap().get_terminator().is_none() {
            self.builder.build_unconditional_branch(incr_bb)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }

        // Increment
        self.builder.position_at_end(incr_bb);
        if let Some(incr) = &for_stmt.increment {
            self.compile_expr(incr)?;
        }
        self.builder.build_unconditional_branch(cond_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        self.builder.position_at_end(end_bb);
        Ok(())
    }

    /// Compile an assignment
    fn compile_assignment(&mut self, assign: &AssignmentStmt) -> Result<()> {
        let value = self.compile_expr(&assign.value)?;

        // Get the target pointer
        let ptr = self.compile_lvalue(&assign.target)?;

        // Handle compound assignment operators
        let final_value = match &assign.op {
            Some(op) => {
                let current = self.builder.build_load(value.get_type(), ptr, "load")
                    .map_err(|e| BpfError::LlvmError(e.to_string()))?;
                self.compile_binary_op(op, current, value)?
            }
            None => value,
        };

        self.builder.build_store(ptr, final_value)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        Ok(())
    }

    /// Compile an lvalue (target of assignment)
    fn compile_lvalue(&mut self, expr: &Expr) -> Result<PointerValue<'ctx>> {
        match expr {
            Expr::Ident(ident) => {
                // Check local variables first
                if let Some(ptr) = self.variables.get(&ident.name) {
                    return Ok(*ptr);
                }
                // Then state variables
                if let Some((ptr, _)) = self.state_vars.get(&ident.name) {
                    return Ok(*ptr);
                }
                Err(BpfError::CodegenError(format!("Undefined variable: {}", ident.name)))
            }
            Expr::FieldAccess(access) => {
                // Handle field access (e.g., struct.field)
                let base_ptr = self.compile_lvalue(&access.object)?;
                // TODO: Compute field offset
                Ok(base_ptr)
            }
            Expr::Index(index) => {
                // Handle array indexing
                let base_ptr = self.compile_lvalue(&index.object)?;
                let idx = self.compile_expr(&index.index)?;

                self.builder.build_gep(
                    self.context.i64_type(),
                    base_ptr,
                    &[idx.into_int_value()],
                    "arrayidx",
                ).map_err(|e| BpfError::LlvmError(e.to_string()))
            }
            _ => Err(BpfError::CodegenError("Invalid lvalue".to_string())),
        }
    }

    /// Compile an emit statement (event logging)
    fn compile_emit(&mut self, emit: &EmitStmt) -> Result<()> {
        // Emit events using sol_log
        if let Some(log_fn) = self.intrinsics.get_sol_log(self.module) {
            // Create event message
            let event_name = &emit.event.name;
            let msg = format!("Event: {}", event_name);
            let msg_const = self.context.const_string(msg.as_bytes(), false);
            let msg_global = self.module.add_global(msg_const.get_type(), None, "event_msg");
            msg_global.set_initializer(&msg_const);

            let msg_ptr = msg_global.as_pointer_value();
            let msg_len = self.context.i64_type().const_int(msg.len() as u64, false);

            self.builder.build_call(log_fn, &[msg_ptr.into(), msg_len.into()], "log")
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }
        Ok(())
    }

    /// Compile a require statement
    fn compile_require(&mut self, req: &RequireStmt) -> Result<()> {
        let function = self.current_function
            .ok_or_else(|| BpfError::CodegenError("No current function".to_string()))?;

        let cond = self.compile_expr(&req.condition)?;
        let cond_bool = self.builder.build_int_compare(
            IntPredicate::NE,
            cond.into_int_value(),
            self.context.bool_type().const_zero(),
            "require",
        ).map_err(|e| BpfError::LlvmError(e.to_string()))?;

        let pass_bb = self.context.append_basic_block(function, "require.pass");
        let fail_bb = self.context.append_basic_block(function, "require.fail");

        self.builder.build_conditional_branch(cond_bool, pass_bb, fail_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Fail block - call sol_panic
        self.builder.position_at_end(fail_bb);
        if let Some(panic_fn) = self.intrinsics.get_sol_panic(self.module) {
            let msg = req.message.as_deref().unwrap_or("Requirement failed");
            let msg_const = self.context.const_string(msg.as_bytes(), false);
            let msg_global = self.module.add_global(msg_const.get_type(), None, "panic_msg");
            msg_global.set_initializer(&msg_const);

            self.builder.build_call(
                panic_fn,
                &[
                    msg_global.as_pointer_value().into(),
                    self.context.i64_type().const_int(msg.len() as u64, false).into(),
                    self.context.i64_type().const_int(0, false).into(),
                    self.context.i64_type().const_int(0, false).into(),
                ],
                "panic",
            ).map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }
        self.builder.build_unreachable()
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        self.builder.position_at_end(pass_bb);
        Ok(())
    }

    /// Compile a revert statement
    fn compile_revert(&mut self, _rev: &RevertStmt) -> Result<()> {
        if let Some(panic_fn) = self.intrinsics.get_sol_panic(self.module) {
            let msg = "Reverted";
            let msg_const = self.context.const_string(msg.as_bytes(), false);
            let msg_global = self.module.add_global(msg_const.get_type(), None, "revert_msg");
            msg_global.set_initializer(&msg_const);

            self.builder.build_call(
                panic_fn,
                &[
                    msg_global.as_pointer_value().into(),
                    self.context.i64_type().const_int(msg.len() as u64, false).into(),
                    self.context.i64_type().const_int(0, false).into(),
                    self.context.i64_type().const_int(0, false).into(),
                ],
                "panic",
            ).map_err(|e| BpfError::LlvmError(e.to_string()))?;
        }
        self.builder.build_unreachable()
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        Ok(())
    }

    /// Compile an expression
    fn compile_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>> {
        match expr {
            Expr::Literal(lit) => self.compile_literal(lit),
            Expr::Ident(ident) => self.compile_ident(ident),
            Expr::Binary(bin) => self.compile_binary(bin),
            Expr::Unary(unary) => self.compile_unary(unary),
            Expr::Call(call) => self.compile_call(call),
            Expr::FieldAccess(access) => self.compile_field_access(access),
            Expr::Index(index) => self.compile_index(index),
            Expr::Ternary(ternary) => self.compile_ternary(ternary),
            _ => Err(BpfError::Unsupported(format!("Expression type: {:?}", expr))),
        }
    }

    /// Compile a literal
    fn compile_literal(&mut self, lit: &Literal) -> Result<BasicValueEnum<'ctx>> {
        match lit {
            Literal::Integer(n) => {
                Ok(self.context.i64_type().const_int(*n as u64, false).into())
            }
            Literal::Bool(b) => {
                Ok(self.context.bool_type().const_int(*b as u64, false).into())
            }
            Literal::String(s) => {
                let str_const = self.context.const_string(s.as_bytes(), false);
                let global = self.module.add_global(str_const.get_type(), None, "str");
                global.set_initializer(&str_const);
                Ok(global.as_pointer_value().into())
            }
            Literal::Address(addr) => {
                // Address is 32 bytes
                let bytes: Vec<u8> = if addr.starts_with("0x") {
                    hex::decode(&addr[2..]).unwrap_or_else(|_| vec![0; 32])
                } else {
                    vec![0; 32]
                };
                let arr_type = self.context.i8_type().array_type(32);
                let values: Vec<_> = bytes.iter()
                    .map(|b| self.context.i8_type().const_int(*b as u64, false))
                    .collect();
                Ok(self.context.i8_type().const_array(&values).into())
            }
            _ => Ok(self.context.i64_type().const_int(0, false).into()),
        }
    }

    /// Compile an identifier
    fn compile_ident(&mut self, ident: &Ident) -> Result<BasicValueEnum<'ctx>> {
        // Check local variables first
        if let Some(ptr) = self.variables.get(&ident.name) {
            let ty = self.context.i64_type(); // TODO: Get actual type
            let value = self.builder.build_load(ty, *ptr, &ident.name)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
            return Ok(value);
        }

        // Check state variables
        if let Some((ptr, ty)) = self.state_vars.get(&ident.name) {
            let value = self.builder.build_load(*ty, *ptr, &ident.name)
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;
            return Ok(value);
        }

        Err(BpfError::CodegenError(format!("Undefined variable: {}", ident.name)))
    }

    /// Compile a binary expression
    fn compile_binary(&mut self, bin: &BinaryExpr) -> Result<BasicValueEnum<'ctx>> {
        let left = self.compile_expr(&bin.left)?;
        let right = self.compile_expr(&bin.right)?;
        self.compile_binary_op(&bin.op, left, right)
    }

    /// Compile a binary operation
    fn compile_binary_op(
        &mut self,
        op: &BinaryOp,
        left: BasicValueEnum<'ctx>,
        right: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>> {
        let lhs = left.into_int_value();
        let rhs = right.into_int_value();

        let result = match op {
            BinaryOp::Add => self.builder.build_int_add(lhs, rhs, "add"),
            BinaryOp::Sub => self.builder.build_int_sub(lhs, rhs, "sub"),
            BinaryOp::Mul => self.builder.build_int_mul(lhs, rhs, "mul"),
            BinaryOp::Div => self.builder.build_int_unsigned_div(lhs, rhs, "div"),
            BinaryOp::Mod => self.builder.build_int_unsigned_rem(lhs, rhs, "mod"),
            BinaryOp::Eq => self.builder.build_int_compare(IntPredicate::EQ, lhs, rhs, "eq"),
            BinaryOp::Ne => self.builder.build_int_compare(IntPredicate::NE, lhs, rhs, "ne"),
            BinaryOp::Lt => self.builder.build_int_compare(IntPredicate::ULT, lhs, rhs, "lt"),
            BinaryOp::Le => self.builder.build_int_compare(IntPredicate::ULE, lhs, rhs, "le"),
            BinaryOp::Gt => self.builder.build_int_compare(IntPredicate::UGT, lhs, rhs, "gt"),
            BinaryOp::Ge => self.builder.build_int_compare(IntPredicate::UGE, lhs, rhs, "ge"),
            BinaryOp::And => self.builder.build_and(lhs, rhs, "and"),
            BinaryOp::Or => self.builder.build_or(lhs, rhs, "or"),
            BinaryOp::BitAnd => self.builder.build_and(lhs, rhs, "bitand"),
            BinaryOp::BitOr => self.builder.build_or(lhs, rhs, "bitor"),
            BinaryOp::BitXor => self.builder.build_xor(lhs, rhs, "bitxor"),
            BinaryOp::Shl => self.builder.build_left_shift(lhs, rhs, "shl"),
            BinaryOp::Shr => self.builder.build_right_shift(lhs, rhs, false, "shr"),
            _ => return Err(BpfError::Unsupported(format!("Binary op: {:?}", op))),
        }.map_err(|e| BpfError::LlvmError(e.to_string()))?;

        Ok(result.into())
    }

    /// Compile a unary expression
    fn compile_unary(&mut self, unary: &UnaryExpr) -> Result<BasicValueEnum<'ctx>> {
        let operand = self.compile_expr(&unary.operand)?;
        let int_val = operand.into_int_value();

        let result = match unary.op {
            UnaryOp::Neg => {
                self.builder.build_int_neg(int_val, "neg")
                    .map_err(|e| BpfError::LlvmError(e.to_string()))?
            }
            UnaryOp::Not => {
                self.builder.build_not(int_val, "not")
                    .map_err(|e| BpfError::LlvmError(e.to_string()))?
            }
            UnaryOp::BitNot => {
                self.builder.build_not(int_val, "bitnot")
                    .map_err(|e| BpfError::LlvmError(e.to_string()))?
            }
        };

        Ok(result.into())
    }

    /// Compile a function call
    fn compile_call(&mut self, call: &CallExpr) -> Result<BasicValueEnum<'ctx>> {
        let fn_name = match &*call.function {
            Expr::Ident(ident) => ident.name.clone(),
            Expr::FieldAccess(access) => {
                // Handle method calls like msg.sender
                if let Expr::Ident(obj) = &*access.object {
                    if obj.name == "msg" && access.field.name == "sender" {
                        // Return a placeholder for msg.sender
                        return Ok(self.context.i8_type().array_type(32).const_zero().into());
                    }
                }
                access.field.name.clone()
            }
            _ => return Err(BpfError::CodegenError("Invalid function call".to_string())),
        };

        let mangled_name = self.mangle_function_name(&fn_name);

        if let Some(function) = self.module.get_function(&mangled_name) {
            let args: Result<Vec<_>> = call.args.iter()
                .map(|arg| self.compile_expr(arg).map(|v| v.into()))
                .collect();
            let args = args?;

            let result = self.builder.build_call(function, &args, "call")
                .map_err(|e| BpfError::LlvmError(e.to_string()))?;

            result.try_as_basic_value()
                .left()
                .ok_or_else(|| BpfError::CodegenError("Function returns void".to_string()))
        } else {
            Err(BpfError::CodegenError(format!("Unknown function: {}", fn_name)))
        }
    }

    /// Compile field access
    fn compile_field_access(&mut self, access: &FieldAccessExpr) -> Result<BasicValueEnum<'ctx>> {
        // Handle special cases like msg.sender
        if let Expr::Ident(obj) = &*access.object {
            if obj.name == "msg" {
                match access.field.name.as_str() {
                    "sender" => {
                        // Return a placeholder address
                        return Ok(self.context.i8_type().array_type(32).const_zero().into());
                    }
                    "value" => {
                        return Ok(self.context.i64_type().const_int(0, false).into());
                    }
                    _ => {}
                }
            }
            if obj.name == "block" {
                match access.field.name.as_str() {
                    "timestamp" => {
                        return Ok(self.context.i64_type().const_int(0, false).into());
                    }
                    "number" => {
                        return Ok(self.context.i64_type().const_int(0, false).into());
                    }
                    _ => {}
                }
            }
        }

        // Regular field access
        let base = self.compile_expr(&access.object)?;
        // TODO: Implement proper struct field access
        Ok(base)
    }

    /// Compile array indexing
    fn compile_index(&mut self, index: &IndexExpr) -> Result<BasicValueEnum<'ctx>> {
        let base_ptr = self.compile_lvalue(&index.object)?;
        let idx = self.compile_expr(&index.index)?;

        let elem_ptr = self.builder.build_gep(
            self.context.i64_type(),
            base_ptr,
            &[idx.into_int_value()],
            "arrayidx",
        ).map_err(|e| BpfError::LlvmError(e.to_string()))?;

        let value = self.builder.build_load(self.context.i64_type(), elem_ptr, "load")
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        Ok(value)
    }

    /// Compile a ternary expression
    fn compile_ternary(&mut self, ternary: &TernaryExpr) -> Result<BasicValueEnum<'ctx>> {
        let function = self.current_function
            .ok_or_else(|| BpfError::CodegenError("No current function".to_string()))?;

        let cond = self.compile_expr(&ternary.condition)?;
        let cond_bool = self.builder.build_int_compare(
            IntPredicate::NE,
            cond.into_int_value(),
            self.context.bool_type().const_zero(),
            "terncond",
        ).map_err(|e| BpfError::LlvmError(e.to_string()))?;

        let then_bb = self.context.append_basic_block(function, "tern.then");
        let else_bb = self.context.append_basic_block(function, "tern.else");
        let merge_bb = self.context.append_basic_block(function, "tern.merge");

        self.builder.build_conditional_branch(cond_bool, then_bb, else_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        // Then
        self.builder.position_at_end(then_bb);
        let then_val = self.compile_expr(&ternary.then_expr)?;
        self.builder.build_unconditional_branch(merge_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        let then_bb = self.builder.get_insert_block().unwrap();

        // Else
        self.builder.position_at_end(else_bb);
        let else_val = self.compile_expr(&ternary.else_expr)?;
        self.builder.build_unconditional_branch(merge_bb)
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        let else_bb = self.builder.get_insert_block().unwrap();

        // Merge with phi
        self.builder.position_at_end(merge_bb);
        let phi = self.builder.build_phi(then_val.get_type(), "ternphi")
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;
        phi.add_incoming(&[(&then_val, then_bb), (&else_val, else_bb)]);

        Ok(phi.as_basic_value())
    }

    /// Generate the Solana entrypoint function
    fn generate_entrypoint(&mut self, _contract: &ContractDef) -> Result<()> {
        let i64_type = self.context.i64_type();
        let ptr_type = self.context.ptr_type(AddressSpace::default());

        // entrypoint(input: *const u8) -> u64
        let fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        let entrypoint = self.module.add_function("entrypoint", fn_type, None);

        let entry = self.context.append_basic_block(entrypoint, "entry");
        self.builder.position_at_end(entry);

        // Parse instruction data and dispatch to the appropriate function
        // For now, just return success
        self.builder.build_return(Some(&i64_type.const_int(0, false)))
            .map_err(|e| BpfError::LlvmError(e.to_string()))?;

        Ok(())
    }

    /// Mangle a function name
    fn mangle_function_name(&self, name: &str) -> String {
        if let Some(contract) = &self.current_contract {
            format!("{}_{}", contract, name)
        } else {
            name.to_string()
        }
    }
}
