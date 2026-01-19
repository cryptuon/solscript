//! Type checker implementation for Solidity-style SolScript

use indexmap::IndexMap;
use smol_str::SmolStr;
use solscript_ast::{self as ast, Span};

use crate::error::TypeError;
use crate::scope::{ScopeKind, SymbolTable};
use crate::types::{
    ContractDef, EnumDef, ErrorDef, ErrorParam, EventDef, EventParam, FunctionType, InterfaceDef,
    ModifierType, NamedType, PrimitiveType, StructDef, Type, TypeDef, TypeVar,
};

/// The type checker
pub struct TypeChecker {
    /// Symbol table
    symbols: SymbolTable,
    /// Source code for error reporting
    source: String,
    /// Type variable counter
    next_type_var: u32,
    /// Collected errors
    errors: Vec<TypeError>,
    /// Current return type (when in a function)
    return_type: Option<Type>,
    /// Current self type (when in a contract)
    self_type: Option<Type>,
    /// All contracts for inheritance lookup
    contracts: std::collections::HashMap<String, ast::ContractDef>,
}

impl TypeChecker {
    pub fn new(source: String) -> Self {
        Self {
            symbols: SymbolTable::new(),
            source,
            next_type_var: 0,
            errors: Vec::new(),
            return_type: None,
            self_type: None,
            contracts: std::collections::HashMap::new(),
        }
    }

