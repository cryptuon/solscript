//! Core type system types for SolScript (Solidity-style)

use indexmap::IndexMap;
use smol_str::SmolStr;
use std::fmt;

/// A unique identifier for a type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeId(pub u32);

/// The core type representation
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Primitive types
    Primitive(PrimitiveType),
    /// Unit type ()
    Unit,
    /// Never type (for functions that don't return)
    Never,
    /// A named type (struct, enum, contract, interface, etc.)
    Named(NamedType),
    /// Fixed-size array type T[N]
    Array(Box<Type>, u64),
    /// Dynamic array type T[]
    DynamicArray(Box<Type>),
    /// Tuple type (T1, T2, ...)
    Tuple(Vec<Type>),
    /// Mapping type mapping(K => V)
    Mapping(Box<Type>, Box<Type>),
    /// Function type function(A, B) returns (C)
    Function(FunctionType),
    /// A type variable (for inference)
    Var(TypeVar),
    /// An error type (used during type checking when errors occur)
    Error,
}

impl Type {
    /// Check if this is a numeric type
    pub fn is_numeric(&self) -> bool {
        matches!(self, Type::Primitive(p) if p.is_numeric())
    }

    /// Check if this is an integer type
    pub fn is_integer(&self) -> bool {
        matches!(self, Type::Primitive(p) if p.is_integer())
    }

    /// Check if this is a signed integer type
    pub fn is_signed(&self) -> bool {
        matches!(self, Type::Primitive(p) if p.is_signed())
    }

    /// Check if this is a boolean type
    pub fn is_bool(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Bool))
    }

    /// Check if this is an address type
    pub fn is_address(&self) -> bool {
        matches!(self, Type::Primitive(PrimitiveType::Address))
    }

    /// Check if this type contains any type variables
    pub fn has_type_vars(&self) -> bool {
        match self {
            Type::Var(_) => true,
            Type::Array(t, _) | Type::DynamicArray(t) => t.has_type_vars(),
            Type::Tuple(ts) => ts.iter().any(|t| t.has_type_vars()),
            Type::Mapping(k, v) => k.has_type_vars() || v.has_type_vars(),
            Type::Function(f) => {
                f.params.iter().any(|t| t.has_type_vars())
                    || f.return_type.has_type_vars()
            }
            Type::Named(n) => n.type_args.iter().any(|t| t.has_type_vars()),
            _ => false,
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Primitive(p) => write!(f, "{}", p),
            Type::Unit => write!(f, "()"),
            Type::Never => write!(f, "!"),
            Type::Named(n) => write!(f, "{}", n),
            Type::Array(t, n) => write!(f, "{}[{}]", t, n),
            Type::DynamicArray(t) => write!(f, "{}[]", t),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                for (i, t) in ts.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", t)?;
                }
                write!(f, ")")
            }
            Type::Mapping(k, v) => write!(f, "mapping({} => {})", k, v),
            Type::Function(ft) => write!(f, "{}", ft),
            Type::Var(v) => write!(f, "{}", v),
            Type::Error => write!(f, "<error>"),
        }
    }
}

/// Primitive types (Solidity-style)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    // Unsigned integers
    Uint8,
    Uint16,
    Uint24,
    Uint32,
    Uint40,
    Uint48,
    Uint56,
    Uint64,
    Uint72,
    Uint80,
    Uint88,
    Uint96,
    Uint104,
    Uint112,
    Uint120,
    Uint128,
    Uint136,
    Uint144,
    Uint152,
    Uint160,
    Uint168,
    Uint176,
    Uint184,
    Uint192,
    Uint200,
    Uint208,
    Uint216,
    Uint224,
    Uint232,
    Uint240,
    Uint248,
    Uint256,
    // Signed integers
    Int8,
    Int16,
    Int24,
    Int32,
    Int40,
    Int48,
    Int56,
    Int64,
    Int72,
    Int80,
    Int88,
    Int96,
    Int104,
    Int112,
    Int120,
    Int128,
    Int136,
    Int144,
    Int152,
    Int160,
    Int168,
    Int176,
    Int184,
    Int192,
    Int200,
    Int208,
    Int216,
    Int224,
    Int232,
    Int240,
    Int248,
    Int256,
    // Other primitives
    Bool,
    Address,
    Signer,  // Solana-specific: represents a required signer account
    String,
    Bytes,
    Bytes1,
    Bytes2,
    Bytes3,
    Bytes4,
    Bytes5,
    Bytes6,
    Bytes7,
    Bytes8,
    Bytes9,
    Bytes10,
    Bytes11,
    Bytes12,
    Bytes13,
    Bytes14,
    Bytes15,
    Bytes16,
    Bytes17,
    Bytes18,
    Bytes19,
    Bytes20,
    Bytes21,
    Bytes22,
    Bytes23,
    Bytes24,
    Bytes25,
    Bytes26,
    Bytes27,
    Bytes28,
    Bytes29,
    Bytes30,
    Bytes31,
    Bytes32,
}

