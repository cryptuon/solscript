//! Type expression AST nodes (Solidity-Style)

use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::{GenericArgs, Ident, Span};

/// A type expression in the source code
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    /// A simple or qualified path type: `uint256`, `address`, `ContractName`
    Path(TypePath),
    /// Mapping type: `mapping(address => uint256)`
    Mapping(Box<MappingType>),
    /// An array type: `uint256[]` (dynamic) or `uint256[10]` (fixed)
    Array(Box<ArrayType>),
    /// A tuple type: `(uint256, bool)`
    Tuple(TypeTuple),
}

impl TypeExpr {
    pub fn span(&self) -> Span {
        match self {
            TypeExpr::Path(p) => p.span,
            TypeExpr::Mapping(m) => m.span,
            TypeExpr::Array(a) => a.span,
            TypeExpr::Tuple(t) => t.span,
        }
    }

    /// Get a string representation of the type name
    pub fn name(&self) -> String {
        match self {
            TypeExpr::Path(p) => p.name().to_string(),
            TypeExpr::Mapping(m) => {
                format!("mapping({} => {})", m.key.name(), m.value.name())
            }
            TypeExpr::Array(a) => {
                let base = a.element.name().to_string();
                let mut result = base;
                for size in &a.sizes {
                    if let Some(n) = size {
                        result = format!("{}[{}]", result, n);
                    } else {
                        result = format!("{}[]", result);
                    }
                }
                result
            }
            TypeExpr::Tuple(t) => {
                let types: Vec<_> = t.elements.iter().map(|ty| ty.name()).collect();
                format!("({})", types.join(", "))
            }
        }
    }
}

/// A type path: `uint256`, `address`, `MyContract`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypePath {
    pub segments: Vec<Ident>,
    pub generic_args: Option<GenericArgs>,
    pub span: Span,
}

impl TypePath {
    /// Create a simple type path from a single identifier
    pub fn simple(name: Ident) -> Self {
        let span = name.span;
        Self {
            segments: vec![name],
            generic_args: None,
            span,
        }
    }

    /// Get the full path as a string
    pub fn full_path(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.name.as_str())
            .collect::<Vec<_>>()
            .join("::")
    }

    /// Check if this is a simple (single-segment) path
    pub fn is_simple(&self) -> bool {
        self.segments.len() == 1 && self.generic_args.is_none()
    }

    /// Get the last segment name
    pub fn name(&self) -> &SmolStr {
        &self.segments.last().unwrap().name
    }
}

/// Mapping type: `mapping(KeyType => ValueType)`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingType {
    pub key: TypeExpr,
    pub value: TypeExpr,
    pub span: Span,
}

/// Array type: `T[]` (dynamic) or `T[N]` (fixed)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayType {
    pub element: TypePath,
    pub sizes: Vec<Option<u64>>, // None = dynamic [], Some(n) = fixed [n]
    pub span: Span,
}

/// A tuple type: `(T, U, V)`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeTuple {
    pub elements: Vec<TypeExpr>,
    pub span: Span,
}