    /// Check a program
    pub fn check_program(&mut self, program: &ast::Program) -> Result<(), Vec<TypeError>> {
        // First pass: collect all type definitions
        for item in &program.items {
            self.collect_type_def(item);
        }

        // Collect all contracts for inheritance lookup
        for item in &program.items {
            if let ast::Item::Contract(c) = item {
                self.contracts.insert(c.name.name.to_string(), c.clone());
            }
        }

        // Second pass: check all items
        for item in &program.items {
            self.check_item(item);
        }

        if self.errors.is_empty() {
            Ok(())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    /// Generate a fresh type variable
    fn fresh_type_var(&mut self) -> Type {
        let var = TypeVar(self.next_type_var);
        self.next_type_var += 1;
        Type::Var(var)
    }

    /// Get the span as a tuple
    fn span(&self, span: Span) -> (usize, usize) {
        (span.start, span.end)
    }

    /// Report an error
    fn error(&mut self, err: TypeError) {
        self.errors.push(err);
    }

    // =========================================================================
    // Type Definition Collection
    // =========================================================================

    fn collect_type_def(&mut self, item: &ast::Item) {
        match item {
            ast::Item::Struct(s) => {
                let def = self.build_struct_def(s);
                self.symbols.define_type(s.name.name.clone(), TypeDef::Struct(def));
            }
            ast::Item::Enum(e) => {
                let def = self.build_enum_def(e);
                self.symbols.define_type(e.name.name.clone(), TypeDef::Enum(def));
            }
            ast::Item::Contract(c) => {
                let def = self.build_contract_def(c);
                self.symbols.define_type(c.name.name.clone(), TypeDef::Contract(def));

                // Also register events, errors, structs, and enums defined inside the contract
                for member in &c.members {
                    match member {
                        ast::ContractMember::Event(e) => {
                            let event_def = self.build_event_def(e);
                            self.symbols.define_type(e.name.name.clone(), TypeDef::Event(event_def));
                        }
                        ast::ContractMember::Error(e) => {
                            let error_def = self.build_error_def(e);
                            self.symbols.define_type(e.name.name.clone(), TypeDef::Error(error_def));
                        }
                        ast::ContractMember::Struct(s) => {
                            let struct_def = self.build_struct_def(s);
                            self.symbols.define_type(s.name.name.clone(), TypeDef::Struct(struct_def));
                        }
                        ast::ContractMember::Enum(e) => {
                            let enum_def = self.build_enum_def(e);
                            self.symbols.define_type(e.name.name.clone(), TypeDef::Enum(enum_def));
                        }
                        _ => {}
                    }
                }
            }
            ast::Item::Interface(i) => {
                let def = self.build_interface_def(i);
                self.symbols.define_type(i.name.name.clone(), TypeDef::Interface(def));
            }
            ast::Item::Event(e) => {
                let def = self.build_event_def(e);
                self.symbols.define_type(e.name.name.clone(), TypeDef::Event(def));
            }
            ast::Item::Error(e) => {
                let def = self.build_error_def(e);
                self.symbols.define_type(e.name.name.clone(), TypeDef::Error(def));
            }
            _ => {}
        }
    }

    fn build_struct_def(&mut self, s: &ast::StructDef) -> StructDef {
        let type_params = s
            .generic_params
            .as_ref()
            .map(|g| g.params.iter().map(|p| p.name.name.clone()).collect())
            .unwrap_or_default();

        let mut fields = IndexMap::new();
        for field in &s.fields {
            let ty = self.resolve_type_expr(&field.ty);
            fields.insert(field.name.name.clone(), ty);
        }

        StructDef {
            name: s.name.name.clone(),
            type_params,
            fields,
        }
    }

    fn build_enum_def(&mut self, e: &ast::EnumDef) -> EnumDef {
        // Solidity-style enums: simple variants only
        let variants = e.variants.iter().map(|v| v.name.name.clone()).collect();

        EnumDef {
            name: e.name.name.clone(),
            variants,
        }
    }

    fn build_contract_def(&mut self, c: &ast::ContractDef) -> ContractDef {
        let type_params = Vec::new(); // Contracts don't have generics in Solidity-style

        let bases: Vec<SmolStr> = c.bases.iter().map(|b| b.name().clone()).collect();

        let mut state_fields = IndexMap::new();
        let mut methods = IndexMap::new();
        let mut modifiers = IndexMap::new();

        for member in &c.members {
            match member {
                ast::ContractMember::StateVar(f) => {
                    state_fields.insert(f.name.name.clone(), self.resolve_type_expr(&f.ty));
                }
                ast::ContractMember::Function(f) => {
                    let fn_ty = self.build_function_type(f);
                    methods.insert(f.name.name.clone(), fn_ty);
                }
                ast::ContractMember::Constructor(_) => {} // Constructor handled separately
                ast::ContractMember::Modifier(m) => {
                    let mod_ty = self.build_modifier_type(m);
                    modifiers.insert(m.name.name.clone(), mod_ty);
                }
                ast::ContractMember::Event(_) | ast::ContractMember::Error(_) => {
                    // Events and errors are already registered at the program level
                }
                ast::ContractMember::Struct(_) | ast::ContractMember::Enum(_) => {
                    // Structs and enums defined inside contracts are handled at the program level
                }
            }
        }

        ContractDef {
            name: c.name.name.clone(),
            type_params,
            bases,
            state_fields,
            methods,
            modifiers,
        }
    }

    fn build_modifier_type(&mut self, m: &ast::ModifierDef) -> ModifierType {
        let params: Vec<Type> = m.params.iter().map(|p| self.resolve_type_expr(&p.ty)).collect();
        ModifierType {
            name: m.name.name.clone(),
            params,
        }
    }

    fn build_event_def(&mut self, e: &ast::EventDef) -> EventDef {
        let params: Vec<EventParam> = e
            .params
            .iter()
            .map(|p| EventParam {
                name: p.name.name.clone(),
                ty: self.resolve_type_expr(&p.ty),
                indexed: p.indexed,
            })
            .collect();
        EventDef {
            name: e.name.name.clone(),
            params,
        }
    }

    fn build_error_def(&mut self, e: &ast::ErrorDef) -> ErrorDef {
        let params: Vec<ErrorParam> = e
            .params
            .iter()
            .map(|p| ErrorParam {
                name: p.name.name.clone(),
                ty: self.resolve_type_expr(&p.ty),
            })
            .collect();
        ErrorDef {
            name: e.name.name.clone(),
            params,
        }
    }

    fn build_interface_def(&mut self, i: &ast::InterfaceDef) -> InterfaceDef {
        let bases: Vec<SmolStr> = i.bases.iter().map(|b| b.name().clone()).collect();

        let mut methods = IndexMap::new();
        for sig in &i.members {
            let fn_ty = self.build_fn_sig_type(sig);
            methods.insert(sig.name.name.clone(), fn_ty);
        }

        InterfaceDef {
            name: i.name.name.clone(),
            bases,
            methods,
        }
    }

    fn build_function_type(&mut self, f: &ast::FnDef) -> FunctionType {
        let params: Vec<Type> = f.params.iter().map(|p| self.resolve_type_expr(&p.ty)).collect();

        // Get return type from return_params
        let return_type = if f.return_params.is_empty() {
            Type::Unit
        } else if f.return_params.len() == 1 {
            self.resolve_type_expr(&f.return_params[0].ty)
        } else {
            // Multiple return values -> tuple
            let types: Vec<Type> = f.return_params.iter().map(|rp| self.resolve_type_expr(&rp.ty)).collect();
            Type::Tuple(types)
        };

        FunctionType {
            params,
            return_type: Box::new(return_type),
        }
    }

    fn build_fn_sig_type(&mut self, sig: &ast::FnSig) -> FunctionType {
        let params: Vec<Type> = sig.params.iter().map(|p| self.resolve_type_expr(&p.ty)).collect();

        let return_type = if sig.return_params.is_empty() {
            Type::Unit
        } else if sig.return_params.len() == 1 {
            self.resolve_type_expr(&sig.return_params[0].ty)
        } else {
            let types: Vec<Type> = sig.return_params.iter().map(|rp| self.resolve_type_expr(&rp.ty)).collect();
            Type::Tuple(types)
        };

        FunctionType {
            params,
            return_type: Box::new(return_type),
        }
    }

    // =========================================================================
    // Type Resolution
    // =========================================================================

    fn resolve_type_expr(&mut self, ty: &ast::TypeExpr) -> Type {
        match ty {
            ast::TypeExpr::Path(path) => self.resolve_type_path(path),
            ast::TypeExpr::Array(arr) => {
                let elem = self.resolve_type_path(&arr.element);
                // Handle multiple dimensions
                let mut current_type = elem;
                for size in arr.sizes.iter().rev() {
                    current_type = match size {
                        Some(n) => Type::Array(Box::new(current_type), *n),
                        None => Type::DynamicArray(Box::new(current_type)),
                    };
                }
                current_type
            }
            ast::TypeExpr::Mapping(mapping) => {
                let key = self.resolve_type_expr(&mapping.key);
                let value = self.resolve_type_expr(&mapping.value);
                Type::Mapping(Box::new(key), Box::new(value))
            }
            ast::TypeExpr::Tuple(tuple) => {
                let elems: Vec<Type> = tuple.elements.iter().map(|t| self.resolve_type_expr(t)).collect();
                Type::Tuple(elems)
            }
        }
    }

    fn resolve_type_path(&mut self, path: &ast::TypePath) -> Type {
        let name = path.name();

        // Check for primitive types
        if let Some(prim) = PrimitiveType::from_str(name.as_str()) {
            return Type::Primitive(prim);
        }

        // Look up user-defined type
        if self.symbols.lookup_type(name).is_some() {
            let type_args = path
                .generic_args
                .as_ref()
                .map(|g| g.args.iter().map(|a| self.resolve_type_expr(a)).collect())
                .unwrap_or_default();
            Type::Named(NamedType::with_args(name.clone(), type_args))
        } else {
            self.error(TypeError::undefined_type(name, self.span(path.span), &self.source));
            Type::Error
        }
    }

    // =========================================================================
    // Item Checking
    // =========================================================================

    fn check_item(&mut self, item: &ast::Item) {
        match item {
            ast::Item::Contract(c) => self.check_contract(c),
            ast::Item::Struct(s) => self.check_struct(s),
            ast::Item::Enum(e) => self.check_enum(e),
            ast::Item::Function(f) => self.check_function(f),
            ast::Item::Interface(_) => {} // Already collected
            ast::Item::Import(_) => {}    // Handled separately
            ast::Item::Event(_) => {}     // Events are just declarations
            ast::Item::Error(_) => {}     // Errors are just declarations
        }
    }

    fn check_contract(&mut self, contract: &ast::ContractDef) {
        let contract_type = Type::Named(NamedType::new(contract.name.name.clone()));
        self.self_type = Some(contract_type);

        self.symbols.push_scope(ScopeKind::Contract);

        // First, add inherited state variables from base contracts
        for base in &contract.bases {
            let base_name = base.segments.first().map(|s| s.name.as_str()).unwrap_or("");
            if let Some(base_contract) = self.contracts.get(base_name).cloned() {
                // Add inherited state variables
                for member in &base_contract.members {
                    if let ast::ContractMember::StateVar(f) = member {
                        let ty = self.resolve_type_expr(&f.ty);
                        self.symbols.define_variable(f.name.name.clone(), ty, true);
                    }
                }
            }
        }

        // Add this contract's state variables to scope
        for member in &contract.members {
            if let ast::ContractMember::StateVar(f) = member {
                let ty = self.resolve_type_expr(&f.ty);
                self.symbols.define_variable(f.name.name.clone(), ty, true);
            }
        }

        // First pass: Register all function signatures so they can be called internally
        for member in &contract.members {
            if let ast::ContractMember::Function(f) = member {
                let fn_ty = self.build_function_type(f);
                self.symbols.define_variable(
                    f.name.name.clone(),
                    Type::Function(fn_ty),
                    false,
                );
            }
        }

        // Second pass: Check constructors and function bodies
        for member in &contract.members {
            match member {
                ast::ContractMember::Constructor(c) => self.check_constructor(c),
                ast::ContractMember::Function(f) => self.check_function(f),
                ast::ContractMember::Modifier(m) => self.check_modifier_def(m),
                ast::ContractMember::StateVar(_) => {} // Already added
                ast::ContractMember::Event(_) => {}    // Events are declarations
                ast::ContractMember::Error(_) => {}    // Errors are declarations
                ast::ContractMember::Struct(s) => self.check_struct(s),
                ast::ContractMember::Enum(e) => self.check_enum(e),
            }
        }

        self.symbols.pop_scope();
        self.self_type = None;
    }

    fn check_struct(&mut self, s: &ast::StructDef) {
        // Check for duplicate fields
        let mut seen_fields = std::collections::HashSet::new();
        for field in &s.fields {
            let field_name = field.name.name.as_str();
            if seen_fields.contains(field_name) {
                self.error(TypeError::DuplicateDefinition {
                    name: field_name.to_string(),
                    span: miette::SourceSpan::new(field.name.span.start.into(), (field.name.span.end - field.name.span.start).into()),
                    src: self.source.clone(),
                });
            } else {
                seen_fields.insert(field_name.to_string());
            }

            // Verify field type is valid
            let _ = self.resolve_type_expr(&field.ty);
        }
    }

    fn check_enum(&mut self, e: &ast::EnumDef) {
        // Check for duplicate variants
        let mut seen_variants = std::collections::HashSet::new();
        for variant in &e.variants {
            let variant_name = variant.name.name.as_str();
            if seen_variants.contains(variant_name) {
                self.error(TypeError::DuplicateDefinition {
                    name: variant_name.to_string(),
                    span: miette::SourceSpan::new(variant.name.span.start.into(), (variant.name.span.end - variant.name.span.start).into()),
                    src: self.source.clone(),
                });
            } else {
                seen_variants.insert(variant_name.to_string());
            }
        }
    }

    fn check_function(&mut self, f: &ast::FnDef) {
        let fn_ty = self.build_function_type(f);
        self.return_type = Some((*fn_ty.return_type).clone());

        // Validate modifier invocations
        for modifier in &f.modifiers {
            self.check_modifier_invocation(modifier);
        }

        self.symbols.push_scope(ScopeKind::Function);

        // Add parameters to scope
        for param in &f.params {
            let ty = self.resolve_type_expr(&param.ty);
            self.symbols.define_variable(param.name.name.clone(), ty, false);
        }

        // Check function body (if present - abstract functions have no body)
        if let Some(body) = &f.body {
            self.check_block(body);
        }

        self.symbols.pop_scope();
        self.return_type = None;
    }

    fn check_modifier_invocation(&mut self, modifier: &ast::ModifierInvocation) {
        let modifier_name = &modifier.name.name;

        // Look up modifier in the current contract context and base contracts
        if let Some(contract_type) = &self.self_type {
            if let Type::Named(named) = contract_type {
                if let Some(TypeDef::Contract(contract_def)) = self.symbols.lookup_type(&named.name) {
                    // First try this contract
                    if let Some(mod_type) = contract_def.modifiers.get(modifier_name).cloned() {
                        self.validate_modifier_args(modifier, &mod_type);
                        return;
                    }

                    // Then try base contracts
                    for base_name in &contract_def.bases {
                        if let Some(TypeDef::Contract(base_def)) = self.symbols.lookup_type(base_name) {
                            if let Some(mod_type) = base_def.modifiers.get(modifier_name).cloned() {
                                self.validate_modifier_args(modifier, &mod_type);
                                return;
                            }
                        }
                    }
                }
            }
        }

        // Modifier not found
        self.error(TypeError::UndefinedModifier {
            name: modifier_name.to_string(),
            span: miette::SourceSpan::new(modifier.name.span.start.into(), (modifier.name.span.end - modifier.name.span.start).into()),
            src: self.source.clone(),
        });
    }

    fn validate_modifier_args(&mut self, modifier: &ast::ModifierInvocation, mod_type: &ModifierType) {
        // Check argument count
        if modifier.args.len() != mod_type.params.len() {
            self.error(TypeError::wrong_arg_count(
                mod_type.params.len(),
                modifier.args.len(),
                self.span(modifier.span),
                &self.source,
            ));
            return;
        }

        // Check argument types
        for (arg, param_ty) in modifier.args.iter().zip(mod_type.params.iter()) {
            let arg_ty = self.check_expr(&arg.value);
            if !self.types_compatible(param_ty, &arg_ty) {
                self.error(TypeError::type_mismatch(
                    param_ty,
                    &arg_ty,
                    self.span(arg.value.span()),
                    &self.source,
                ));
            }
        }
    }

    fn check_constructor(&mut self, c: &ast::ConstructorDef) {
        self.return_type = Some(Type::Unit);

        self.symbols.push_scope(ScopeKind::Function);

        // Add parameters to scope
        for param in &c.params {
            let ty = self.resolve_type_expr(&param.ty);
            self.symbols.define_variable(param.name.name.clone(), ty, false);
        }

        // Check constructor body
        self.check_block(&c.body);

        self.symbols.pop_scope();
        self.return_type = None;
    }

    fn check_modifier_def(&mut self, m: &ast::ModifierDef) {
        self.return_type = Some(Type::Unit);

        self.symbols.push_scope(ScopeKind::Function);

        // Add parameters to scope
        for param in &m.params {
            let ty = self.resolve_type_expr(&param.ty);
            self.symbols.define_variable(param.name.name.clone(), ty, false);
        }

        // Check modifier body
        self.check_block(&m.body);

        self.symbols.pop_scope();
        self.return_type = None;
    }

    // =========================================================================
    // Statement Checking
    // =========================================================================

    fn check_block(&mut self, block: &ast::Block) {
        self.symbols.push_scope(ScopeKind::Block);

        for stmt in &block.stmts {
            self.check_stmt(stmt);
        }

        self.symbols.pop_scope();
    }

    fn check_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::VarDecl(v) => self.check_var_decl_stmt(v),
            ast::Stmt::Return(r) => self.check_return_stmt(r),
            ast::Stmt::If(i) => self.check_if_stmt(i),
            ast::Stmt::While(w) => self.check_while_stmt(w),
            ast::Stmt::For(f) => self.check_for_stmt(f),
            ast::Stmt::Emit(e) => self.check_emit_stmt(e),
            ast::Stmt::Require(r) => self.check_require_stmt(r),
            ast::Stmt::Revert(r) => self.check_revert_stmt(r),
            ast::Stmt::Delete(d) => {
                // Delete is valid for any lvalue expression
                self.check_expr(&d.target);
            }
            ast::Stmt::Selfdestruct(s) => {
                // Selfdestruct recipient must be an address
                let recipient_ty = self.check_expr(&s.recipient);
                if !matches!(recipient_ty, Type::Primitive(PrimitiveType::Address)) {
                    self.error(TypeError::type_mismatch(
                        &Type::Primitive(PrimitiveType::Address),
                        &recipient_ty,
                        self.span(s.span),
                        &self.source,
                    ));
                }
            }
            ast::Stmt::Placeholder(_) => {} // Placeholder _ in modifier
            ast::Stmt::Expr(e) => {
                self.check_expr(&e.expr);
            }
        }
    }

