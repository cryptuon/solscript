//! Solana Intermediate Representation
//!
//! This module defines an IR that's closer to Solana's execution model,
//! making it easier to generate Anchor Rust code.

use crate::error::CodegenError;
use solscript_ast::{self as ast, StateMutability, Visibility};

/// A Solana program (corresponds to a SolScript contract)
#[derive(Debug, Clone)]
pub struct SolanaProgram {
    pub name: String,
    pub state: ProgramState,
    pub mappings: Vec<MappingDef>,
    pub modifiers: Vec<ModifierDefinition>,
    pub instructions: Vec<Instruction>,
    pub events: Vec<Event>,
    pub errors: Vec<ProgramError>,
    pub structs: Vec<StructDef>,
    pub enums: Vec<EnumDef>,
    /// Test functions marked with #[test]
    pub tests: Vec<TestFunction>,
}

/// A test function
#[derive(Debug, Clone)]
pub struct TestFunction {
    pub name: String,
    pub body: Vec<Statement>,
    /// Expected error message for #[should_fail("...")] tests
    pub should_fail: Option<String>,
}

/// An enum definition
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: String,
    pub variants: Vec<String>,
}

/// A struct definition
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

/// A field in a struct
#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: SolanaType,
}

/// A modifier definition (gets inlined into functions)
#[derive(Debug, Clone)]
pub struct ModifierDefinition {
    pub name: String,
    pub params: Vec<InstructionParam>,
    pub body: Vec<Statement>,
}

/// A mapping definition (becomes PDA-based storage)
#[derive(Debug, Clone)]
pub struct MappingDef {
    pub name: String,
    pub key_ty: SolanaType,
    pub value_ty: SolanaType,
    pub is_public: bool,
}

/// Program state account
#[derive(Debug, Clone)]
pub struct ProgramState {
    pub fields: Vec<StateField>,
}

/// A field in the state account
#[derive(Debug, Clone)]
pub struct StateField {
    pub name: String,
    pub ty: SolanaType,
    pub is_public: bool,
}

/// An instruction (function) in the program
#[derive(Debug, Clone)]
pub struct Instruction {
    pub name: String,
    pub params: Vec<InstructionParam>,
    pub returns: Option<SolanaType>,
    pub body: Vec<Statement>,
    pub is_public: bool,
    pub is_view: bool,
    pub is_payable: bool,
    pub uses_token_program: bool,
    pub uses_sol_transfer: bool,
    pub modifiers: Vec<ModifierCall>,
    /// Mapping accesses needed for this instruction
    pub mapping_accesses: Vec<MappingAccess>,
    /// If true, this instruction closes the state account (selfdestruct)
    pub closes_state: bool,
}

/// A mapping access within an instruction
#[derive(Debug, Clone)]
pub struct MappingAccess {
    /// Name of the mapping being accessed
    pub mapping_name: String,
    /// The key expression(s) used to access the mapping (multiple for nested mappings)
    pub key_exprs: Vec<Expression>,
    /// Whether this is a write access (needs init_if_needed)
    pub is_write: bool,
    /// Whether this access should close the PDA (delete operation)
    pub should_close: bool,
    /// Generated account name for this access
    pub account_name: String,
}

/// A parameter for an instruction
#[derive(Debug, Clone)]
pub struct InstructionParam {
    pub name: String,
    pub ty: SolanaType,
}

/// A modifier invocation
#[derive(Debug, Clone)]
pub struct ModifierCall {
    pub name: String,
    pub args: Vec<Expression>,
}

/// An event
#[derive(Debug, Clone)]
pub struct Event {
    pub name: String,
    pub fields: Vec<EventField>,
}

/// An event field
#[derive(Debug, Clone)]
pub struct EventField {
    pub name: String,
    pub ty: SolanaType,
    pub indexed: bool,
}

/// A custom error
#[derive(Debug, Clone)]
pub struct ProgramError {
    pub name: String,
    pub fields: Vec<ErrorField>,
}

/// An error field
#[derive(Debug, Clone)]
pub struct ErrorField {
    pub name: String,
    pub ty: SolanaType,
}

/// Types in Solana IR
#[derive(Debug, Clone)]
pub enum SolanaType {
    U8,
    U16,
    U32,
    U64,
    U128,
    I8,
    I16,
    I32,
    I64,
    I128,
    Bool,
    Pubkey, // Solana's address type
    Signer, // A required signer account
    String,
    Bytes,
    FixedBytes(usize), // bytes1 through bytes32 -> [u8; N]
    Array(Box<SolanaType>, usize),
    Vec(Box<SolanaType>),
    Option(Box<SolanaType>),
    // For mappings, we use PDA-based storage which requires special handling
    Mapping(Box<SolanaType>, Box<SolanaType>),
    // User-defined types
    Custom(String),
}