/// Built-in primitive types (Solidity-style naming)
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
    /// Try to parse a primitive type from a string
    pub fn parse(s: &str) -> Option<Self> {
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

    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Uint8 => "uint8",
            Self::Uint16 => "uint16",
            Self::Uint24 => "uint24",
            Self::Uint32 => "uint32",
            Self::Uint40 => "uint40",
            Self::Uint48 => "uint48",
            Self::Uint56 => "uint56",
            Self::Uint64 => "uint64",
            Self::Uint72 => "uint72",
            Self::Uint80 => "uint80",
            Self::Uint88 => "uint88",
            Self::Uint96 => "uint96",
            Self::Uint104 => "uint104",
            Self::Uint112 => "uint112",
            Self::Uint120 => "uint120",
            Self::Uint128 => "uint128",
            Self::Uint136 => "uint136",
            Self::Uint144 => "uint144",
            Self::Uint152 => "uint152",
            Self::Uint160 => "uint160",
            Self::Uint168 => "uint168",
            Self::Uint176 => "uint176",
            Self::Uint184 => "uint184",
            Self::Uint192 => "uint192",
            Self::Uint200 => "uint200",
            Self::Uint208 => "uint208",
            Self::Uint216 => "uint216",
            Self::Uint224 => "uint224",
            Self::Uint232 => "uint232",
            Self::Uint240 => "uint240",
            Self::Uint248 => "uint248",
            Self::Uint256 => "uint256",
            Self::Int8 => "int8",
            Self::Int16 => "int16",
            Self::Int24 => "int24",
            Self::Int32 => "int32",
            Self::Int40 => "int40",
            Self::Int48 => "int48",
            Self::Int56 => "int56",
            Self::Int64 => "int64",
            Self::Int72 => "int72",
            Self::Int80 => "int80",
            Self::Int88 => "int88",
            Self::Int96 => "int96",
            Self::Int104 => "int104",
            Self::Int112 => "int112",
            Self::Int120 => "int120",
            Self::Int128 => "int128",
            Self::Int136 => "int136",
            Self::Int144 => "int144",
            Self::Int152 => "int152",
            Self::Int160 => "int160",
            Self::Int168 => "int168",
            Self::Int176 => "int176",
            Self::Int184 => "int184",
            Self::Int192 => "int192",
            Self::Int200 => "int200",
            Self::Int208 => "int208",
            Self::Int216 => "int216",
            Self::Int224 => "int224",
            Self::Int232 => "int232",
            Self::Int240 => "int240",
            Self::Int248 => "int248",
            Self::Int256 => "int256",
            Self::Bool => "bool",
            Self::Address => "address",
            Self::String => "string",
            Self::Bytes => "bytes",
            Self::Bytes1 => "bytes1",
            Self::Bytes2 => "bytes2",
            Self::Bytes3 => "bytes3",
            Self::Bytes4 => "bytes4",
            Self::Bytes5 => "bytes5",
            Self::Bytes6 => "bytes6",
            Self::Bytes7 => "bytes7",
            Self::Bytes8 => "bytes8",
            Self::Bytes9 => "bytes9",
            Self::Bytes10 => "bytes10",
            Self::Bytes11 => "bytes11",
            Self::Bytes12 => "bytes12",
            Self::Bytes13 => "bytes13",
            Self::Bytes14 => "bytes14",
            Self::Bytes15 => "bytes15",
            Self::Bytes16 => "bytes16",
            Self::Bytes17 => "bytes17",
            Self::Bytes18 => "bytes18",
            Self::Bytes19 => "bytes19",
            Self::Bytes20 => "bytes20",
            Self::Bytes21 => "bytes21",
            Self::Bytes22 => "bytes22",
            Self::Bytes23 => "bytes23",
            Self::Bytes24 => "bytes24",
            Self::Bytes25 => "bytes25",
            Self::Bytes26 => "bytes26",
            Self::Bytes27 => "bytes27",
            Self::Bytes28 => "bytes28",
            Self::Bytes29 => "bytes29",
            Self::Bytes30 => "bytes30",
            Self::Bytes31 => "bytes31",
            Self::Bytes32 => "bytes32",
        }
    }

    /// Get the byte size of the type
    pub fn byte_size(&self) -> Option<usize> {
        match self {
            Self::Uint8 | Self::Int8 | Self::Bytes1 => Some(1),
            Self::Uint16 | Self::Int16 | Self::Bytes2 => Some(2),
            Self::Uint24 | Self::Int24 | Self::Bytes3 => Some(3),
            Self::Uint32 | Self::Int32 | Self::Bytes4 => Some(4),
            Self::Uint40 | Self::Int40 | Self::Bytes5 => Some(5),
            Self::Uint48 | Self::Int48 | Self::Bytes6 => Some(6),
            Self::Uint56 | Self::Int56 | Self::Bytes7 => Some(7),
            Self::Uint64 | Self::Int64 | Self::Bytes8 => Some(8),
            Self::Uint72 | Self::Int72 | Self::Bytes9 => Some(9),
            Self::Uint80 | Self::Int80 | Self::Bytes10 => Some(10),
            Self::Uint88 | Self::Int88 | Self::Bytes11 => Some(11),
            Self::Uint96 | Self::Int96 | Self::Bytes12 => Some(12),
            Self::Uint104 | Self::Int104 | Self::Bytes13 => Some(13),
            Self::Uint112 | Self::Int112 | Self::Bytes14 => Some(14),
            Self::Uint120 | Self::Int120 | Self::Bytes15 => Some(15),
            Self::Uint128 | Self::Int128 | Self::Bytes16 => Some(16),
            Self::Uint136 | Self::Int136 | Self::Bytes17 => Some(17),
            Self::Uint144 | Self::Int144 | Self::Bytes18 => Some(18),
            Self::Uint152 | Self::Int152 | Self::Bytes19 => Some(19),
            Self::Uint160 | Self::Int160 | Self::Bytes20 | Self::Address => Some(20),
            Self::Uint168 | Self::Int168 | Self::Bytes21 => Some(21),
            Self::Uint176 | Self::Int176 | Self::Bytes22 => Some(22),
            Self::Uint184 | Self::Int184 | Self::Bytes23 => Some(23),
            Self::Uint192 | Self::Int192 | Self::Bytes24 => Some(24),
            Self::Uint200 | Self::Int200 | Self::Bytes25 => Some(25),
            Self::Uint208 | Self::Int208 | Self::Bytes26 => Some(26),
            Self::Uint216 | Self::Int216 | Self::Bytes27 => Some(27),
            Self::Uint224 | Self::Int224 | Self::Bytes28 => Some(28),
            Self::Uint232 | Self::Int232 | Self::Bytes29 => Some(29),
            Self::Uint240 | Self::Int240 | Self::Bytes30 => Some(30),
            Self::Uint248 | Self::Int248 | Self::Bytes31 => Some(31),
            Self::Uint256 | Self::Int256 | Self::Bytes32 => Some(32),
            Self::Bool => Some(1),
            Self::String | Self::Bytes => None, // Dynamic size
        }
    }
}