impl PrimitiveType {
    pub fn is_numeric(&self) -> bool {
        self.is_integer()
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Uint8 | PrimitiveType::Uint16 | PrimitiveType::Uint24 |
            PrimitiveType::Uint32 | PrimitiveType::Uint40 | PrimitiveType::Uint48 |
            PrimitiveType::Uint56 | PrimitiveType::Uint64 | PrimitiveType::Uint72 |
            PrimitiveType::Uint80 | PrimitiveType::Uint88 | PrimitiveType::Uint96 |
            PrimitiveType::Uint104 | PrimitiveType::Uint112 | PrimitiveType::Uint120 |
            PrimitiveType::Uint128 | PrimitiveType::Uint136 | PrimitiveType::Uint144 |
            PrimitiveType::Uint152 | PrimitiveType::Uint160 | PrimitiveType::Uint168 |
            PrimitiveType::Uint176 | PrimitiveType::Uint184 | PrimitiveType::Uint192 |
            PrimitiveType::Uint200 | PrimitiveType::Uint208 | PrimitiveType::Uint216 |
            PrimitiveType::Uint224 | PrimitiveType::Uint232 | PrimitiveType::Uint240 |
            PrimitiveType::Uint248 | PrimitiveType::Uint256 |
            PrimitiveType::Int8 | PrimitiveType::Int16 | PrimitiveType::Int24 |
            PrimitiveType::Int32 | PrimitiveType::Int40 | PrimitiveType::Int48 |
            PrimitiveType::Int56 | PrimitiveType::Int64 | PrimitiveType::Int72 |
            PrimitiveType::Int80 | PrimitiveType::Int88 | PrimitiveType::Int96 |
            PrimitiveType::Int104 | PrimitiveType::Int112 | PrimitiveType::Int120 |
            PrimitiveType::Int128 | PrimitiveType::Int136 | PrimitiveType::Int144 |
            PrimitiveType::Int152 | PrimitiveType::Int160 | PrimitiveType::Int168 |
            PrimitiveType::Int176 | PrimitiveType::Int184 | PrimitiveType::Int192 |
            PrimitiveType::Int200 | PrimitiveType::Int208 | PrimitiveType::Int216 |
            PrimitiveType::Int224 | PrimitiveType::Int232 | PrimitiveType::Int240 |
            PrimitiveType::Int248 | PrimitiveType::Int256
        )
    }

    pub fn is_signed(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Int8 | PrimitiveType::Int16 | PrimitiveType::Int24 |
            PrimitiveType::Int32 | PrimitiveType::Int40 | PrimitiveType::Int48 |
            PrimitiveType::Int56 | PrimitiveType::Int64 | PrimitiveType::Int72 |
            PrimitiveType::Int80 | PrimitiveType::Int88 | PrimitiveType::Int96 |
            PrimitiveType::Int104 | PrimitiveType::Int112 | PrimitiveType::Int120 |
            PrimitiveType::Int128 | PrimitiveType::Int136 | PrimitiveType::Int144 |
            PrimitiveType::Int152 | PrimitiveType::Int160 | PrimitiveType::Int168 |
            PrimitiveType::Int176 | PrimitiveType::Int184 | PrimitiveType::Int192 |
            PrimitiveType::Int200 | PrimitiveType::Int208 | PrimitiveType::Int216 |
            PrimitiveType::Int224 | PrimitiveType::Int232 | PrimitiveType::Int240 |
            PrimitiveType::Int248 | PrimitiveType::Int256
        )
    }

    pub fn is_fixed_bytes(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Bytes1 | PrimitiveType::Bytes2 | PrimitiveType::Bytes3 |
            PrimitiveType::Bytes4 | PrimitiveType::Bytes5 | PrimitiveType::Bytes6 |
            PrimitiveType::Bytes7 | PrimitiveType::Bytes8 | PrimitiveType::Bytes9 |
            PrimitiveType::Bytes10 | PrimitiveType::Bytes11 | PrimitiveType::Bytes12 |
            PrimitiveType::Bytes13 | PrimitiveType::Bytes14 | PrimitiveType::Bytes15 |
            PrimitiveType::Bytes16 | PrimitiveType::Bytes17 | PrimitiveType::Bytes18 |
            PrimitiveType::Bytes19 | PrimitiveType::Bytes20 | PrimitiveType::Bytes21 |
            PrimitiveType::Bytes22 | PrimitiveType::Bytes23 | PrimitiveType::Bytes24 |
            PrimitiveType::Bytes25 | PrimitiveType::Bytes26 | PrimitiveType::Bytes27 |
            PrimitiveType::Bytes28 | PrimitiveType::Bytes29 | PrimitiveType::Bytes30 |
            PrimitiveType::Bytes31 | PrimitiveType::Bytes32
        )
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            // uint aliases
            "uint" | "uint256" => Some(Self::Uint256),
            "uint8" => Some(Self::Uint8),
            "uint16" => Some(Self::Uint16),
            "uint24" => Some(Self::Uint24),
            "uint32" => Some(Self::Uint32),
            "uint40" => Some(Self::Uint40),
            "uint48" => Some(Self::Uint48),
            "uint56" => Some(Self::Uint56),
            "uint64" => Some(Self::Uint64),
            "uint72" => Some(Self::Uint72),
            "uint80" => Some(Self::Uint80),
            "uint88" => Some(Self::Uint88),
            "uint96" => Some(Self::Uint96),
            "uint104" => Some(Self::Uint104),
            "uint112" => Some(Self::Uint112),
            "uint120" => Some(Self::Uint120),
            "uint128" => Some(Self::Uint128),
            "uint136" => Some(Self::Uint136),
            "uint144" => Some(Self::Uint144),
            "uint152" => Some(Self::Uint152),
            "uint160" => Some(Self::Uint160),
            "uint168" => Some(Self::Uint168),
            "uint176" => Some(Self::Uint176),
            "uint184" => Some(Self::Uint184),
            "uint192" => Some(Self::Uint192),
            "uint200" => Some(Self::Uint200),
            "uint208" => Some(Self::Uint208),
            "uint216" => Some(Self::Uint216),
            "uint224" => Some(Self::Uint224),
            "uint232" => Some(Self::Uint232),
            "uint240" => Some(Self::Uint240),
            "uint248" => Some(Self::Uint248),
            // int aliases
            "int" | "int256" => Some(Self::Int256),
            "int8" => Some(Self::Int8),
            "int16" => Some(Self::Int16),
            "int24" => Some(Self::Int24),
            "int32" => Some(Self::Int32),
            "int40" => Some(Self::Int40),
            "int48" => Some(Self::Int48),
            "int56" => Some(Self::Int56),
            "int64" => Some(Self::Int64),
            "int72" => Some(Self::Int72),
            "int80" => Some(Self::Int80),
            "int88" => Some(Self::Int88),
            "int96" => Some(Self::Int96),
            "int104" => Some(Self::Int104),
            "int112" => Some(Self::Int112),
            "int120" => Some(Self::Int120),
            "int128" => Some(Self::Int128),
            "int136" => Some(Self::Int136),
            "int144" => Some(Self::Int144),
            "int152" => Some(Self::Int152),
            "int160" => Some(Self::Int160),
            "int168" => Some(Self::Int168),
            "int176" => Some(Self::Int176),
            "int184" => Some(Self::Int184),
            "int192" => Some(Self::Int192),
            "int200" => Some(Self::Int200),
            "int208" => Some(Self::Int208),
            "int216" => Some(Self::Int216),
            "int224" => Some(Self::Int224),
            "int232" => Some(Self::Int232),
            "int240" => Some(Self::Int240),
            "int248" => Some(Self::Int248),
            // Other types
            "bool" => Some(Self::Bool),
            "address" => Some(Self::Address),
            "signer" => Some(Self::Signer),
            "string" => Some(Self::String),
            "bytes" => Some(Self::Bytes),
            "bytes1" => Some(Self::Bytes1),
            "bytes2" => Some(Self::Bytes2),
            "bytes3" => Some(Self::Bytes3),
            "bytes4" => Some(Self::Bytes4),
            "bytes5" => Some(Self::Bytes5),
            "bytes6" => Some(Self::Bytes6),
            "bytes7" => Some(Self::Bytes7),
            "bytes8" => Some(Self::Bytes8),
            "bytes9" => Some(Self::Bytes9),
            "bytes10" => Some(Self::Bytes10),
            "bytes11" => Some(Self::Bytes11),
            "bytes12" => Some(Self::Bytes12),
            "bytes13" => Some(Self::Bytes13),
            "bytes14" => Some(Self::Bytes14),
            "bytes15" => Some(Self::Bytes15),
            "bytes16" => Some(Self::Bytes16),
            "bytes17" => Some(Self::Bytes17),
            "bytes18" => Some(Self::Bytes18),
            "bytes19" => Some(Self::Bytes19),
            "bytes20" => Some(Self::Bytes20),
            "bytes21" => Some(Self::Bytes21),
            "bytes22" => Some(Self::Bytes22),
            "bytes23" => Some(Self::Bytes23),
            "bytes24" => Some(Self::Bytes24),
            "bytes25" => Some(Self::Bytes25),
            "bytes26" => Some(Self::Bytes26),
            "bytes27" => Some(Self::Bytes27),
            "bytes28" => Some(Self::Bytes28),
            "bytes29" => Some(Self::Bytes29),
            "bytes30" => Some(Self::Bytes30),
            "bytes31" => Some(Self::Bytes31),
            "bytes32" => Some(Self::Bytes32),
            _ => None,
        }
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrimitiveType::Uint8 => write!(f, "uint8"),
            PrimitiveType::Uint16 => write!(f, "uint16"),
            PrimitiveType::Uint24 => write!(f, "uint24"),
            PrimitiveType::Uint32 => write!(f, "uint32"),
            PrimitiveType::Uint40 => write!(f, "uint40"),
            PrimitiveType::Uint48 => write!(f, "uint48"),
            PrimitiveType::Uint56 => write!(f, "uint56"),
            PrimitiveType::Uint64 => write!(f, "uint64"),
            PrimitiveType::Uint72 => write!(f, "uint72"),
            PrimitiveType::Uint80 => write!(f, "uint80"),
            PrimitiveType::Uint88 => write!(f, "uint88"),
            PrimitiveType::Uint96 => write!(f, "uint96"),
            PrimitiveType::Uint104 => write!(f, "uint104"),
            PrimitiveType::Uint112 => write!(f, "uint112"),
            PrimitiveType::Uint120 => write!(f, "uint120"),
            PrimitiveType::Uint128 => write!(f, "uint128"),
            PrimitiveType::Uint136 => write!(f, "uint136"),
            PrimitiveType::Uint144 => write!(f, "uint144"),
            PrimitiveType::Uint152 => write!(f, "uint152"),
            PrimitiveType::Uint160 => write!(f, "uint160"),
            PrimitiveType::Uint168 => write!(f, "uint168"),
            PrimitiveType::Uint176 => write!(f, "uint176"),
            PrimitiveType::Uint184 => write!(f, "uint184"),
            PrimitiveType::Uint192 => write!(f, "uint192"),
            PrimitiveType::Uint200 => write!(f, "uint200"),
            PrimitiveType::Uint208 => write!(f, "uint208"),
            PrimitiveType::Uint216 => write!(f, "uint216"),
            PrimitiveType::Uint224 => write!(f, "uint224"),
            PrimitiveType::Uint232 => write!(f, "uint232"),
            PrimitiveType::Uint240 => write!(f, "uint240"),
            PrimitiveType::Uint248 => write!(f, "uint248"),
            PrimitiveType::Uint256 => write!(f, "uint256"),
            PrimitiveType::Int8 => write!(f, "int8"),
            PrimitiveType::Int16 => write!(f, "int16"),
            PrimitiveType::Int24 => write!(f, "int24"),
            PrimitiveType::Int32 => write!(f, "int32"),
            PrimitiveType::Int40 => write!(f, "int40"),
            PrimitiveType::Int48 => write!(f, "int48"),
            PrimitiveType::Int56 => write!(f, "int56"),
            PrimitiveType::Int64 => write!(f, "int64"),
            PrimitiveType::Int72 => write!(f, "int72"),
            PrimitiveType::Int80 => write!(f, "int80"),
            PrimitiveType::Int88 => write!(f, "int88"),
            PrimitiveType::Int96 => write!(f, "int96"),
            PrimitiveType::Int104 => write!(f, "int104"),
            PrimitiveType::Int112 => write!(f, "int112"),
            PrimitiveType::Int120 => write!(f, "int120"),
            PrimitiveType::Int128 => write!(f, "int128"),
            PrimitiveType::Int136 => write!(f, "int136"),
            PrimitiveType::Int144 => write!(f, "int144"),
            PrimitiveType::Int152 => write!(f, "int152"),
            PrimitiveType::Int160 => write!(f, "int160"),
            PrimitiveType::Int168 => write!(f, "int168"),
            PrimitiveType::Int176 => write!(f, "int176"),
            PrimitiveType::Int184 => write!(f, "int184"),
            PrimitiveType::Int192 => write!(f, "int192"),
            PrimitiveType::Int200 => write!(f, "int200"),
            PrimitiveType::Int208 => write!(f, "int208"),
            PrimitiveType::Int216 => write!(f, "int216"),
            PrimitiveType::Int224 => write!(f, "int224"),
            PrimitiveType::Int232 => write!(f, "int232"),
            PrimitiveType::Int240 => write!(f, "int240"),
            PrimitiveType::Int248 => write!(f, "int248"),
            PrimitiveType::Int256 => write!(f, "int256"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Address => write!(f, "address"),
            PrimitiveType::Signer => write!(f, "signer"),
            PrimitiveType::String => write!(f, "string"),
            PrimitiveType::Bytes => write!(f, "bytes"),
            PrimitiveType::Bytes1 => write!(f, "bytes1"),
            PrimitiveType::Bytes2 => write!(f, "bytes2"),
            PrimitiveType::Bytes3 => write!(f, "bytes3"),
            PrimitiveType::Bytes4 => write!(f, "bytes4"),
            PrimitiveType::Bytes5 => write!(f, "bytes5"),
            PrimitiveType::Bytes6 => write!(f, "bytes6"),
            PrimitiveType::Bytes7 => write!(f, "bytes7"),
            PrimitiveType::Bytes8 => write!(f, "bytes8"),
            PrimitiveType::Bytes9 => write!(f, "bytes9"),
            PrimitiveType::Bytes10 => write!(f, "bytes10"),
            PrimitiveType::Bytes11 => write!(f, "bytes11"),
            PrimitiveType::Bytes12 => write!(f, "bytes12"),
            PrimitiveType::Bytes13 => write!(f, "bytes13"),
            PrimitiveType::Bytes14 => write!(f, "bytes14"),
            PrimitiveType::Bytes15 => write!(f, "bytes15"),
            PrimitiveType::Bytes16 => write!(f, "bytes16"),
            PrimitiveType::Bytes17 => write!(f, "bytes17"),
            PrimitiveType::Bytes18 => write!(f, "bytes18"),
            PrimitiveType::Bytes19 => write!(f, "bytes19"),
            PrimitiveType::Bytes20 => write!(f, "bytes20"),
            PrimitiveType::Bytes21 => write!(f, "bytes21"),
            PrimitiveType::Bytes22 => write!(f, "bytes22"),
            PrimitiveType::Bytes23 => write!(f, "bytes23"),
            PrimitiveType::Bytes24 => write!(f, "bytes24"),
            PrimitiveType::Bytes25 => write!(f, "bytes25"),
            PrimitiveType::Bytes26 => write!(f, "bytes26"),
            PrimitiveType::Bytes27 => write!(f, "bytes27"),
            PrimitiveType::Bytes28 => write!(f, "bytes28"),
            PrimitiveType::Bytes29 => write!(f, "bytes29"),
            PrimitiveType::Bytes30 => write!(f, "bytes30"),
            PrimitiveType::Bytes31 => write!(f, "bytes31"),
            PrimitiveType::Bytes32 => write!(f, "bytes32"),
        }
    }
}