    fn check_var_decl_stmt(&mut self, v: &ast::VarDeclStmt) {
        let declared_ty = self.resolve_type_expr(&v.ty);

        if let Some(init) = &v.initializer {
            let value_ty = self.check_expr(init);

            if !self.types_compatible(&declared_ty, &value_ty) {
                self.error(TypeError::type_mismatch(
                    &declared_ty,
                    &value_ty,
                    self.span(v.span),
                    &self.source,
                ));
            }
        }

        // Add variable to scope
        self.symbols.define_variable(v.name.name.clone(), declared_ty, true);
    }

    fn check_return_stmt(&mut self, r: &ast::ReturnStmt) {
        let value_ty = r
            .value
            .as_ref()
            .map(|v| self.check_expr(v))
            .unwrap_or(Type::Unit);

        if let Some(expected) = &self.return_type {
            if !self.types_compatible(expected, &value_ty) {
                self.error(TypeError::type_mismatch(
                    expected,
                    &value_ty,
                    self.span(r.span),
                    &self.source,
                ));
            }
        }
    }

    fn check_if_stmt(&mut self, i: &ast::IfStmt) {
        let cond_ty = self.check_expr(&i.condition);
        if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
            self.error(TypeError::type_mismatch(
                &Type::Primitive(PrimitiveType::Bool),
                &cond_ty,
                self.span(i.condition.span()),
                &self.source,
            ));
        }

        self.check_block(&i.then_block);

