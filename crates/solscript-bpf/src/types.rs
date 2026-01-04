//! Type mapping from SolScript types to LLVM types

use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum, StructType};
use inkwell::AddressSpace;
use std::collections::HashMap;

/// Type mapper for converting SolScript types to LLVM types
pub struct TypeMapper<'ctx> {
    context: &'ctx Context,
    /// Cache of struct types by name
    struct_types: HashMap<String, StructType<'ctx>>,
}

impl<'ctx> TypeMapper<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self {
            context,
            struct_types: HashMap::new(),
        }
    }

    /// Get the LLVM type for a SolScript type expression
    pub fn get_type(&mut self, ty: &solscript_ast::TypeExpr) -> BasicTypeEnum<'ctx> {
        match ty {
            solscript_ast::TypeExpr::Path(path) => {
                self.get_primitive_type(&path.name())
            }
            solscript_ast::TypeExpr::Array(arr) => {
                let element_type = self.get_type(&arr.element);
                if let Some(Some(size)) = arr.sizes.first() {
                    // Fixed-size array
                    element_type.array_type(*size as u32).into()
                } else {
                    // Dynamic array - represented as a pointer + length struct
                    self.get_dynamic_array_type(element_type)
                }
            }
            solscript_ast::TypeExpr::Mapping(_) => {
                // Mappings are represented as PDAs in Solana
                // For now, we'll use a placeholder struct
                self.get_mapping_type()
            }
            solscript_ast::TypeExpr::Tuple(tuple) => {
                let types: Vec<_> = tuple.elements.iter()
                    .map(|t| self.get_type(t))
                    .collect();
                self.context.struct_type(&types, false).into()
            }
        }
    }

    /// Get LLVM type for a primitive type name
    fn get_primitive_type(&self, name: &str) -> BasicTypeEnum<'ctx> {
        match name {
            // Unsigned integers
            "uint8" | "u8" => self.context.i8_type().into(),
            "uint16" | "u16" => self.context.i16_type().into(),
            "uint32" | "u32" => self.context.i32_type().into(),
            "uint64" | "u64" => self.context.i64_type().into(),
            "uint128" | "u128" => self.context.i128_type().into(),
            "uint256" | "u256" => self.context.custom_width_int_type(256).into(),

            // Signed integers
            "int8" | "i8" => self.context.i8_type().into(),
            "int16" | "i16" => self.context.i16_type().into(),
            "int32" | "i32" => self.context.i32_type().into(),
            "int64" | "i64" => self.context.i64_type().into(),
            "int128" | "i128" => self.context.i128_type().into(),
            "int256" | "i256" => self.context.custom_width_int_type(256).into(),

            // Boolean
            "bool" => self.context.bool_type().into(),

            // Address (32 bytes for Solana public key)
            "address" | "pubkey" | "Pubkey" => {
                self.context.i8_type().array_type(32).into()
            }

            // String (pointer to data + length)
            "string" => self.get_string_type(),

            // Bytes
            "bytes" => self.get_bytes_type(),
            "bytes32" => self.context.i8_type().array_type(32).into(),

            // Default to i64 for unknown types
            _ => self.context.i64_type().into(),
        }
    }

    /// Get the string type (pointer + length)
    fn get_string_type(&self) -> BasicTypeEnum<'ctx> {
        self.context.struct_type(
            &[
                self.context.ptr_type(AddressSpace::default()).into(),
                self.context.i64_type().into(),
            ],
            false,
        ).into()
    }

    /// Get the bytes type (dynamic byte array)
    fn get_bytes_type(&self) -> BasicTypeEnum<'ctx> {
        self.context.struct_type(
            &[
                self.context.ptr_type(AddressSpace::default()).into(),
                self.context.i64_type().into(),
            ],
            false,
        ).into()
    }

    /// Get a dynamic array type
    fn get_dynamic_array_type(&self, element_type: BasicTypeEnum<'ctx>) -> BasicTypeEnum<'ctx> {
        // Dynamic arrays are represented as { ptr, len }
        self.context.struct_type(
            &[
                self.context.ptr_type(AddressSpace::default()).into(),
                self.context.i64_type().into(),
            ],
            false,
        ).into()
    }

    /// Get the mapping type placeholder
    fn get_mapping_type(&self) -> BasicTypeEnum<'ctx> {
        // Mappings in Solana are PDAs, represented as a special struct
        self.context.struct_type(
            &[
                // PDA bump seed
                self.context.i8_type().into(),
                // Program ID
                self.context.i8_type().array_type(32).into(),
            ],
            false,
        ).into()
    }

    /// Register a custom struct type
    pub fn register_struct(&mut self, name: &str, fields: &[BasicTypeEnum<'ctx>]) -> StructType<'ctx> {
        let struct_type = self.context.struct_type(fields, false);
        self.struct_types.insert(name.to_string(), struct_type);
        struct_type
    }

    /// Get a previously registered struct type
    pub fn get_struct(&self, name: &str) -> Option<StructType<'ctx>> {
        self.struct_types.get(name).copied()
    }

    /// Get the size of a type in bytes
    pub fn size_of(&self, ty: BasicTypeEnum<'ctx>) -> u64 {
        match ty {
            BasicTypeEnum::IntType(t) => (t.get_bit_width() / 8) as u64,
            BasicTypeEnum::ArrayType(t) => {
                let elem_size = self.size_of(t.get_element_type());
                elem_size * t.len() as u64
            }
            BasicTypeEnum::StructType(t) => {
                t.get_field_types().iter()
                    .map(|f| self.size_of(*f))
                    .sum()
            }
            BasicTypeEnum::PointerType(_) => 8, // 64-bit pointers
            _ => 8, // Default
        }
    }

    /// Get the i64 type (commonly used)
    pub fn i64_type(&self) -> inkwell::types::IntType<'ctx> {
        self.context.i64_type()
    }

    /// Get the i32 type
    pub fn i32_type(&self) -> inkwell::types::IntType<'ctx> {
        self.context.i32_type()
    }

    /// Get the i8 type
    pub fn i8_type(&self) -> inkwell::types::IntType<'ctx> {
        self.context.i8_type()
    }

    /// Get the bool type
    pub fn bool_type(&self) -> inkwell::types::IntType<'ctx> {
        self.context.bool_type()
    }

    /// Get a pointer type
    pub fn ptr_type(&self) -> inkwell::types::PointerType<'ctx> {
        self.context.ptr_type(AddressSpace::default())
    }
}
