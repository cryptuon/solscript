//! SolScript Abstract Syntax Tree (Solidity-Style)
//!
//! This crate defines all AST node types for the SolScript language.

mod span;
mod types;

pub use span::*;
pub use types::*;

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

/// A complete SolScript program (compilation unit)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub items: Vec<Item>,
    pub span: Span,
}

/// Top-level items
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Item {
    Import(ImportStmt),
    Contract(ContractDef),
    Interface(InterfaceDef),
    Struct(StructDef),
    Enum(EnumDef),
    Event(EventDef),
    Error(ErrorDef),
    Function(FnDef),
}

// =============================================================================
// Imports
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportStmt {
    pub items: Vec<ImportItem>,
    pub source: SmolStr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImportItem {
    pub name: Ident,
    pub alias: Option<Ident>,
    pub span: Span,
}

// =============================================================================
// Visibility & Modifiers
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Internal,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StateMutability {
    View,
    Pure,
    Payable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageLocation {
    Memory,
    Storage,
    Calldata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifierInvocation {
    pub name: Ident,
    pub args: Vec<Arg>,
    pub span: Span,
}

// =============================================================================
// Contract Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractDef {
    pub attributes: Vec<Attribute>,
    pub is_abstract: bool,
    pub name: Ident,
    pub bases: Vec<TypePath>,
    pub members: Vec<ContractMember>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContractMember {
    StateVar(StateVar),
    Constructor(ConstructorDef),
    Function(FnDef),
    Modifier(ModifierDef),
    Event(EventDef),
    Error(ErrorDef),
    Struct(StructDef),
    Enum(EnumDef),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StateVar {
    pub attributes: Vec<Attribute>,
    pub ty: TypeExpr,
    pub visibility: Option<Visibility>,
    pub name: Ident,
    pub initializer: Option<Expr>,
    pub span: Span,
}

// =============================================================================
// Interface Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InterfaceDef {
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub bases: Vec<TypePath>,
    pub members: Vec<FnSig>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnSig {
    pub name: Ident,
    pub generic_params: Option<GenericParams>,
    pub params: Vec<Param>,
    pub visibility: Option<Visibility>,
    pub state_mutability: Vec<StateMutability>,
    pub modifiers: Vec<ModifierInvocation>,
    pub return_params: Vec<ReturnParam>,
    pub span: Span,
}

// =============================================================================
// Struct Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructDef {
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub generic_params: Option<GenericParams>,
    pub fields: Vec<StructField>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructField {
    pub ty: TypeExpr,
    pub name: Ident,
    pub span: Span,
}

// =============================================================================
// Enum Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumDef {
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: Ident,
    pub span: Span,
}

// =============================================================================
// Event & Error Definitions
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventDef {
    pub name: Ident,
    pub params: Vec<EventParam>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EventParam {
    pub ty: TypeExpr,
    pub indexed: bool,
    pub name: Ident,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorDef {
    pub name: Ident,
    pub params: Vec<ErrorParam>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorParam {
    pub ty: TypeExpr,
    pub name: Ident,
    pub span: Span,
}

// =============================================================================
// Constructor Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConstructorDef {
    pub params: Vec<Param>,
    pub modifiers: Vec<ModifierInvocation>,
    pub body: Block,
    pub span: Span,
}

// =============================================================================
// Modifier Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModifierDef {
    pub name: Ident,
    pub params: Vec<Param>,
    pub body: Block,
    pub span: Span,
}

// =============================================================================
// Function Definition
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnDef {
    pub attributes: Vec<Attribute>,
    pub name: Ident,
    pub generic_params: Option<GenericParams>,
    pub params: Vec<Param>,
    pub visibility: Option<Visibility>,
    pub state_mutability: Vec<StateMutability>,
    pub modifiers: Vec<ModifierInvocation>,
    pub return_params: Vec<ReturnParam>,
    pub body: Option<Block>, // None for abstract functions
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Param {
    pub ty: TypeExpr,
    pub storage_location: Option<StorageLocation>,
    pub name: Ident,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnParam {
    pub ty: TypeExpr,
    pub name: Option<Ident>,
    pub span: Span,
}

// =============================================================================
// Generics (for advanced features)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParams {
    pub params: Vec<GenericParam>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<TypeExpr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GenericArgs {
    pub args: Vec<TypeExpr>,
    pub span: Span,
}

// =============================================================================
// Attributes (for metadata)
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    pub name: Ident,
    pub args: Vec<AttributeArg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttributeArg {
    pub name: Option<Ident>,
    pub value: AttributeValue,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AttributeValue {
    Ident(Ident),
    Literal(Literal),
}

// =============================================================================
// Statements
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Stmt {
    VarDecl(VarDeclStmt),
    Return(ReturnStmt),
    If(IfStmt),
    While(WhileStmt),
    For(ForStmt),
    Emit(EmitStmt),
    Require(RequireStmt),
    Revert(RevertStmt),
    Delete(DeleteStmt),
    Selfdestruct(SelfdestructStmt),
    Placeholder(Span), // _ in modifiers
    Expr(ExprStmt),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VarDeclStmt {
    pub ty: TypeExpr,
    pub storage_location: Option<StorageLocation>,
    pub name: Ident,
    pub initializer: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnStmt {
    pub value: Option<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfStmt {
    pub condition: Expr,
    pub then_block: Block,
    pub else_branch: Option<ElseBranch>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElseBranch {
    ElseIf(Box<IfStmt>),
    Else(Block),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileStmt {
    pub condition: Expr,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForStmt {
    pub init: Option<ForInit>,
    pub condition: Option<Expr>,
    pub update: Option<Expr>,
    pub body: Block,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ForInit {
    VarDecl(VarDeclStmt),
    Expr(Expr),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmitStmt {
    pub event: Ident,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequireStmt {
    pub condition: Expr,
    pub message: Option<SmolStr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RevertStmt {
    /// Either a string message or a custom error
    pub kind: RevertKind,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RevertKind {
    /// revert("message") or revert()
    Message(Option<SmolStr>),
    /// revert CustomError(args)
    Error { name: Ident, args: Vec<Arg> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeleteStmt {
    pub target: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfdestructStmt {
    /// The recipient address to receive the rent
    pub recipient: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExprStmt {
    pub expr: Expr,
    pub span: Span,
}

// =============================================================================
// Expressions
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Expr {
    Literal(Literal),
    Ident(Ident),
    Binary(Box<BinaryExpr>),
    Unary(Box<UnaryExpr>),
    Ternary(Box<TernaryExpr>),
    Call(Box<CallExpr>),
    MethodCall(Box<MethodCallExpr>),
    FieldAccess(Box<FieldAccessExpr>),
    Index(Box<IndexExpr>),
    Array(ArrayExpr),
    Tuple(TupleExpr),
    New(Box<NewExpr>),
    If(Box<IfExpr>),
    Assign(Box<AssignExpr>),
    Paren(Box<Expr>),
}

impl Expr {
    pub fn span(&self) -> Span {
        match self {
            Expr::Literal(lit) => lit.span(),
            Expr::Ident(id) => id.span,
            Expr::Binary(b) => b.span,
            Expr::Unary(u) => u.span,
            Expr::Ternary(t) => t.span,
            Expr::Call(c) => c.span,
            Expr::MethodCall(m) => m.span,
            Expr::FieldAccess(f) => f.span,
            Expr::Index(i) => i.span,
            Expr::Array(a) => a.span,
            Expr::Tuple(t) => t.span,
            Expr::New(n) => n.span,
            Expr::If(i) => i.span,
            Expr::Assign(a) => a.span,
            Expr::Paren(e) => e.span(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpr {
    pub left: Expr,
    pub op: BinaryOp,
    pub right: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    Exp, // **
    // Comparison
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    // Logical
    And,
    Or,
    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Not,     // !
    Neg,     // -
    BitNot,  // ~
    PreInc,  // ++x
    PreDec,  // --x
    PostInc, // x++
    PostDec, // x--
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TernaryExpr {
    pub condition: Expr,
    pub then_expr: Expr,
    pub else_expr: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallExpr {
    pub callee: Expr,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arg {
    pub name: Option<Ident>,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MethodCallExpr {
    pub receiver: Expr,
    pub method: Ident,
    pub generic_args: Option<GenericArgs>,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldAccessExpr {
    pub expr: Expr,
    pub field: Ident,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndexExpr {
    pub expr: Expr,
    pub index: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayExpr {
    pub elements: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TupleExpr {
    pub elements: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewExpr {
    pub ty: TypePath,
    pub args: Vec<Arg>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfExpr {
    pub condition: Expr,
    pub then_block: Block,
    pub else_branch: Box<IfExprElse>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum IfExprElse {
    ElseIf(IfExpr),
    Else(Block),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AssignExpr {
    pub target: Expr,
    pub op: AssignOp,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssignOp {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}

// =============================================================================
// Literals
// =============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Bool(bool, Span),
    Int(u128, Span),
    HexInt(SmolStr, Span),
    String(SmolStr, Span),
    HexString(SmolStr, Span),
    Address(SmolStr, Span),
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Bool(_, span) => *span,
            Literal::Int(_, span) => *span,
            Literal::HexInt(_, span) => *span,
            Literal::String(_, span) => *span,
            Literal::HexString(_, span) => *span,
            Literal::Address(_, span) => *span,
        }
    }
}

// =============================================================================
// Identifiers
// =============================================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Ident {
    pub name: SmolStr,
    pub span: Span,
}

impl Ident {
    pub fn new(name: impl Into<SmolStr>, span: Span) -> Self {
        Self {
            name: name.into(),
            span,
        }
    }
}