/// Statements in IR
#[derive(Debug, Clone)]
pub enum Statement {
    VarDecl {
        name: String,
        ty: SolanaType,
        value: Option<Expression>,
    },
    Assign {
        target: Expression,
        value: Expression,
    },
    If {
        condition: Expression,
        then_block: Vec<Statement>,
        else_block: Option<Vec<Statement>>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    For {
        init: Option<Box<Statement>>,
        condition: Option<Expression>,
        update: Option<Expression>,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
    Emit {
        event: String,
        args: Vec<Expression>,
    },
    Require {
        condition: Expression,
        message: Option<String>,
    },
    /// Revert with a custom error: revert ErrorName(args)
    RevertWithError {
        error_name: String,
        args: Vec<Expression>,
    },
    /// Delete statement: reset target to default value
    Delete(Expression),
    /// Selfdestruct: close the state account and send rent to recipient
    Selfdestruct {
        recipient: Expression,
    },
    Expr(Expression),
    /// Placeholder for modifier body insertion (`_` in Solidity)
    Placeholder,
}

/// Expressions in IR
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Var(String),
    StateAccess(String), // Access to state field
    /// Mapping access: `mapping_name[key1][key2]...` → `ctx.accounts.{account_name}.value`
    MappingAccess {
        mapping_name: String,
        /// All keys in order (outer to inner)
        keys: Vec<Expression>,
        /// Generated account name for this access point
        account_name: String,
    },
    MsgSender,      // msg.sender → ctx.accounts.signer
    MsgValue,       // msg.value (not directly supported in Solana)
    BlockTimestamp, // block.timestamp → Clock::get()
    // Solana Clock sysvar fields
    ClockSlot,          // clock.slot → Clock::get()?.slot
    ClockEpoch,         // clock.epoch → Clock::get()?.epoch
    ClockUnixTimestamp, // clock.unix_timestamp → Clock::get()?.unix_timestamp
    // Solana Rent sysvar methods
    RentMinimumBalance {
        data_len: Box<Expression>,
    },
    RentIsExempt {
        lamports: Box<Expression>,
        data_len: Box<Expression>,
    },
    Binary {
        op: BinaryOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    Call {
        func: String,
        args: Vec<Expression>,
    },
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        args: Vec<Expression>,
    },
    /// Interface cast for CPI: IERC20(programId) -> allows calling methods on external programs
    InterfaceCast {
        /// The interface name (e.g., "IERC20")
        interface_name: String,
        /// The program ID to call
        program_id: Box<Expression>,
    },
    /// Cross-Program Invocation call
    CpiCall {
        /// The program to call (expression evaluating to program ID)
        program: Box<Expression>,
        /// Interface/program name for generating the right instruction
        interface_name: String,
        /// Method name to call
        method: String,
        /// Arguments to the CPI call
        args: Vec<Expression>,
    },
    /// SPL Token transfer CPI
    TokenTransfer {
        /// from account
        from: Box<Expression>,
        /// to account
        to: Box<Expression>,
        /// authority
        authority: Box<Expression>,
        /// amount
        amount: Box<Expression>,
    },
    /// SPL Token mint CPI
    TokenMint {
        /// mint account
        mint: Box<Expression>,
        /// to account
        to: Box<Expression>,
        /// authority
        authority: Box<Expression>,
        /// amount
        amount: Box<Expression>,
    },
    /// SPL Token burn CPI
    TokenBurn {
        /// from account
        from: Box<Expression>,
        /// mint account
        mint: Box<Expression>,
        /// authority
        authority: Box<Expression>,
        /// amount
        amount: Box<Expression>,
    },
    /// Direct SOL transfer via system_program::transfer
    SolTransfer {
        /// to account (Pubkey)
        to: Box<Expression>,
        /// amount in lamports
        amount: Box<Expression>,
    },
    /// Get Associated Token Address
    GetATA {
        /// owner/wallet address
        owner: Box<Expression>,
        /// token mint address
        mint: Box<Expression>,
    },
    Index {
        expr: Box<Expression>,
        index: Box<Expression>,
    },
    Field {
        expr: Box<Expression>,
        field: String,
    },
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    /// Assert expression: assert(condition, "message")
    Assert {
        condition: Box<Expression>,
        message: Option<String>,
    },
    /// Assert equality: assertEq(left, right, "message")
    AssertEq {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
    /// Assert not equal: assertNe(left, right, "message")
    AssertNe {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
    /// Assert greater than: assertGt(left, right, "message")
    AssertGt {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
    /// Assert greater or equal: assertGe(left, right, "message")
    AssertGe {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
    /// Assert less than: assertLt(left, right, "message")
    AssertLt {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
    /// Assert less or equal: assertLe(left, right, "message")
    AssertLe {
        left: Box<Expression>,
        right: Box<Expression>,
        message: Option<String>,
    },
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Int(i128),
    Uint(u128),
    String(String),
    Pubkey(String),   // Base58 encoded
    ZeroAddress,      // address(0) - the default/null address
    ZeroBytes(usize), // bytes32(0) etc. - zero-filled fixed bytes
}

/// Binary operators
#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

/// Unary operators
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
    BitNot,
}

/// Lower the AST to Solana IR
pub fn lower_to_ir(program: &ast::Program) -> Result<Vec<SolanaProgram>, CodegenError> {
    let mut programs = Vec::new();
    let mut events = Vec::new();
    let mut errors = Vec::new();
    let mut structs = Vec::new();
    let mut enums = Vec::new();

    // First pass: collect events, errors, structs, enums, and interfaces
    let mut interface_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    for item in &program.items {
        match item {
            ast::Item::Event(e) => {
                events.push(lower_event(e)?);
            }
            ast::Item::Error(e) => {
                errors.push(lower_error(e)?);
            }
            ast::Item::Struct(s) => {
                structs.push(lower_struct(s)?);
            }
            ast::Item::Enum(e) => {
                enums.push(lower_enum(e));
            }
            ast::Item::Interface(i) => {
                interface_names.insert(i.name.name.to_string());
            }
            ast::Item::Contract(c) => {
                // Also collect events, errors, structs, and enums defined inside contracts
                for member in &c.members {
                    match member {
                        ast::ContractMember::Event(e) => {
                            events.push(lower_event(e)?);
                        }
                        ast::ContractMember::Error(e) => {
                            errors.push(lower_error(e)?);
                        }
                        ast::ContractMember::Struct(s) => {
                            structs.push(lower_struct(s)?);
                        }
                        ast::ContractMember::Enum(e) => {
                            enums.push(lower_enum(e));
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    // Collect all contracts for inheritance resolution
    let contracts: std::collections::HashMap<String, &ast::ContractDef> = program
        .items
        .iter()
        .filter_map(|item| {
            if let ast::Item::Contract(c) = item {
                Some((c.name.name.to_string(), c))
            } else {
                None
            }
        })
        .collect();

    // Second pass: process contracts (skip abstract contracts)
    for item in &program.items {
        if let ast::Item::Contract(contract) = item {
            // Skip abstract contracts - they can't be deployed
            if contract.is_abstract {
                continue;
            }
            let prog = lower_contract(
                contract,
                &events,
                &errors,
                &structs,
                &enums,
                &contracts,
                &interface_names,
            )?;
            programs.push(prog);
        }
    }

    Ok(programs)
}

/// Context for lowering expressions, tracking state fields and mappings
struct LoweringContext {
    state_fields: std::collections::HashSet<String>,
    mapping_names: std::collections::HashSet<String>,
    mappings: Vec<MappingDef>,
    interface_names: std::collections::HashSet<String>,
}

impl LoweringContext {
    fn new() -> Self {
        Self {
            state_fields: std::collections::HashSet::new(),
            mapping_names: std::collections::HashSet::new(),
            mappings: Vec::new(),
            interface_names: std::collections::HashSet::new(),
        }
    }

    fn is_state_field(&self, name: &str) -> bool {
        self.state_fields.contains(name)
    }

    fn is_interface(&self, name: &str) -> bool {
        self.interface_names.contains(name)
    }

    fn is_mapping(&self, name: &str) -> bool {
        self.mapping_names.contains(name)
    }
}

/// Collector for mapping accesses within a function
struct MappingAccessCollector {
    accesses: Vec<MappingAccess>,
    counter: usize,
    uses_token_program: bool,
    uses_sol_transfer: bool,
}

impl MappingAccessCollector {
    fn new() -> Self {
        Self {
            accesses: Vec::new(),
            counter: 0,
            uses_token_program: false,
            uses_sol_transfer: false,
        }
    }

    fn mark_uses_token_program(&mut self) {
        self.uses_token_program = true;
    }

    fn mark_uses_sol_transfer(&mut self) {
        self.uses_sol_transfer = true;
    }

    /// Record a mapping access and return a unique account name
    fn record_access(
        &mut self,
        mapping_name: &str,
        keys: Vec<Expression>,
        is_write: bool,
        should_close: bool,
    ) -> String {
        // Generate unique account name based on mapping name and counter
        let account_name = format!("{}_entry_{}", to_snake_case(mapping_name), self.counter);
        self.counter += 1;

        self.accesses.push(MappingAccess {
            mapping_name: mapping_name.to_string(),
            key_exprs: keys,
            is_write,
            should_close,
            account_name: account_name.clone(),
        });

        account_name
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut prev_upper = false;

    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_upper = true;
        } else {
            result.push(c);
            prev_upper = false;
        }
    }

    result
}

fn lower_contract(
    contract: &ast::ContractDef,
    events: &[Event],
    errors: &[ProgramError],
    structs: &[StructDef],
    enums: &[EnumDef],
    all_contracts: &std::collections::HashMap<String, &ast::ContractDef>,
    interface_names: &std::collections::HashSet<String>,
) -> Result<SolanaProgram, CodegenError> {
    let name = contract.name.name.to_string();

    // Collect all members including inherited ones
    // Order: base contracts first (in declaration order), then this contract
    let mut all_members: Vec<&ast::ContractMember> = Vec::new();

    // Process base contracts (inheritance)
    for base in &contract.bases {
        let base_name = base.segments.first().map(|s| s.name.as_str()).unwrap_or("");
        if let Some(base_contract) = all_contracts.get(base_name) {
            // Add base contract members (excluding constructors - those are handled separately)
            for member in &base_contract.members {
                if !matches!(member, ast::ContractMember::Constructor(_)) {
                    all_members.push(member);
                }
            }
        }
    }

    // Add this contract's members (may override inherited ones)
    for member in &contract.members {
        all_members.push(member);
    }

    // First pass: collect state fields and mappings from all members
    let mut fields = Vec::new();
    let mut ctx = LoweringContext::new();
    ctx.interface_names = interface_names.clone();
    let mut seen_fields = std::collections::HashSet::new();

    for member in &all_members {
        if let ast::ContractMember::StateVar(var) = member {
            let field_name = var.name.name.to_string();

            // Skip if already defined (child overrides parent)
            if seen_fields.contains(&field_name) {
                continue;
            }
            seen_fields.insert(field_name.clone());

            let field_ty = lower_type(&var.ty)?;
            let is_public = matches!(var.visibility, Some(Visibility::Public));

            // Check if this is a mapping type
            if let SolanaType::Mapping(key_ty, value_ty) = field_ty {
                ctx.mapping_names.insert(field_name.clone());
                ctx.mappings.push(MappingDef {
                    name: field_name,
                    key_ty: (*key_ty).clone(),
                    value_ty: (*value_ty).clone(),
                    is_public,
                });
            } else {
                ctx.state_fields.insert(field_name.clone());
                fields.push(StateField {
                    name: field_name,
                    ty: field_ty,
                    is_public,
                });
            }
        }
    }

    // Second pass: collect modifiers and lower functions
    let mut modifiers = Vec::new();
    let mut instructions = Vec::new();
    let mut seen_modifiers = std::collections::HashSet::new();
    let mut seen_functions = std::collections::HashSet::new();

    // First, collect all modifiers (child overrides parent)
    for member in all_members.iter().rev() {
        if let ast::ContractMember::Modifier(modifier) = member {
            let mod_name = modifier.name.name.to_string();
            if !seen_modifiers.contains(&mod_name) {
                seen_modifiers.insert(mod_name);
                modifiers.push(lower_modifier(modifier, &ctx)?);
            }
        }
    }

    // Then lower functions (child overrides parent)
    for member in all_members.iter().rev() {
        match member {
            ast::ContractMember::StateVar(_) => {
                // Already handled
            }
            ast::ContractMember::Function(func) => {
                let func_name = func.name.name.to_string();
                // Skip abstract functions (those without a body)
                if func.body.is_some() && !seen_functions.contains(&func_name) {
                    seen_functions.insert(func_name);
                    instructions.push(lower_function(func, &ctx)?);
                }
            }
            ast::ContractMember::Constructor(_) => {
                // Handle constructors separately - only use the child's constructor
            }
            ast::ContractMember::Modifier(_) => {
                // Already handled above
            }
            ast::ContractMember::Event(_) | ast::ContractMember::Error(_) => {
                // Events and errors are handled at the top level during lowering
            }
            ast::ContractMember::Struct(_) | ast::ContractMember::Enum(_) => {
                // Structs and enums are handled at the top level during lowering
            }
        }
    }

    // Handle constructor from this contract only
    for member in &contract.members {
        if let ast::ContractMember::Constructor(ctor) = member {
            instructions.insert(0, lower_constructor(ctor, &ctx)?);
            break;
        }
    }

    // Extract test functions (functions with #[test] attribute)
    let mut tests = Vec::new();
    for member in all_members.iter() {
        if let ast::ContractMember::Function(func) = member {
            if has_test_attribute(&func.attributes) {
                tests.push(lower_test_function(func, &ctx)?);
            }
        }
    }

    Ok(SolanaProgram {
        name,
        state: ProgramState { fields },
        mappings: ctx.mappings,
        modifiers,
        instructions,
        events: events.to_vec(),
        errors: errors.to_vec(),
        structs: structs.to_vec(),
        enums: enums.to_vec(),
        tests,
    })
}

/// Check if a function has the #[test] attribute
fn has_test_attribute(attrs: &[ast::Attribute]) -> bool {
    attrs.iter().any(|a| a.name.name.as_str() == "test")
}

/// Get the expected failure message from #[should_fail("...")] attribute
fn get_should_fail_message(attrs: &[ast::Attribute]) -> Option<String> {
    for attr in attrs {
        if attr.name.name.as_str() == "should_fail" {
            if let Some(arg) = attr.args.first() {
                if let ast::AttributeValue::Literal(ast::Literal::String(s, _)) = &arg.value {
                    return Some(s.to_string());
                }
            }
            // #[should_fail] without message
            return Some(String::new());
        }
    }
    None
}

/// Lower a test function
fn lower_test_function(
    func: &ast::FnDef,
    ctx: &LoweringContext,
) -> Result<TestFunction, CodegenError> {
    let name = func.name.name.to_string();
    let mut collector = MappingAccessCollector::new();

    let body = if let Some(block) = &func.body {
        lower_block(block, ctx, &mut collector)?
    } else {
        Vec::new()
    };

    let should_fail = get_should_fail_message(&func.attributes);

    Ok(TestFunction {
        name,
        body,
        should_fail,
    })
}

fn lower_function(func: &ast::FnDef, ctx: &LoweringContext) -> Result<Instruction, CodegenError> {
    let name = func.name.name.to_string();
    let mut collector = MappingAccessCollector::new();

    let params: Vec<InstructionParam> = func
        .params
        .iter()
        .map(|p| {
            Ok(InstructionParam {
                name: p.name.name.to_string(),
                ty: lower_type(&p.ty)?,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    let returns = if func.return_params.is_empty() {
        None
    } else if func.return_params.len() == 1 {
        Some(lower_type(&func.return_params[0].ty)?)
    } else {
        return Err(CodegenError::UnsupportedFeature(
            "Multiple return values".to_string(),
        ));
    };

    let is_public = matches!(
        func.visibility,
        Some(Visibility::Public) | Some(Visibility::External)
    );
    let is_view = func
        .state_mutability
        .iter()
        .any(|m| matches!(m, StateMutability::View | StateMutability::Pure));
    let is_payable = func
        .state_mutability
        .iter()
        .any(|m| matches!(m, StateMutability::Payable));

    let mut modifiers = Vec::new();
    for m in &func.modifiers {
        let args: Vec<Expression> = m
            .args
            .iter()
            .map(|a| lower_expr(&a.value, ctx, &mut collector))
            .collect::<Result<Vec<_>, _>>()?;
        modifiers.push(ModifierCall {
            name: m.name.name.to_string(),
            args,
        });
    }

    // Safe to unwrap since we only call lower_function for functions with bodies
    let body = lower_block(func.body.as_ref().unwrap(), ctx, &mut collector)?;

    // Check if body contains selfdestruct
    let closes_state = body_contains_selfdestruct(&body);

    Ok(Instruction {
        name,
        params,
        returns,
        body,
        is_public,
        is_view,
        is_payable,
        uses_token_program: collector.uses_token_program,
        uses_sol_transfer: collector.uses_sol_transfer,
        modifiers,
        mapping_accesses: collector.accesses,
        closes_state,
    })
}

/// Check if a statement list contains a Selfdestruct statement
fn body_contains_selfdestruct(stmts: &[Statement]) -> bool {
    for stmt in stmts {
        match stmt {
            Statement::Selfdestruct { .. } => return true,
            Statement::If {
                then_block,
                else_block,
                ..
            } => {
                if body_contains_selfdestruct(then_block) {
                    return true;
                }
                if let Some(else_stmts) = else_block {
                    if body_contains_selfdestruct(else_stmts) {
                        return true;
                    }
                }
            }
            Statement::While { body, .. } => {
                if body_contains_selfdestruct(body) {
                    return true;
                }
            }
            Statement::For { body, .. } => {
                if body_contains_selfdestruct(body) {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn lower_constructor(
    ctor: &ast::ConstructorDef,
    ctx: &LoweringContext,
) -> Result<Instruction, CodegenError> {
    let mut collector = MappingAccessCollector::new();

    let params: Vec<InstructionParam> = ctor
        .params
        .iter()
        .map(|p| {
            Ok(InstructionParam {
                name: p.name.name.to_string(),
                ty: lower_type(&p.ty)?,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    let body = lower_block(&ctor.body, ctx, &mut collector)?;

    Ok(Instruction {
        name: "initialize".to_string(),
        params,
        returns: None,
        body,
        is_public: true,
        is_view: false,
        is_payable: ctor.modifiers.iter().any(|m| m.name.name == "payable"),
        uses_token_program: collector.uses_token_program,
        uses_sol_transfer: collector.uses_sol_transfer,
        modifiers: Vec::new(),
        mapping_accesses: collector.accesses,
        closes_state: false, // Constructor never closes state
    })
}

fn lower_modifier(
    modifier: &ast::ModifierDef,
    ctx: &LoweringContext,
) -> Result<ModifierDefinition, CodegenError> {
    let mut collector = MappingAccessCollector::new();

    let params: Vec<InstructionParam> = modifier
        .params
        .iter()
        .map(|p| {
            Ok(InstructionParam {
                name: p.name.name.to_string(),
                ty: lower_type(&p.ty)?,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    let body = lower_block(&modifier.body, ctx, &mut collector)?;

    Ok(ModifierDefinition {
        name: modifier.name.name.to_string(),
        params,
        body,
    })
}

fn lower_event(event: &ast::EventDef) -> Result<Event, CodegenError> {
    let fields: Vec<EventField> = event
        .params
        .iter()
        .map(|p| {
            Ok(EventField {
                name: p.name.name.to_string(),
                ty: lower_type(&p.ty)?,
                indexed: p.indexed,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    Ok(Event {
        name: event.name.name.to_string(),
        fields,
    })
}

fn lower_error(error: &ast::ErrorDef) -> Result<ProgramError, CodegenError> {
    let fields: Vec<ErrorField> = error
        .params
        .iter()
        .map(|p| {
            Ok(ErrorField {
                name: p.name.name.to_string(),
                ty: lower_type(&p.ty)?,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    Ok(ProgramError {
        name: error.name.name.to_string(),
        fields,
    })
}

fn lower_struct(s: &ast::StructDef) -> Result<StructDef, CodegenError> {
    let fields: Vec<StructField> = s
        .fields
        .iter()
        .map(|f| {
            Ok(StructField {
                name: f.name.name.to_string(),
                ty: lower_type(&f.ty)?,
            })
        })
        .collect::<Result<Vec<_>, CodegenError>>()?;

    Ok(StructDef {
        name: s.name.name.to_string(),
        fields,
    })
}

fn lower_enum(e: &ast::EnumDef) -> EnumDef {
    EnumDef {
        name: e.name.name.to_string(),
        variants: e.variants.iter().map(|v| v.name.name.to_string()).collect(),
    }
}

fn lower_type(ty: &ast::TypeExpr) -> Result<SolanaType, CodegenError> {
    match ty {
        ast::TypeExpr::Path(path) => {
            let name = path.name();
            match name.as_str() {
                "uint8" | "u8" => Ok(SolanaType::U8),
                "uint16" | "u16" => Ok(SolanaType::U16),
                "uint32" | "u32" => Ok(SolanaType::U32),
                "uint64" | "u64" => Ok(SolanaType::U64),
                "uint128" | "u128" => Ok(SolanaType::U128),
                "uint256" | "uint" => Ok(SolanaType::U128), // Solana doesn't have u256 natively
                "int8" | "i8" => Ok(SolanaType::I8),
                "int16" | "i16" => Ok(SolanaType::I16),
                "int32" | "i32" => Ok(SolanaType::I32),
                "int64" | "i64" => Ok(SolanaType::I64),
                "int128" | "i128" => Ok(SolanaType::I128),
                "int256" | "int" => Ok(SolanaType::I128),
                "bool" => Ok(SolanaType::Bool),
                "address" => Ok(SolanaType::Pubkey),
                "signer" => Ok(SolanaType::Signer),
                "string" => Ok(SolanaType::String),
                "bytes" => Ok(SolanaType::Bytes),
                // Fixed-size bytes: bytes1 through bytes32
                s if s.starts_with("bytes") => {
                    if let Ok(n) = s[5..].parse::<usize>() {
                        if (1..=32).contains(&n) {
                            Ok(SolanaType::FixedBytes(n))
                        } else {
                            Err(CodegenError::UnsupportedFeature(format!(
                                "Invalid bytes size: {}",
                                n
                            )))
                        }
                    } else {
                        Ok(SolanaType::Custom(s.to_string()))
                    }
                }
                other => Ok(SolanaType::Custom(other.to_string())),
            }
        }
        ast::TypeExpr::Array(arr) => {
            let elem = lower_type(&ast::TypeExpr::Path(arr.element.clone()))?;
            if arr.sizes.len() != 1 {
                return Err(CodegenError::UnsupportedFeature(
                    "Multi-dimensional arrays".to_string(),
                ));
            }
            match &arr.sizes[0] {
                Some(size) => Ok(SolanaType::Array(Box::new(elem), *size as usize)),
                None => Ok(SolanaType::Vec(Box::new(elem))),
            }
        }
        ast::TypeExpr::Mapping(mapping) => {
            let key = lower_type(&mapping.key)?;
            let value = lower_type(&mapping.value)?;
            Ok(SolanaType::Mapping(Box::new(key), Box::new(value)))
        }
        ast::TypeExpr::Tuple(_) => Err(CodegenError::UnsupportedFeature("Tuple types".to_string())),
    }
}

fn lower_block(
    block: &ast::Block,
    ctx: &LoweringContext,
    collector: &mut MappingAccessCollector,
) -> Result<Vec<Statement>, CodegenError> {
    block
        .stmts
        .iter()
        .map(|s| lower_stmt(s, ctx, collector))
        .collect()
}

fn lower_stmt(
    stmt: &ast::Stmt,
    ctx: &LoweringContext,
    collector: &mut MappingAccessCollector,
) -> Result<Statement, CodegenError> {
    match stmt {
        ast::Stmt::VarDecl(v) => Ok(Statement::VarDecl {
            name: v.name.name.to_string(),
            ty: lower_type(&v.ty)?,
            value: v
                .initializer
                .as_ref()
                .map(|e| lower_expr(e, ctx, collector))
                .transpose()?,
        }),
        ast::Stmt::Return(r) => Ok(Statement::Return(
            r.value
                .as_ref()
                .map(|e| lower_expr(e, ctx, collector))
                .transpose()?,
        )),
        ast::Stmt::If(i) => lower_if_stmt(i, ctx, collector),
        ast::Stmt::While(w) => Ok(Statement::While {
            condition: lower_expr(&w.condition, ctx, collector)?,
            body: lower_block(&w.body, ctx, collector)?,
        }),
        ast::Stmt::For(f) => lower_for_stmt(f, ctx, collector),
        ast::Stmt::Emit(e) => Ok(Statement::Emit {
            event: e.event.name.to_string(),
            args: e
                .args
                .iter()
                .map(|a| lower_expr(&a.value, ctx, collector))
                .collect::<Result<Vec<_>, _>>()?,
        }),
        ast::Stmt::Require(r) => Ok(Statement::Require {
            condition: lower_expr(&r.condition, ctx, collector)?,
            message: r.message.as_ref().map(|s| s.to_string()),
        }),
        ast::Stmt::Revert(r) => match &r.kind {
            ast::RevertKind::Message(msg) => Ok(Statement::Require {
                condition: Expression::Literal(Literal::Bool(false)),
                message: msg
                    .as_ref()
                    .map(|s| s.to_string())
                    .or_else(|| Some("Reverted".to_string())),
            }),
            ast::RevertKind::Error { name, args } => {
                let lowered_args: Vec<Expression> = args
                    .iter()
                    .map(|a| lower_expr(&a.value, ctx, collector))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Statement::RevertWithError {
                    error_name: name.name.to_string(),
                    args: lowered_args,
                })
            }
        },
        ast::Stmt::Delete(d) => {
            let target = lower_expr(&d.target, ctx, collector)?;
            // If we're deleting a mapping access, mark it as should_close
            if let Expression::MappingAccess { account_name, .. } = &target {
                // Find and update the mapping access to mark it for closing
                for access in &mut collector.accesses {
                    if &access.account_name == account_name {
                        access.should_close = true;
                        break;
                    }
                }
            }
            Ok(Statement::Delete(target))
        }
        ast::Stmt::Selfdestruct(s) => Ok(Statement::Selfdestruct {
            recipient: lower_expr(&s.recipient, ctx, collector)?,
        }),
        ast::Stmt::Expr(e) => Ok(Statement::Expr(lower_expr(&e.expr, ctx, collector)?)),
        ast::Stmt::Placeholder(_) => Ok(Statement::Placeholder),
    }
}

fn lower_if_stmt(
    i: &ast::IfStmt,
    ctx: &LoweringContext,
    collector: &mut MappingAccessCollector,
) -> Result<Statement, CodegenError> {
    let condition = lower_expr(&i.condition, ctx, collector)?;
    let then_block = lower_block(&i.then_block, ctx, collector)?;
    let else_block = match &i.else_branch {
        Some(ast::ElseBranch::Else(block)) => Some(lower_block(block, ctx, collector)?),
        Some(ast::ElseBranch::ElseIf(elif)) => Some(vec![lower_if_stmt(elif, ctx, collector)?]),
        None => None,
    };

    Ok(Statement::If {
        condition,
        then_block,
        else_block,
    })
}

fn lower_for_stmt(
    f: &ast::ForStmt,
    ctx: &LoweringContext,
    collector: &mut MappingAccessCollector,
) -> Result<Statement, CodegenError> {
    let init = match &f.init {
        Some(ast::ForInit::VarDecl(v)) => Some(Box::new(Statement::VarDecl {
            name: v.name.name.to_string(),
            ty: lower_type(&v.ty)?,
            value: v
                .initializer
                .as_ref()
                .map(|e| lower_expr(e, ctx, collector))
                .transpose()?,
        })),
        Some(ast::ForInit::Expr(e)) => {
            Some(Box::new(Statement::Expr(lower_expr(e, ctx, collector)?)))
        }
        None => None,
    };

    Ok(Statement::For {
        init,
        condition: f
            .condition
            .as_ref()
            .map(|e| lower_expr(e, ctx, collector))
            .transpose()?,
        update: f
            .update
            .as_ref()
            .map(|e| lower_expr(e, ctx, collector))
            .transpose()?,
        body: lower_block(&f.body, ctx, collector)?,
    })
}

/// Extract mapping access including nested mappings
/// Returns (mapping_name, keys) if this is a mapping access, None otherwise
fn extract_mapping_access<'a>(
    base: &'a ast::Expr,
    index: &'a ast::Expr,
    ctx: &LoweringContext,
) -> Result<Option<(String, Vec<&'a ast::Expr>)>, CodegenError> {
    // Check if base is a simple mapping identifier
    if let ast::Expr::Ident(ident) = base {
        let name = ident.name.to_string();
        if ctx.is_mapping(&name) {
            return Ok(Some((name, vec![index])));
        }
    }

    // Check if base is another index expression (nested mapping)
    if let ast::Expr::Index(inner) = base {
        if let Some((mapping_name, mut keys)) =
            extract_mapping_access(&inner.expr, &inner.index, ctx)?
        {
            // Add the outer key
            keys.push(index);
            return Ok(Some((mapping_name, keys)));
        }
    }

    Ok(None)
}

fn lower_expr(
    expr: &ast::Expr,
    ctx: &LoweringContext,
    collector: &mut MappingAccessCollector,
) -> Result<Expression, CodegenError> {
    match expr {
        ast::Expr::Literal(lit) => lower_literal(lit),
        ast::Expr::Ident(ident) => {
            let name = ident.name.to_string();
            match name.as_str() {
                "msg" => Ok(Expression::Var("msg".to_string())),
                "block" => Ok(Expression::Var("block".to_string())),
                "tx" => Ok(Expression::Var("tx".to_string())),
                _ => {
                    // Check if this is a state field
                    if ctx.is_state_field(&name) {
                        Ok(Expression::StateAccess(name))
                    } else {
                        Ok(Expression::Var(name))
                    }
                }
            }
        }
        ast::Expr::Binary(b) => Ok(Expression::Binary {
            op: lower_binary_op(&b.op),
            left: Box::new(lower_expr(&b.left, ctx, collector)?),
            right: Box::new(lower_expr(&b.right, ctx, collector)?),
        }),
        ast::Expr::Unary(u) => Ok(Expression::Unary {
            op: lower_unary_op(&u.op),
            expr: Box::new(lower_expr(&u.expr, ctx, collector)?),
        }),
        ast::Expr::Call(c) => {
            if let ast::Expr::Ident(ident) = &c.callee {
                let func_name = ident.name.to_string();

                // Handle assert functions
                match func_name.as_str() {
                    "assert" => {
                        let condition = lower_expr(&c.args[0].value, ctx, collector)?;
                        let message = if c.args.len() > 1 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[1].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::Assert {
                            condition: Box::new(condition),
                            message,
                        });
                    }
                    "assertEq" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertEq {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    "assertNe" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertNe {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    "assertGt" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertGt {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    "assertGe" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertGe {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    "assertLt" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertLt {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    "assertLe" => {
                        let left = lower_expr(&c.args[0].value, ctx, collector)?;
                        let right = lower_expr(&c.args[1].value, ctx, collector)?;
                        let message = if c.args.len() > 2 {
                            if let ast::Expr::Literal(ast::Literal::String(s, _)) = &c.args[2].value
                            {
                                Some(s.to_string())
                            } else {
                                None
                            }
                        } else {
                            None
                        };
                        return Ok(Expression::AssertLe {
                            left: Box::new(left),
                            right: Box::new(right),
                            message,
                        });
                    }
                    _ => {}
                }

                // Handle address(0) - the zero/null address pattern
                if func_name == "address" && c.args.len() == 1 {
                    if let ast::Expr::Literal(ast::Literal::Int(0, _)) = &c.args[0].value {
                        return Ok(Expression::Literal(Literal::ZeroAddress));
                    }
                }

                // Handle bytes32(0), bytes4(0), etc. - zero-filled fixed bytes
                if func_name.starts_with("bytes") && c.args.len() == 1 {
                    if let Ok(n) = func_name[5..].parse::<usize>() {
                        if (1..=32).contains(&n) {
                            if let ast::Expr::Literal(ast::Literal::Int(0, _)) = &c.args[0].value {
                                return Ok(Expression::Literal(Literal::ZeroBytes(n)));
                            }
                        }
                    }
                }

                // Handle interface type cast: IERC20(address) -> InterfaceCast for CPI
                if ctx.is_interface(&func_name) && c.args.len() == 1 {
                    let program_id = lower_expr(&c.args[0].value, ctx, collector)?;
                    return Ok(Expression::InterfaceCast {
                        interface_name: func_name,
                        program_id: Box::new(program_id),
                    });
                }

                // Handle transfer(to, amount) - direct SOL transfer
                if func_name == "transfer" && c.args.len() == 2 {
                    collector.mark_uses_sol_transfer();
                    let to = lower_expr(&c.args[0].value, ctx, collector)?;
                    let amount = lower_expr(&c.args[1].value, ctx, collector)?;
                    return Ok(Expression::SolTransfer {
                        to: Box::new(to),
                        amount: Box::new(amount),
                    });
                }

                Ok(Expression::Call {
                    func: func_name,
                    args: c
                        .args
                        .iter()
                        .map(|a| lower_expr(&a.value, ctx, collector))
                        .collect::<Result<Vec<_>, _>>()?,
                })
            } else {
                Err(CodegenError::UnsupportedFeature(
                    "Complex call expressions".to_string(),
                ))
            }
        }
        ast::Expr::MethodCall(m) => {
            let receiver = lower_expr(&m.receiver, ctx, collector)?;
            let method = m.method.name.to_string();
            let args: Vec<Expression> = m
                .args
                .iter()
                .map(|a| lower_expr(&a.value, ctx, collector))
                .collect::<Result<Vec<_>, _>>()?;

            // Handle CPI calls: IERC20(programId).transfer(...) -> CpiCall
            if let Expression::InterfaceCast {
                interface_name,
                program_id,
            } = receiver
            {
                return Ok(Expression::CpiCall {
                    program: program_id,
                    interface_name,
                    method,
                    args,
                });
            }

            // Handle built-in objects
            if let Expression::Var(ref name) = receiver {
                match (name.as_str(), method.as_str()) {
                    ("msg", "sender") => return Ok(Expression::MsgSender),
                    ("msg", "value") => return Ok(Expression::MsgValue),
                    ("block", "timestamp") => return Ok(Expression::BlockTimestamp),
                    // Solana Rent sysvar methods
                    ("rent", "minimumBalance") if args.len() == 1 => {
                        return Ok(Expression::RentMinimumBalance {
                            data_len: Box::new(args[0].clone()),
                        });
                    }
                    ("rent", "isExempt") if args.len() == 2 => {
                        return Ok(Expression::RentIsExempt {
                            lamports: Box::new(args[0].clone()),
                            data_len: Box::new(args[1].clone()),
                        });
                    }
                    // SPL Token operations: token.transfer(from, to, authority, amount)
                    ("token", "transfer") if args.len() == 4 => {
                        collector.mark_uses_token_program();
                        return Ok(Expression::TokenTransfer {
                            from: Box::new(args[0].clone()),
                            to: Box::new(args[1].clone()),
                            authority: Box::new(args[2].clone()),
                            amount: Box::new(args[3].clone()),
                        });
                    }
                    // SPL Token mint: token.mint(mint, to, authority, amount)
                    ("token", "mint") if args.len() == 4 => {
                        collector.mark_uses_token_program();
                        return Ok(Expression::TokenMint {
                            mint: Box::new(args[0].clone()),
                            to: Box::new(args[1].clone()),
                            authority: Box::new(args[2].clone()),
                            amount: Box::new(args[3].clone()),
                        });
                    }
                    // SPL Token burn: token.burn(from, mint, authority, amount)
                    ("token", "burn") if args.len() == 4 => {
                        collector.mark_uses_token_program();
                        return Ok(Expression::TokenBurn {
                            from: Box::new(args[0].clone()),
                            mint: Box::new(args[1].clone()),
                            authority: Box::new(args[2].clone()),
                            amount: Box::new(args[3].clone()),
                        });
                    }
                    // Get Associated Token Address: token.getATA(owner, mint)
                    ("token", "getATA") if args.len() == 2 => {
                        collector.mark_uses_token_program();
                        return Ok(Expression::GetATA {
                            owner: Box::new(args[0].clone()),
                            mint: Box::new(args[1].clone()),
                        });
                    }
                    _ => {}
                }
            }

            Ok(Expression::MethodCall {
                receiver: Box::new(receiver),
                method,
                args,
            })
        }
        ast::Expr::FieldAccess(f) => {
            let lowered_expr = lower_expr(&f.expr, ctx, collector)?;
            let field = f.field.name.to_string();

            // Handle built-in objects
            if let Expression::Var(ref name) = lowered_expr {
                match (name.as_str(), field.as_str()) {
                    ("msg", "sender") => return Ok(Expression::MsgSender),
                    ("msg", "value") => return Ok(Expression::MsgValue),
                    ("block", "timestamp") => return Ok(Expression::BlockTimestamp),
                    ("block", "number") => return Ok(Expression::BlockTimestamp), // Solana uses slots
                    // Solana Clock sysvar fields
                    ("clock", "timestamp") => return Ok(Expression::ClockUnixTimestamp),
                    ("clock", "unix_timestamp") => return Ok(Expression::ClockUnixTimestamp),
                    ("clock", "slot") => return Ok(Expression::ClockSlot),
                    ("clock", "epoch") => return Ok(Expression::ClockEpoch),
                    _ => {}
                }
            }

            Ok(Expression::Field {
                expr: Box::new(lowered_expr),
                field,
            })
        }
        ast::Expr::Index(i) => {
            // Try to extract mapping access (including nested mappings)
            if let Some((mapping_name, keys)) = extract_mapping_access(&i.expr, &i.index, ctx)? {
                let lowered_keys: Vec<Expression> = keys
                    .into_iter()
                    .map(|k| lower_expr(k, ctx, collector))
                    .collect::<Result<Vec<_>, _>>()?;

                // Record the mapping access (not closing)
                let account_name =
                    collector.record_access(&mapping_name, lowered_keys.clone(), true, false);
                return Ok(Expression::MappingAccess {
                    mapping_name,
                    keys: lowered_keys,
                    account_name,
                });
            }

            // Regular index access
            Ok(Expression::Index {
                expr: Box::new(lower_expr(&i.expr, ctx, collector)?),
                index: Box::new(lower_expr(&i.index, ctx, collector)?),
            })
        }
        ast::Expr::Ternary(t) => Ok(Expression::Ternary {
            condition: Box::new(lower_expr(&t.condition, ctx, collector)?),
            then_expr: Box::new(lower_expr(&t.then_expr, ctx, collector)?),
            else_expr: Box::new(lower_expr(&t.else_expr, ctx, collector)?),
        }),
        ast::Expr::Assign(a) => {
            let target = lower_expr(&a.target, ctx, collector)?;
            let value = lower_expr(&a.value, ctx, collector)?;

            // Handle compound assignment
            let final_value = match a.op {
                ast::AssignOp::Assign => value,
                ast::AssignOp::AddAssign => Expression::Binary {
                    op: BinaryOp::Add,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::SubAssign => Expression::Binary {
                    op: BinaryOp::Sub,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::MulAssign => Expression::Binary {
                    op: BinaryOp::Mul,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::DivAssign => Expression::Binary {
                    op: BinaryOp::Div,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::RemAssign => Expression::Binary {
                    op: BinaryOp::Rem,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::BitAndAssign => Expression::Binary {
                    op: BinaryOp::BitAnd,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::BitOrAssign => Expression::Binary {
                    op: BinaryOp::BitOr,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
                ast::AssignOp::BitXorAssign => Expression::Binary {
                    op: BinaryOp::BitXor,
                    left: Box::new(target.clone()),
                    right: Box::new(value),
                },
            };

            // Convert assignment to Statement::Assign which will be properly handled
            // For now, return as a special marker that rust_gen will recognize
            Ok(Expression::MethodCall {
                receiver: Box::new(target),
                method: "__assign__".to_string(),
                args: vec![final_value],
            })
        }
        ast::Expr::Array(a) => {
            // Array literals become Vec construction
            if a.elements.is_empty() {
                Ok(Expression::Call {
                    func: "Vec::new".to_string(),
                    args: vec![],
                })
            } else {
                Ok(Expression::Call {
                    func: "vec!".to_string(),
                    args: a
                        .elements
                        .iter()
                        .map(|e| lower_expr(e, ctx, collector))
                        .collect::<Result<Vec<_>, _>>()?,
                })
            }
        }
        ast::Expr::Paren(e) => lower_expr(e, ctx, collector),
        ast::Expr::If(_) => Err(CodegenError::UnsupportedFeature(
            "If expressions".to_string(),
        )),
        ast::Expr::Tuple(_) => Err(CodegenError::UnsupportedFeature(
            "Tuple expressions".to_string(),
        )),
        ast::Expr::New(_) => Err(CodegenError::UnsupportedFeature(
            "New expressions (use CPI instead)".to_string(),
        )),
    }
}

fn lower_literal(lit: &ast::Literal) -> Result<Expression, CodegenError> {
    match lit {
        ast::Literal::Bool(b, _) => Ok(Expression::Literal(Literal::Bool(*b))),
        ast::Literal::Int(n, _) => Ok(Expression::Literal(Literal::Uint(*n))),
        ast::Literal::HexInt(s, _) => {
            let n = u128::from_str_radix(s.trim_start_matches("0x"), 16)
                .map_err(|_| CodegenError::TypeConversion(format!("Invalid hex: {}", s)))?;
            Ok(Expression::Literal(Literal::Uint(n)))
        }
        ast::Literal::String(s, _) => Ok(Expression::Literal(Literal::String(s.to_string()))),
        ast::Literal::HexString(s, _) => Ok(Expression::Literal(Literal::String(s.to_string()))),
        ast::Literal::Address(s, _) => Ok(Expression::Literal(Literal::Pubkey(s.to_string()))),
    }
}

fn lower_binary_op(op: &ast::BinaryOp) -> BinaryOp {
    match op {
        ast::BinaryOp::Add => BinaryOp::Add,
        ast::BinaryOp::Sub => BinaryOp::Sub,
        ast::BinaryOp::Mul => BinaryOp::Mul,
        ast::BinaryOp::Div => BinaryOp::Div,
        ast::BinaryOp::Rem => BinaryOp::Rem,
        ast::BinaryOp::Exp => BinaryOp::Mul, // No native exp, would need custom impl
        ast::BinaryOp::Eq => BinaryOp::Eq,
        ast::BinaryOp::Ne => BinaryOp::Ne,
        ast::BinaryOp::Lt => BinaryOp::Lt,
        ast::BinaryOp::Le => BinaryOp::Le,
        ast::BinaryOp::Gt => BinaryOp::Gt,
        ast::BinaryOp::Ge => BinaryOp::Ge,
        ast::BinaryOp::And => BinaryOp::And,
        ast::BinaryOp::Or => BinaryOp::Or,
        ast::BinaryOp::BitAnd => BinaryOp::BitAnd,
        ast::BinaryOp::BitOr => BinaryOp::BitOr,
        ast::BinaryOp::BitXor => BinaryOp::BitXor,
        ast::BinaryOp::Shl => BinaryOp::Shl,
        ast::BinaryOp::Shr => BinaryOp::Shr,
    }
}

fn lower_unary_op(op: &ast::UnaryOp) -> UnaryOp {
    match op {
        ast::UnaryOp::Neg => UnaryOp::Neg,
        ast::UnaryOp::Not => UnaryOp::Not,
        ast::UnaryOp::BitNot => UnaryOp::BitNot,
        ast::UnaryOp::PreInc | ast::UnaryOp::PostInc => UnaryOp::Neg, // Placeholder
        ast::UnaryOp::PreDec | ast::UnaryOp::PostDec => UnaryOp::Neg, // Placeholder
    }
}