/// A named type (struct, enum, contract, interface)
#[derive(Debug, Clone, PartialEq)]
pub struct NamedType {
    pub name: SmolStr,
    pub type_args: Vec<Type>,
}

impl NamedType {
    pub fn new(name: impl Into<SmolStr>) -> Self {
        Self {
            name: name.into(),
            type_args: Vec::new(),
        }
    }

    pub fn with_args(name: impl Into<SmolStr>, type_args: Vec<Type>) -> Self {
        Self {
            name: name.into(),
            type_args,
        }
    }
}

impl fmt::Display for NamedType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.type_args.is_empty() {
            write!(f, "<")?;
            for (i, arg) in self.type_args.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, ">")?;
        }
        Ok(())
    }
}

/// A function type
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub return_type: Box<Type>,
}

impl fmt::Display for FunctionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "function(")?;
        for (i, param) in self.params.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", param)?;
        }
        write!(f, ") returns ({})", self.return_type)
    }
}

/// A type variable for inference
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVar(pub u32);

impl fmt::Display for TypeVar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "?T{}", self.0)
    }
}

/// Type definitions (for structs, enums, etc.)
#[derive(Debug, Clone)]
pub enum TypeDef {
    Struct(StructDef),
    Enum(EnumDef),
    Contract(ContractDef),
    Interface(InterfaceDef),
    Event(EventDef),
    Error(ErrorDef),
}