        if let Some(else_branch) = &i.else_branch {
            match else_branch {
                ast::ElseBranch::Else(block) => self.check_block(block),
                ast::ElseBranch::ElseIf(elif) => self.check_if_stmt(elif),
            }
        }
    }

    fn check_while_stmt(&mut self, w: &ast::WhileStmt) {
        let cond_ty = self.check_expr(&w.condition);
        if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
            self.error(TypeError::type_mismatch(
                &Type::Primitive(PrimitiveType::Bool),
                &cond_ty,
                self.span(w.condition.span()),
                &self.source,
            ));
        }

        self.check_block(&w.body);
    }

    fn check_for_stmt(&mut self, f: &ast::ForStmt) {
        self.symbols.push_scope(ScopeKind::Block);

        // Check init
        if let Some(init) = &f.init {
            match init {
                ast::ForInit::VarDecl(v) => self.check_var_decl_stmt(v),
                ast::ForInit::Expr(e) => {
                    self.check_expr(e);
                }
            }
        }

        // Check condition
        if let Some(cond) = &f.condition {
            let cond_ty = self.check_expr(cond);
            if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
                self.error(TypeError::type_mismatch(
                    &Type::Primitive(PrimitiveType::Bool),
                    &cond_ty,
                    self.span(cond.span()),
                    &self.source,
                ));
            }
        }

        // Check update
        if let Some(update) = &f.update {
            self.check_expr(update);
        }

        // Check body
        self.check_block(&f.body);

        self.symbols.pop_scope();
    }

    fn check_emit_stmt(&mut self, e: &ast::EmitStmt) {
        let event_name = &e.event.name;

        // Look up the event
        if let Some(type_def) = self.symbols.lookup_type(event_name) {
            if let TypeDef::Event(event_def) = type_def {
                // Check argument count
                if e.args.len() != event_def.params.len() {
                    self.error(TypeError::wrong_arg_count(
                        event_def.params.len(),
                        e.args.len(),
                        self.span(e.span),
                        &self.source,
                    ));
                    return;
                }

                // Check argument types
                let event_params = event_def.params.clone();
                for (arg, param) in e.args.iter().zip(event_params.iter()) {
                    let arg_ty = self.check_expr(&arg.value);
                    if !self.types_compatible(&param.ty, &arg_ty) {
                        self.error(TypeError::type_mismatch(
                            &param.ty,
                            &arg_ty,
                            self.span(arg.value.span()),
                            &self.source,
                        ));
                    }
                }
            } else {
                self.error(TypeError::UndefinedEvent {
                    name: event_name.to_string(),
                    span: miette::SourceSpan::new(e.event.span.start.into(), (e.event.span.end - e.event.span.start).into()),
                    src: self.source.clone(),
                });
            }
        } else {
            self.error(TypeError::UndefinedEvent {
                name: event_name.to_string(),
                span: miette::SourceSpan::new(e.event.span.start.into(), (e.event.span.end - e.event.span.start).into()),
                src: self.source.clone(),
            });
        }
    }

    fn check_require_stmt(&mut self, r: &ast::RequireStmt) {
        let cond_ty = self.check_expr(&r.condition);
        if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
            self.error(TypeError::type_mismatch(
                &Type::Primitive(PrimitiveType::Bool),
                &cond_ty,
                self.span(r.condition.span()),
                &self.source,
            ));
        }
    }

    fn check_revert_stmt(&mut self, r: &ast::RevertStmt) {
        match &r.kind {
            ast::RevertKind::Message(_) => {
                // Just a string message, nothing to check
            }
            ast::RevertKind::Error { name, args } => {
                // Check that the error exists
                let error_name = &name.name;

                if let Some(type_def) = self.symbols.lookup_type(error_name) {
                    if let TypeDef::Error(error_def) = type_def {
                        // Check argument count
                        if args.len() != error_def.params.len() {
                            self.error(TypeError::wrong_arg_count(
                                error_def.params.len(),
                                args.len(),
                                self.span(r.span),
                                &self.source,
                            ));
                            return;
                        }
                        // Check argument types
                        let error_params = error_def.params.clone();
                        for (arg, param) in args.iter().zip(error_params.iter()) {
                            let arg_ty = self.check_expr(&arg.value);
                            if !self.types_compatible(&param.ty, &arg_ty) {
                                self.error(TypeError::type_mismatch(
                                    &param.ty,
                                    &arg_ty,
                                    self.span(arg.span),
                                    &self.source,
                                ));
                            }
                        }
                    } else {
                        // Not an error type, report error
                        self.error(TypeError::not_callable(
                            &Type::Named(NamedType::new(error_name.clone())),
                            self.span(name.span),
                            &self.source,
                        ));
                    }
                } else {
                    self.error(TypeError::undefined_type(
                        error_name.as_str(),
                        self.span(name.span),
                        &self.source,
                    ));
                }
            }
        }
    }

    // =========================================================================
    // Expression Checking
    // =========================================================================

    fn check_expr(&mut self, expr: &ast::Expr) -> Type {
        match expr {
            ast::Expr::Literal(lit) => self.check_literal(lit),
            ast::Expr::Ident(ident) => self.check_ident_expr(ident),
            ast::Expr::Binary(bin) => self.check_binary_expr(bin),
            ast::Expr::Unary(un) => self.check_unary_expr(un),
            ast::Expr::Call(call) => self.check_call_expr(call),
            ast::Expr::MethodCall(mc) => self.check_method_call(mc),
            ast::Expr::FieldAccess(fa) => self.check_field_access(fa),
            ast::Expr::Index(idx) => self.check_index_expr(idx),
            ast::Expr::If(if_expr) => self.check_if_expr(if_expr),
            ast::Expr::Array(arr) => self.check_array_expr(arr),
            ast::Expr::Tuple(tuple) => self.check_tuple_expr(tuple),
            ast::Expr::Assign(a) => self.check_assign_expr(a),
            ast::Expr::Ternary(t) => self.check_ternary_expr(t),
            ast::Expr::New(n) => self.check_new_expr(n),
            ast::Expr::Paren(e) => self.check_expr(e),
        }
    }

    fn check_literal(&mut self, lit: &ast::Literal) -> Type {
        match lit {
            ast::Literal::Bool(_, _) => Type::Primitive(PrimitiveType::Bool),
            ast::Literal::Int(_, _) => Type::Primitive(PrimitiveType::Uint256), // Default integer type
            ast::Literal::HexInt(_, _) => Type::Primitive(PrimitiveType::Uint256),
            ast::Literal::String(_, _) => Type::Primitive(PrimitiveType::String),
            ast::Literal::HexString(_, _) => Type::Primitive(PrimitiveType::Bytes),
            ast::Literal::Address(_, _) => Type::Primitive(PrimitiveType::Address),
        }
    }

    fn check_ident_expr(&mut self, ident: &ast::Ident) -> Type {
        let name = &ident.name;

        // Handle built-in objects
        match name.as_str() {
            "msg" | "block" | "tx" | "token" | "clock" | "rent" => {
                // These are built-in objects with specific fields/methods
                // For now, return a placeholder type
                return Type::Named(NamedType::new(name.clone()));
            }
            _ => {}
        }

        // Look up variable
        if let Some(var) = self.symbols.lookup_variable(name) {
            return var.ty.clone();
        }

        // Look up function
        if let Some(func) = self.symbols.lookup_function(name) {
            return Type::Function(func.ty.clone());
        }

        self.error(TypeError::undefined_variable(
            name,
            self.span(ident.span),
            &self.source,
        ));
        Type::Error
    }

    fn check_binary_expr(&mut self, bin: &ast::BinaryExpr) -> Type {
        let left_ty = self.check_expr(&bin.left);
        let right_ty = self.check_expr(&bin.right);

        // Skip type checking if either side has an error
        if matches!(left_ty, Type::Error) || matches!(right_ty, Type::Error) {
            return Type::Error;
        }

        match bin.op {
            // Arithmetic operators
            ast::BinaryOp::Add | ast::BinaryOp::Sub | ast::BinaryOp::Mul |
            ast::BinaryOp::Div | ast::BinaryOp::Rem | ast::BinaryOp::Exp => {
                if left_ty.is_integer() && self.types_compatible(&left_ty, &right_ty) {
                    left_ty
                } else {
                    self.error(TypeError::invalid_binary_op(
                        &format!("{:?}", bin.op),
                        &left_ty,
                        &right_ty,
                        self.span(bin.span),
                        &self.source,
                    ));
                    Type::Error
                }
            }
            // Comparison operators
            ast::BinaryOp::Eq | ast::BinaryOp::Ne | ast::BinaryOp::Lt |
            ast::BinaryOp::Le | ast::BinaryOp::Gt | ast::BinaryOp::Ge => {
                if self.types_compatible(&left_ty, &right_ty) {
                    Type::Primitive(PrimitiveType::Bool)
                } else {
                    self.error(TypeError::invalid_binary_op(
                        &format!("{:?}", bin.op),
                        &left_ty,
                        &right_ty,
                        self.span(bin.span),
                        &self.source,
                    ));
                    Type::Error
                }
            }
            // Logical operators
            ast::BinaryOp::And | ast::BinaryOp::Or => {
                if left_ty.is_bool() && right_ty.is_bool() {
                    Type::Primitive(PrimitiveType::Bool)
                } else {
                    self.error(TypeError::invalid_binary_op(
                        &format!("{:?}", bin.op),
                        &left_ty,
                        &right_ty,
                        self.span(bin.span),
                        &self.source,
                    ));
                    Type::Error
                }
            }
            // Bitwise operators
            ast::BinaryOp::BitAnd | ast::BinaryOp::BitOr | ast::BinaryOp::BitXor |
            ast::BinaryOp::Shl | ast::BinaryOp::Shr => {
                if left_ty.is_integer() && right_ty.is_integer() {
                    left_ty
                } else {
                    self.error(TypeError::invalid_binary_op(
                        &format!("{:?}", bin.op),
                        &left_ty,
                        &right_ty,
                        self.span(bin.span),
                        &self.source,
                    ));
                    Type::Error
                }
            }
        }
    }

    fn check_unary_expr(&mut self, un: &ast::UnaryExpr) -> Type {
        let expr_ty = self.check_expr(&un.expr);

        if matches!(expr_ty, Type::Error) {
            return Type::Error;
        }

        match un.op {
            ast::UnaryOp::Neg => {
                if expr_ty.is_integer() {
                    expr_ty
                } else {
                    self.error(TypeError::InvalidUnaryOp {
                        op: "-".to_string(),
                        ty: expr_ty.to_string(),
                        span: miette::SourceSpan::new(un.span.start.into(), (un.span.end - un.span.start).into()),
                        src: self.source.clone(),
                    });
                    Type::Error
                }
            }
            ast::UnaryOp::Not => {
                if expr_ty.is_bool() {
                    Type::Primitive(PrimitiveType::Bool)
                } else {
                    self.error(TypeError::InvalidUnaryOp {
                        op: "!".to_string(),
                        ty: expr_ty.to_string(),
                        span: miette::SourceSpan::new(un.span.start.into(), (un.span.end - un.span.start).into()),
                        src: self.source.clone(),
                    });
                    Type::Error
                }
            }
            ast::UnaryOp::BitNot => {
                if expr_ty.is_integer() {
                    expr_ty
                } else {
                    self.error(TypeError::InvalidUnaryOp {
                        op: "~".to_string(),
                        ty: expr_ty.to_string(),
                        span: miette::SourceSpan::new(un.span.start.into(), (un.span.end - un.span.start).into()),
                        src: self.source.clone(),
                    });
                    Type::Error
                }
            }
            ast::UnaryOp::PreInc | ast::UnaryOp::PreDec |
            ast::UnaryOp::PostInc | ast::UnaryOp::PostDec => {
                if expr_ty.is_integer() {
                    expr_ty
                } else {
                    self.error(TypeError::InvalidUnaryOp {
                        op: "++/--".to_string(),
                        ty: expr_ty.to_string(),
                        span: miette::SourceSpan::new(un.span.start.into(), (un.span.end - un.span.start).into()),
                        src: self.source.clone(),
                    });
                    Type::Error
                }
            }
        }
    }

    fn check_call_expr(&mut self, call: &ast::CallExpr) -> Type {
        // Check for type cast expressions like address(0), uint256(x), etc.
        if let ast::Expr::Ident(ident) = &call.callee {
            let name = ident.name.as_str();

            // Handle built-in test functions
            match name {
                "assert" => {
                    // assert(condition) or assert(condition, "message")
                    if call.args.is_empty() || call.args.len() > 2 {
                        self.error(TypeError::wrong_arg_count(
                            1,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Unit;
                    }
                    let cond_ty = self.check_expr(&call.args[0].value);
                    if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
                        self.error(TypeError::type_mismatch(
                            &Type::Primitive(PrimitiveType::Bool),
                            &cond_ty,
                            self.span(call.args[0].value.span()),
                            &self.source,
                        ));
                    }
                    // Optional message argument
                    if call.args.len() == 2 {
                        let msg_ty = self.check_expr(&call.args[1].value);
                        if !matches!(msg_ty, Type::Primitive(PrimitiveType::String)) && !matches!(msg_ty, Type::Error) {
                            self.error(TypeError::type_mismatch(
                                &Type::Primitive(PrimitiveType::String),
                                &msg_ty,
                                self.span(call.args[1].value.span()),
                                &self.source,
                            ));
                        }
                    }
                    return Type::Unit;
                }
                "assertEq" => {
                    // assertEq(left, right) or assertEq(left, right, "message")
                    if call.args.len() < 2 || call.args.len() > 3 {
                        self.error(TypeError::wrong_arg_count(
                            2,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Unit;
                    }
                    let left_ty = self.check_expr(&call.args[0].value);
                    let right_ty = self.check_expr(&call.args[1].value);
                    if !self.types_compatible(&left_ty, &right_ty) {
                        self.error(TypeError::type_mismatch(
                            &left_ty,
                            &right_ty,
                            self.span(call.args[1].value.span()),
                            &self.source,
                        ));
                    }
                    // Optional message argument
                    if call.args.len() == 3 {
                        let msg_ty = self.check_expr(&call.args[2].value);
                        if !matches!(msg_ty, Type::Primitive(PrimitiveType::String)) && !matches!(msg_ty, Type::Error) {
                            self.error(TypeError::type_mismatch(
                                &Type::Primitive(PrimitiveType::String),
                                &msg_ty,
                                self.span(call.args[2].value.span()),
                                &self.source,
                            ));
                        }
                    }
                    return Type::Unit;
                }
                "assertNe" => {
                    // assertNe(left, right) or assertNe(left, right, "message")
                    if call.args.len() < 2 || call.args.len() > 3 {
                        self.error(TypeError::wrong_arg_count(
                            2,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Unit;
                    }
                    let left_ty = self.check_expr(&call.args[0].value);
                    let right_ty = self.check_expr(&call.args[1].value);
                    if !self.types_compatible(&left_ty, &right_ty) {
                        self.error(TypeError::type_mismatch(
                            &left_ty,
                            &right_ty,
                            self.span(call.args[1].value.span()),
                            &self.source,
                        ));
                    }
                    // Optional message argument
                    if call.args.len() == 3 {
                        let msg_ty = self.check_expr(&call.args[2].value);
                        if !matches!(msg_ty, Type::Primitive(PrimitiveType::String)) && !matches!(msg_ty, Type::Error) {
                            self.error(TypeError::type_mismatch(
                                &Type::Primitive(PrimitiveType::String),
                                &msg_ty,
                                self.span(call.args[2].value.span()),
                                &self.source,
                            ));
                        }
                    }
                    return Type::Unit;
                }
                "assertGt" | "assertGe" | "assertLt" | "assertLe" => {
                    // assertGt(left, right) - assert left > right
                    if call.args.len() < 2 || call.args.len() > 3 {
                        self.error(TypeError::wrong_arg_count(
                            2,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Unit;
                    }
                    let left_ty = self.check_expr(&call.args[0].value);
                    let right_ty = self.check_expr(&call.args[1].value);
                    // Both should be comparable (integers)
                    if !left_ty.is_integer() && !matches!(left_ty, Type::Error) {
                        self.error(TypeError::type_mismatch(
                            &Type::Primitive(PrimitiveType::Uint256),
                            &left_ty,
                            self.span(call.args[0].value.span()),
                            &self.source,
                        ));
                    }
                    if !right_ty.is_integer() && !matches!(right_ty, Type::Error) {
                        self.error(TypeError::type_mismatch(
                            &Type::Primitive(PrimitiveType::Uint256),
                            &right_ty,
                            self.span(call.args[1].value.span()),
                            &self.source,
                        ));
                    }
                    return Type::Unit;
                }
                "transfer" => {
                    // transfer(to, amount) - direct SOL transfer
                    if call.args.len() != 2 {
                        self.error(TypeError::wrong_arg_count(
                            2,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Unit;
                    }
                    let to_ty = self.check_expr(&call.args[0].value);
                    let amount_ty = self.check_expr(&call.args[1].value);
                    // First arg should be an address
                    if !matches!(to_ty, Type::Primitive(PrimitiveType::Address)) && !matches!(to_ty, Type::Error) {
                        self.error(TypeError::type_mismatch(
                            &Type::Primitive(PrimitiveType::Address),
                            &to_ty,
                            self.span(call.args[0].value.span()),
                            &self.source,
                        ));
                    }
                    // Second arg should be an integer (lamports)
                    if !amount_ty.is_integer() && !matches!(amount_ty, Type::Error) {
                        self.error(TypeError::type_mismatch(
                            &Type::Primitive(PrimitiveType::Uint64),
                            &amount_ty,
                            self.span(call.args[1].value.span()),
                            &self.source,
                        ));
                    }
                    return Type::Unit;
                }
                _ => {}
            }

            // Handle address(expr) - type cast to address
            if name == "address" {
                if call.args.len() != 1 {
                    self.error(TypeError::wrong_arg_count(
                        1,
                        call.args.len(),
                        self.span(call.span),
                        &self.source,
                    ));
                    return Type::Error;
                }
                // Type check the argument (but don't require specific type for casts)
                self.check_expr(&call.args[0].value);
                return Type::Primitive(PrimitiveType::Address);
            }

            // Handle uint256(expr), uint64(expr), etc. - type cast to integer
            // Handle bytes1(expr), bytes32(expr), etc. - type cast to fixed bytes
            if let Some(prim) = PrimitiveType::from_str(name) {
                if prim.is_integer() || prim.is_fixed_bytes() {
                    if call.args.len() != 1 {
                        self.error(TypeError::wrong_arg_count(
                            1,
                            call.args.len(),
                            self.span(call.span),
                            &self.source,
                        ));
                        return Type::Error;
                    }
                    self.check_expr(&call.args[0].value);
                    return Type::Primitive(prim);
                }
            }

            // Handle interface type cast: InterfaceName(address) -> InterfaceName
            // This enables CPI: IERC20(tokenAddress).transfer(to, amount)
            if let Some(TypeDef::Interface(_)) = self.symbols.lookup_type(&SmolStr::from(name)) {
                if call.args.len() != 1 {
                    self.error(TypeError::wrong_arg_count(
                        1,
                        call.args.len(),
                        self.span(call.span),
                        &self.source,
                    ));
                    return Type::Error;
                }
                // The argument should be an address (program ID)
                let arg_ty = self.check_expr(&call.args[0].value);
                if !matches!(arg_ty, Type::Primitive(PrimitiveType::Address)) && !matches!(arg_ty, Type::Error) {
                    self.error(TypeError::type_mismatch(
                        &Type::Primitive(PrimitiveType::Address),
                        &arg_ty,
                        self.span(call.args[0].span),
                        &self.source,
                    ));
                }
                // Return the interface type (as Named type)
                return Type::Named(NamedType {
                    name: SmolStr::from(name),
                    type_args: Vec::new(),
                });
            }
        }

        let callee_ty = self.check_expr(&call.callee);

        if let Type::Function(fn_ty) = callee_ty {
            // Check argument count
            if call.args.len() != fn_ty.params.len() {
                self.error(TypeError::wrong_arg_count(
                    fn_ty.params.len(),
                    call.args.len(),
                    self.span(call.span),
                    &self.source,
                ));
            }

            // Check argument types
            for (arg, expected_ty) in call.args.iter().zip(fn_ty.params.iter()) {
                let arg_ty = self.check_expr(&arg.value);
                if !self.types_compatible(expected_ty, &arg_ty) {
                    self.error(TypeError::type_mismatch(
                        expected_ty,
                        &arg_ty,
                        self.span(arg.span),
                        &self.source,
                    ));
                }
            }

            *fn_ty.return_type
        } else if matches!(callee_ty, Type::Error) {
            Type::Error
        } else {
            self.error(TypeError::not_callable(&callee_ty, self.span(call.span), &self.source));
            Type::Error
        }
    }

    fn check_method_call(&mut self, mc: &ast::MethodCallExpr) -> Type {
        let receiver_ty = self.check_expr(&mc.receiver);
        let method_name = mc.method.name.clone();

        // Check arguments
        let arg_types: Vec<Type> = mc.args.iter().map(|arg| self.check_expr(&arg.value)).collect();

        // Handle built-in object methods
        if let Type::Named(named) = &receiver_ty {
            let type_name = named.name.as_str();

            // Handle msg, block, tx methods/fields
            match type_name {
                "msg" => {
                    match method_name.as_str() {
                        "sender" => return Type::Primitive(PrimitiveType::Address),
                        "value" => return Type::Primitive(PrimitiveType::Uint256),
                        "data" => return Type::Primitive(PrimitiveType::Bytes),
                        _ => {}
                    }
                }
                "block" => {
                    match method_name.as_str() {
                        "timestamp" => return Type::Primitive(PrimitiveType::Uint256),
                        "number" => return Type::Primitive(PrimitiveType::Uint256),
                        _ => {}
                    }
                }
                "tx" => {
                    match method_name.as_str() {
                        "origin" => return Type::Primitive(PrimitiveType::Address),
                        "gasprice" => return Type::Primitive(PrimitiveType::Uint256),
                        _ => {}
                    }
                }
                "token" => {
                    // SPL Token operations: transfer(from, to, authority, amount)
                    // mint(mint, to, authority, amount), burn(from, mint, authority, amount)
                    match method_name.as_str() {
                        "transfer" | "mint" | "burn" => {
                            // All take 4 args: 3 addresses and 1 amount
                            if arg_types.len() != 4 {
                                self.error(TypeError::wrong_arg_count(
                                    4,
                                    arg_types.len(),
                                    self.span(mc.span),
                                    &self.source,
                                ));
                                return Type::Error;
                            }
                            return Type::Unit;
                        }
                        "getATA" => {
                            // getATA(owner, mint) -> address
                            if arg_types.len() != 2 {
                                self.error(TypeError::wrong_arg_count(
                                    2,
                                    arg_types.len(),
                                    self.span(mc.span),
                                    &self.source,
                                ));
                                return Type::Error;
                            }
                            return Type::Primitive(PrimitiveType::Address);
                        }
                        _ => {}
                    }
                }
                // Solana Rent sysvar: rent.minimumBalance(size), rent.isExempt(balance, size)
                "rent" => {
                    match method_name.as_str() {
                        "minimumBalance" => {
                            // minimumBalance(dataLen: uint64) -> uint64
                            if arg_types.len() != 1 {
                                self.error(TypeError::wrong_arg_count(
                                    1,
                                    arg_types.len(),
                                    self.span(mc.span),
                                    &self.source,
                                ));
                                return Type::Error;
                            }
                            return Type::Primitive(PrimitiveType::Uint64);
                        }
                        "isExempt" => {
                            // isExempt(lamports: uint64, dataLen: uint64) -> bool
                            if arg_types.len() != 2 {
                                self.error(TypeError::wrong_arg_count(
                                    2,
                                    arg_types.len(),
                                    self.span(mc.span),
                                    &self.source,
                                ));
                                return Type::Error;
                            }
                            return Type::Primitive(PrimitiveType::Bool);
                        }
                        _ => {}
                    }
                }
                // Solana Clock sysvar methods
                "clock" => {
                    match method_name.as_str() {
                        "get" => {
                            // clock.get() returns a Clock-like type (for now just return the type itself)
                            return Type::Named(NamedType::new(SmolStr::from("clock")));
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            // Look up the method on the named type
            let method_info = self.symbols.lookup_type(&SmolStr::from(type_name)).and_then(|type_def| {
                match type_def {
                    TypeDef::Contract(c) => c.methods.get(&method_name).cloned(),
                    TypeDef::Interface(i) => i.methods.get(&method_name).cloned(),
                    _ => None,
                }
            });

            if let Some(fn_ty) = method_info {
                // Check argument count
                if arg_types.len() != fn_ty.params.len() {
                    self.error(TypeError::wrong_arg_count(
                        fn_ty.params.len(),
                        arg_types.len(),
                        self.span(mc.span),
                        &self.source,
                    ));
                    return Type::Error;
                }

                // Check argument types
                for (i, (arg_ty, param_ty)) in arg_types.iter().zip(fn_ty.params.iter()).enumerate() {
                    if !self.types_compatible(param_ty, arg_ty) {
                        self.error(TypeError::type_mismatch(
                            param_ty,
                            arg_ty,
                            self.span(mc.args[i].value.span()),
                            &self.source,
                        ));
                    }
                }

                return (*fn_ty.return_type).clone();
            }

            // Method not found
            self.error(TypeError::undefined_method(
                &method_name,
                &receiver_ty,
                self.span(mc.span),
                &self.source,
            ));
            return Type::Error;
        }

        if matches!(receiver_ty, Type::Error) {
            return Type::Error;
        }

        // Handle dynamic array methods (push, pop, etc.)
        if let Type::DynamicArray(elem_ty) = &receiver_ty {
            match method_name.as_str() {
                "push" => {
                    // push takes one argument of the element type
                    if arg_types.len() != 1 {
                        self.error(TypeError::wrong_arg_count(
                            1,
                            arg_types.len(),
                            self.span(mc.span),
                            &self.source,
                        ));
                        return Type::Error;
                    }
                    // Type check the argument against the element type
                    if !self.types_compatible(elem_ty, &arg_types[0]) {
                        self.error(TypeError::type_mismatch(
                            elem_ty,
                            &arg_types[0],
                            self.span(mc.span),
                            &self.source,
                        ));
                    }
                    return Type::Unit;
                }
                "pop" => {
                    if !arg_types.is_empty() {
                        self.error(TypeError::wrong_arg_count(
                            0,
                            arg_types.len(),
                            self.span(mc.span),
                            &self.source,
                        ));
                        return Type::Error;
                    }
                    return (**elem_ty).clone();
                }
                _ => {}
            }
        }

        // Cannot call methods on non-named types
        self.error(TypeError::undefined_method(
            &method_name,
            &receiver_ty,
            self.span(mc.span),
            &self.source,
        ));
        Type::Error
    }

    fn check_field_access(&mut self, fa: &ast::FieldAccessExpr) -> Type {
        let expr_ty = self.check_expr(&fa.expr);

        if matches!(expr_ty, Type::Error) {
            return Type::Error;
        }

        // Handle built-in object fields (msg.sender, block.timestamp, etc.)
        if let Type::Named(named) = &expr_ty {
            let type_name = named.name.as_str();
            let field_name = fa.field.name.as_str();

            match type_name {
                "msg" => {
                    match field_name {
                        "sender" => return Type::Primitive(PrimitiveType::Address),
                        "value" => return Type::Primitive(PrimitiveType::Uint256),
                        "data" => return Type::Primitive(PrimitiveType::Bytes),
                        _ => {}
                    }
                }
                "block" => {
                    match field_name {
                        "timestamp" => return Type::Primitive(PrimitiveType::Uint256),
                        "number" => return Type::Primitive(PrimitiveType::Uint256),
                        _ => {}
                    }
                }
                "tx" => {
                    match field_name {
                        "origin" => return Type::Primitive(PrimitiveType::Address),
                        "gasprice" => return Type::Primitive(PrimitiveType::Uint256),
                        _ => {}
                    }
                }
                // Solana-specific: clock.timestamp, clock.slot, clock.epoch
                "clock" => {
                    match field_name {
                        "timestamp" => return Type::Primitive(PrimitiveType::Int64),
                        "slot" => return Type::Primitive(PrimitiveType::Uint64),
                        "epoch" => return Type::Primitive(PrimitiveType::Uint64),
                        "unix_timestamp" => return Type::Primitive(PrimitiveType::Int64),
                        _ => {}
                    }
                }
                _ => {
                    // Look up field on user-defined type
                    let field_ty = self.symbols.lookup_type(&SmolStr::from(type_name)).and_then(|type_def| {
                        match type_def {
                            TypeDef::Struct(s) => s.fields.get(field_name).cloned(),
                            TypeDef::Contract(c) => c.state_fields.get(field_name).cloned(),
                            _ => None,
                        }
                    });

                    if let Some(ty) = field_ty {
                        return ty;
                    }
                }
            }

            // Report error for unknown field
            self.error(TypeError::undefined_field(
                &fa.field.name,
                &expr_ty,
                self.span(fa.span),
                &self.source,
            ));
            return Type::Error;
        }

        // Handle array/vec length property
        let field_name = fa.field.name.as_str();
        if field_name == "length" {
            match &expr_ty {
                Type::Array(_, _) | Type::DynamicArray(_) => {
                    return Type::Primitive(PrimitiveType::Uint256);
                }
                _ => {}
            }
        }

        // For other types, report error
        self.error(TypeError::undefined_field(
            &fa.field.name,
            &expr_ty,
            self.span(fa.span),
            &self.source,
        ));
        Type::Error
    }

    fn check_index_expr(&mut self, idx: &ast::IndexExpr) -> Type {
        let expr_ty = self.check_expr(&idx.expr);
        let index_ty = self.check_expr(&idx.index);

        // Get element type
        match expr_ty {
            Type::Array(elem, _) | Type::DynamicArray(elem) => {
                // Check index is numeric
                if !index_ty.is_integer() && !matches!(index_ty, Type::Error) {
                    self.error(TypeError::type_mismatch(
                        &Type::Primitive(PrimitiveType::Uint256),
                        &index_ty,
                        self.span(idx.index.span()),
                        &self.source,
                    ));
                }
                *elem
            }
            Type::Mapping(key, value) => {
                // Check key type
                if !self.types_compatible(&key, &index_ty) && !matches!(index_ty, Type::Error) {
                    self.error(TypeError::type_mismatch(
                        &key,
                        &index_ty,
                        self.span(idx.index.span()),
                        &self.source,
                    ));
                }
                *value
            }
            Type::Error => Type::Error,
            _ => {
                self.error(TypeError::NotIndexable {
                    ty: expr_ty.to_string(),
                    span: miette::SourceSpan::new(idx.span.start.into(), (idx.span.end - idx.span.start).into()),
                    src: self.source.clone(),
                });
                Type::Error
            }
        }
    }

    fn check_if_expr(&mut self, if_expr: &ast::IfExpr) -> Type {
        let cond_ty = self.check_expr(&if_expr.condition);
        if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
            self.error(TypeError::type_mismatch(
                &Type::Primitive(PrimitiveType::Bool),
                &cond_ty,
                self.span(if_expr.condition.span()),
                &self.source,
            ));
        }

        self.check_block(&if_expr.then_block);

        match &*if_expr.else_branch {
            ast::IfExprElse::Else(block) => self.check_block(block),
            ast::IfExprElse::ElseIf(elif) => {
                self.check_if_expr(elif);
            }
        }

        // For simplicity, if expressions return Unit
        Type::Unit
    }

    fn check_array_expr(&mut self, arr: &ast::ArrayExpr) -> Type {
        if arr.elements.is_empty() {
            return Type::DynamicArray(Box::new(self.fresh_type_var()));
        }

        let first_ty = self.check_expr(&arr.elements[0]);

        for elem in arr.elements.iter().skip(1) {
            let elem_ty = self.check_expr(elem);
            if !self.types_compatible(&first_ty, &elem_ty) {
                self.error(TypeError::type_mismatch(
                    &first_ty,
                    &elem_ty,
                    self.span(elem.span()),
                    &self.source,
                ));
            }
        }

        Type::Array(Box::new(first_ty), arr.elements.len() as u64)
    }

    fn check_tuple_expr(&mut self, tuple: &ast::TupleExpr) -> Type {
        let elem_types: Vec<Type> = tuple.elements.iter().map(|e| self.check_expr(e)).collect();
        Type::Tuple(elem_types)
    }

    fn check_assign_expr(&mut self, a: &ast::AssignExpr) -> Type {
        let target_ty = self.check_expr(&a.target);
        let value_ty = self.check_expr(&a.value);

        match a.op {
            ast::AssignOp::Assign => {
                // Regular assignment: value must be compatible with target
                if !self.types_compatible(&target_ty, &value_ty) {
                    self.error(TypeError::type_mismatch(
                        &target_ty,
                        &value_ty,
                        self.span(a.span),
                        &self.source,
                    ));
                }
            }
            ast::AssignOp::AddAssign | ast::AssignOp::SubAssign |
            ast::AssignOp::MulAssign | ast::AssignOp::DivAssign |
            ast::AssignOp::RemAssign => {
                // Compound assignment: both must be numeric and compatible
                if !target_ty.is_integer() || !self.types_compatible(&target_ty, &value_ty) {
                    if !matches!(target_ty, Type::Error) && !matches!(value_ty, Type::Error) {
                        self.error(TypeError::invalid_binary_op(
                            &format!("{:?}", a.op),
                            &target_ty,
                            &value_ty,
                            self.span(a.span),
                            &self.source,
                        ));
                    }
                }
            }
            ast::AssignOp::BitAndAssign | ast::AssignOp::BitOrAssign |
            ast::AssignOp::BitXorAssign => {
                // Bitwise compound assignment: both must be integer
                if !target_ty.is_integer() || !value_ty.is_integer() {
                    if !matches!(target_ty, Type::Error) && !matches!(value_ty, Type::Error) {
                        self.error(TypeError::invalid_binary_op(
                            &format!("{:?}", a.op),
                            &target_ty,
                            &value_ty,
                            self.span(a.span),
                            &self.source,
                        ));
                    }
                }
            }
        }

        Type::Unit
    }

    fn check_ternary_expr(&mut self, t: &ast::TernaryExpr) -> Type {
        let cond_ty = self.check_expr(&t.condition);
        if !cond_ty.is_bool() && !matches!(cond_ty, Type::Error) {
            self.error(TypeError::type_mismatch(
                &Type::Primitive(PrimitiveType::Bool),
                &cond_ty,
                self.span(t.condition.span()),
                &self.source,
            ));
        }

        let then_ty = self.check_expr(&t.then_expr);
        let else_ty = self.check_expr(&t.else_expr);

        if !self.types_compatible(&then_ty, &else_ty) {
            self.error(TypeError::type_mismatch(
                &then_ty,
                &else_ty,
                self.span(t.span),
                &self.source,
            ));
            return Type::Error;
        }

        then_ty
    }

    fn check_new_expr(&mut self, n: &ast::NewExpr) -> Type {
        let type_name = n.ty.name();

        // Check if type exists
        if self.symbols.lookup_type(&type_name).is_none() {
            self.error(TypeError::undefined_type(&type_name, self.span(n.span), &self.source));
            return Type::Error;
        }

        // Check constructor arguments
        // Note: Full constructor validation would require storing constructor signatures
        // in the symbol table. For now, we just type-check the argument expressions.
        for arg in &n.args {
            self.check_expr(&arg.value);
        }

        Type::Named(NamedType::new(type_name.clone()))
    }

    // =========================================================================
    // Type Compatibility
    // =========================================================================

    fn types_compatible(&self, expected: &Type, found: &Type) -> bool {
        match (expected, found) {
            (Type::Error, _) | (_, Type::Error) => true,
            (Type::Var(_), _) | (_, Type::Var(_)) => true, // Type variables are compatible with anything
            // Allow integer literals to be compatible with any integer type
            (Type::Primitive(a), Type::Primitive(b)) if a.is_integer() && b.is_integer() => true,
            // Signer is compatible with Address (signers are addresses that have signed)
            (Type::Primitive(PrimitiveType::Address), Type::Primitive(PrimitiveType::Signer)) => true,
            (Type::Primitive(PrimitiveType::Signer), Type::Primitive(PrimitiveType::Address)) => true,
            (Type::Primitive(a), Type::Primitive(b)) => a == b,
            (Type::Unit, Type::Unit) => true,
            (Type::Never, _) => true, // Never is compatible with anything
            (Type::Named(a), Type::Named(b)) => {
                a.name == b.name
                    && a.type_args.len() == b.type_args.len()
                    && a.type_args
                        .iter()
                        .zip(b.type_args.iter())
                        .all(|(x, y)| self.types_compatible(x, y))
            }
            (Type::Array(a, n1), Type::Array(b, n2)) => n1 == n2 && self.types_compatible(a, b),
            (Type::DynamicArray(a), Type::DynamicArray(b)) => self.types_compatible(a, b),
            (Type::Tuple(a), Type::Tuple(b)) => {
                a.len() == b.len()
                    && a.iter()
                        .zip(b.iter())
                        .all(|(x, y)| self.types_compatible(x, y))
            }
            (Type::Mapping(k1, v1), Type::Mapping(k2, v2)) => {
                self.types_compatible(k1, k2) && self.types_compatible(v1, v2)
            }
            (Type::Function(a), Type::Function(b)) => {
                a.params.len() == b.params.len()
                    && a.params
                        .iter()
                        .zip(b.params.iter())
                        .all(|(x, y)| self.types_compatible(x, y))
                    && self.types_compatible(&a.return_type, &b.return_type)
            }
            _ => false,
        }
    }
}
