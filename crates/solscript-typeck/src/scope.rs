//! Symbol table and scope management

use indexmap::IndexMap;
use smol_str::SmolStr;

use crate::types::{FunctionType, Type, TypeDef};

/// A symbol in the symbol table
#[derive(Debug, Clone)]
pub enum Symbol {
    /// A variable binding
    Variable(VariableSymbol),
    /// A function
    Function(FunctionSymbol),
    /// A type definition
    Type(TypeDef),
    /// A module
    Module(ModuleSymbol),
}

/// Variable symbol
#[derive(Debug, Clone)]
pub struct VariableSymbol {
    pub name: SmolStr,
    pub ty: Type,
    pub is_mutable: bool,
}

/// Function symbol
#[derive(Debug, Clone)]
pub struct FunctionSymbol {
    pub name: SmolStr,
    pub ty: FunctionType,
    pub is_public: bool,
}

/// Module symbol
#[derive(Debug, Clone)]
pub struct ModuleSymbol {
    pub name: SmolStr,
    pub symbols: IndexMap<SmolStr, Symbol>,
}

/// A scope in the symbol table
#[derive(Debug, Clone)]
pub struct Scope {
    /// Symbols defined in this scope
    pub symbols: IndexMap<SmolStr, Symbol>,
    /// The kind of scope
    pub kind: ScopeKind,
}

/// The kind of scope
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeKind {
    /// Global/module scope
    Global,
    /// Contract scope
    Contract,
    /// Function scope
    Function,
    /// Block scope (if, while, for, etc.)
    Block,
}

impl Scope {
    pub fn new(kind: ScopeKind) -> Self {
        Self {
            symbols: IndexMap::new(),
            kind,
        }
    }

    /// Define a symbol in this scope
    pub fn define(&mut self, name: SmolStr, symbol: Symbol) -> Option<Symbol> {
        self.symbols.insert(name, symbol)
    }

    /// Look up a symbol in this scope
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name)
    }
}

/// The symbol table managing nested scopes
#[derive(Debug)]
pub struct SymbolTable {
    /// Stack of scopes
    scopes: Vec<Scope>,
    /// Type definitions (global)
    type_defs: IndexMap<SmolStr, TypeDef>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut table = Self {
            scopes: vec![Scope::new(ScopeKind::Global)],
            type_defs: IndexMap::new(),
        };
        table.define_builtins();
        table
    }

    /// Define built-in types and functions
    fn define_builtins(&mut self) {
        // Built-in types are handled by PrimitiveType, no need to add here
    }

    /// Push a new scope
    pub fn push_scope(&mut self, kind: ScopeKind) {
        self.scopes.push(Scope::new(kind));
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) -> Option<Scope> {
        if self.scopes.len() > 1 {
            self.scopes.pop()
        } else {
            None
        }
    }

    /// Get the current scope kind
    pub fn current_scope_kind(&self) -> ScopeKind {
        self.scopes
            .last()
            .map(|s| s.kind)
            .unwrap_or(ScopeKind::Global)
    }

    /// Check if we're in a contract scope
    pub fn in_contract(&self) -> bool {
        self.scopes.iter().any(|s| s.kind == ScopeKind::Contract)
    }

    /// Check if we're in a function scope
    pub fn in_function(&self) -> bool {
        self.scopes.iter().any(|s| s.kind == ScopeKind::Function)
    }

    /// Define a symbol in the current scope
    pub fn define(&mut self, name: SmolStr, symbol: Symbol) -> Option<Symbol> {
        self.scopes.last_mut()?.define(name, symbol)
    }

    /// Define a variable
    pub fn define_variable(&mut self, name: SmolStr, ty: Type, is_mutable: bool) -> Option<Symbol> {
        self.define(
            name.clone(),
            Symbol::Variable(VariableSymbol {
                name,
                ty,
                is_mutable,
            }),
        )
    }

    /// Define a function
    pub fn define_function(
        &mut self,
        name: SmolStr,
        ty: FunctionType,
        is_public: bool,
    ) -> Option<Symbol> {
        self.define(
            name.clone(),
            Symbol::Function(FunctionSymbol {
                name,
                ty,
                is_public,
            }),
        )
    }

    /// Define a type
    pub fn define_type(&mut self, name: SmolStr, def: TypeDef) -> Option<TypeDef> {
        self.type_defs.insert(name, def)
    }

    /// Look up a symbol by name (searches all scopes from innermost to outermost)
    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.lookup(name) {
                return Some(symbol);
            }
        }
        None
    }

    /// Look up a variable
    pub fn lookup_variable(&self, name: &str) -> Option<&VariableSymbol> {
        self.lookup(name).and_then(|s| match s {
            Symbol::Variable(v) => Some(v),
            _ => None,
        })
    }

    /// Look up a function
    pub fn lookup_function(&self, name: &str) -> Option<&FunctionSymbol> {
        self.lookup(name).and_then(|s| match s {
            Symbol::Function(f) => Some(f),
            _ => None,
        })
    }

    /// Look up a type definition
    pub fn lookup_type(&self, name: &str) -> Option<&TypeDef> {
        self.type_defs.get(name)
    }

    /// Look up a symbol in the current scope only
    pub fn lookup_local(&self, name: &str) -> Option<&Symbol> {
        self.scopes.last()?.lookup(name)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