/// Struct type definition
#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: SmolStr,
    pub type_params: Vec<SmolStr>,
    pub fields: IndexMap<SmolStr, Type>,
}

/// Enum type definition (Solidity-style: simple variants only)
#[derive(Debug, Clone)]
pub struct EnumDef {
    pub name: SmolStr,
    pub variants: Vec<SmolStr>,
}

/// Contract type definition
#[derive(Debug, Clone)]
pub struct ContractDef {
    pub name: SmolStr,
    pub type_params: Vec<SmolStr>,
    pub bases: Vec<SmolStr>,
    pub state_fields: IndexMap<SmolStr, Type>,
    pub methods: IndexMap<SmolStr, FunctionType>,
    pub modifiers: IndexMap<SmolStr, ModifierType>,
}

/// Interface type definition
#[derive(Debug, Clone)]
pub struct InterfaceDef {
    pub name: SmolStr,
    pub bases: Vec<SmolStr>,
    pub methods: IndexMap<SmolStr, FunctionType>,
}

/// Event type definition
#[derive(Debug, Clone)]
pub struct EventDef {
    pub name: SmolStr,
    pub params: Vec<EventParam>,
}

/// Event parameter
#[derive(Debug, Clone)]
pub struct EventParam {
    pub name: SmolStr,
    pub ty: Type,
    pub indexed: bool,
}

/// Error type definition
#[derive(Debug, Clone)]
pub struct ErrorDef {
    pub name: SmolStr,
    pub params: Vec<ErrorParam>,
}

/// Error parameter
#[derive(Debug, Clone)]
pub struct ErrorParam {
    pub name: SmolStr,
    pub ty: Type,
}

/// Modifier type definition
#[derive(Debug, Clone)]
pub struct ModifierType {
    pub name: SmolStr,
    pub params: Vec<Type>,
}
