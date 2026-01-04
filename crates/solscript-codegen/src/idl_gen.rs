//! IDL Generator
//!
//! Generates Anchor IDL (Interface Definition Language) JSON for the program.

use crate::ir::*;
use crate::CodegenError;
use serde::Serialize;

/// IDL generator
pub struct IdlGenerator {
    program_name: String,
}

impl IdlGenerator {
    pub fn new() -> Self {
        Self {
            program_name: String::new(),
        }
    }

    /// Generate the IDL JSON
    pub fn generate(&mut self, ir: &SolanaProgram) -> Result<String, CodegenError> {
        self.program_name = to_snake_case(&ir.name);

        let idl = Idl {
            version: "0.1.0".to_string(),
            name: self.program_name.clone(),
            instructions: self.generate_instructions(ir)?,
            accounts: self.generate_accounts(ir)?,
            types: self.generate_types(ir)?,
            events: self.generate_events(ir)?,
            errors: self.generate_errors(ir)?,
            metadata: IdlMetadata {
                address: "11111111111111111111111111111111".to_string(),
            },
        };

        serde_json::to_string_pretty(&idl)
            .map_err(|e| CodegenError::GenerationFailed(format!("Failed to serialize IDL: {}", e)))
    }

    fn generate_instructions(&self, ir: &SolanaProgram) -> Result<Vec<IdlInstruction>, CodegenError> {
        let mut instructions = Vec::new();

        for instr in &ir.instructions {
            let args: Vec<IdlField> = instr.params
                .iter()
                .map(|p| IdlField {
                    name: to_camel_case_lower(&p.name),
                    ty: self.solana_type_to_idl_type(&p.ty),
                })
                .collect();

            // Build accounts list
            let mut accounts = vec![
                IdlAccount {
                    name: "state".to_string(),
                    is_mut: !instr.is_view,
                    is_signer: false,
                },
                IdlAccount {
                    name: "signer".to_string(),
                    is_mut: true,
                    is_signer: true,
                },
            ];

            // Add system program for initialize
            if instr.name.to_lowercase() == "initialize" {
                accounts.push(IdlAccount {
                    name: "systemProgram".to_string(),
                    is_mut: false,
                    is_signer: false,
                });
            }

            // Add mapping accounts
            for (i, access) in instr.mapping_accesses.iter().enumerate() {
                accounts.push(IdlAccount {
                    name: format!("{}_entry_{}", to_camel_case_lower(&access.mapping_name), i),
                    is_mut: !instr.is_view,
                    is_signer: false,
                });
            }

            instructions.push(IdlInstruction {
                name: to_camel_case_lower(&instr.name),
                accounts,
                args,
                returns: instr.returns.as_ref().map(|t| self.solana_type_to_idl_type(t)),
            });
        }

        Ok(instructions)
    }

    fn generate_accounts(&self, ir: &SolanaProgram) -> Result<Vec<IdlAccountDef>, CodegenError> {
        let mut accounts = Vec::new();

        // Main state account
        let state_fields: Vec<IdlField> = ir.state.fields
            .iter()
            .map(|f| IdlField {
                name: to_camel_case_lower(&f.name),
                ty: self.solana_type_to_idl_type(&f.ty),
            })
            .collect();

        accounts.push(IdlAccountDef {
            name: format!("{}State", to_camel_case(&self.program_name)),
            ty: IdlAccountType {
                kind: "struct".to_string(),
                fields: state_fields,
            },
        });

        // Mapping entry accounts
        for mapping in &ir.mappings {
            accounts.push(IdlAccountDef {
                name: format!("{}Entry", to_camel_case(&mapping.name)),
                ty: IdlAccountType {
                    kind: "struct".to_string(),
                    fields: vec![IdlField {
                        name: "value".to_string(),
                        ty: self.solana_type_to_idl_type(&mapping.value_ty),
                    }],
                },
            });
        }

        Ok(accounts)
    }

    fn generate_types(&self, ir: &SolanaProgram) -> Result<Vec<IdlTypeDef>, CodegenError> {
        let mut types = Vec::new();

        // Structs
        for s in &ir.structs {
            let fields: Vec<IdlField> = s.fields
                .iter()
                .map(|f| IdlField {
                    name: to_camel_case_lower(&f.name),
                    ty: self.solana_type_to_idl_type(&f.ty),
                })
                .collect();

            types.push(IdlTypeDef {
                name: s.name.clone(),
                ty: IdlTypeDefType::Struct { fields },
            });
        }

        // Enums
        for e in &ir.enums {
            let variants: Vec<IdlEnumVariant> = e.variants
                .iter()
                .map(|v| IdlEnumVariant {
                    name: v.clone(),
                })
                .collect();

            types.push(IdlTypeDef {
                name: e.name.clone(),
                ty: IdlTypeDefType::Enum { variants },
            });
        }

        Ok(types)
    }

    fn generate_events(&self, ir: &SolanaProgram) -> Result<Vec<IdlEvent>, CodegenError> {
        let mut events = Vec::new();

        for event in &ir.events {
            let fields: Vec<IdlEventField> = event.fields
                .iter()
                .map(|f| IdlEventField {
                    name: to_camel_case_lower(&f.name),
                    ty: self.solana_type_to_idl_type(&f.ty),
                    index: f.indexed,
                })
                .collect();

            events.push(IdlEvent {
                name: event.name.clone(),
                fields,
            });
        }

        Ok(events)
    }

    fn generate_errors(&self, ir: &SolanaProgram) -> Result<Vec<IdlError>, CodegenError> {
        let mut errors = Vec::new();

        // Built-in error
        errors.push(IdlError {
            code: 6000,
            name: "RequireFailed".to_string(),
            msg: "Requirement failed".to_string(),
        });

        // Custom errors
        for (i, error) in ir.errors.iter().enumerate() {
            errors.push(IdlError {
                code: 6001 + i as u32,
                name: error.name.clone(),
                msg: error.name.clone(),
            });
        }

        Ok(errors)
    }

    fn solana_type_to_idl_type(&self, ty: &SolanaType) -> IdlType {
        match ty {
            SolanaType::U8 => IdlType::Primitive("u8".to_string()),
            SolanaType::U16 => IdlType::Primitive("u16".to_string()),
            SolanaType::U32 => IdlType::Primitive("u32".to_string()),
            SolanaType::U64 => IdlType::Primitive("u64".to_string()),
            SolanaType::U128 => IdlType::Primitive("u128".to_string()),
            SolanaType::I8 => IdlType::Primitive("i8".to_string()),
            SolanaType::I16 => IdlType::Primitive("i16".to_string()),
            SolanaType::I32 => IdlType::Primitive("i32".to_string()),
            SolanaType::I64 => IdlType::Primitive("i64".to_string()),
            SolanaType::I128 => IdlType::Primitive("i128".to_string()),
            SolanaType::Bool => IdlType::Primitive("bool".to_string()),
            SolanaType::String => IdlType::Primitive("string".to_string()),
            SolanaType::Pubkey => IdlType::Primitive("publicKey".to_string()),
            SolanaType::Signer => IdlType::Primitive("publicKey".to_string()),
            SolanaType::Bytes => IdlType::Primitive("bytes".to_string()),
            SolanaType::FixedBytes(n) => IdlType::Array {
                array: (Box::new(IdlType::Primitive("u8".to_string())), *n),
            },
            SolanaType::Array(inner, size) => IdlType::Array {
                array: (Box::new(self.solana_type_to_idl_type(inner)), *size),
            },
            SolanaType::Vec(inner) => IdlType::Vec {
                vec: Box::new(self.solana_type_to_idl_type(inner)),
            },
            SolanaType::Option(inner) => IdlType::Option {
                option: Box::new(self.solana_type_to_idl_type(inner)),
            },
            SolanaType::Mapping(_, _) => IdlType::Primitive("bytes".to_string()), // Mappings are PDAs
            SolanaType::Custom(name) => IdlType::Defined(name.clone()),
        }
    }
}

// IDL structure types
#[derive(Serialize)]
struct Idl {
    version: String,
    name: String,
    instructions: Vec<IdlInstruction>,
    accounts: Vec<IdlAccountDef>,
    types: Vec<IdlTypeDef>,
    events: Vec<IdlEvent>,
    errors: Vec<IdlError>,
    metadata: IdlMetadata,
}

#[derive(Serialize)]
struct IdlMetadata {
    address: String,
}

#[derive(Serialize)]
struct IdlInstruction {
    name: String,
    accounts: Vec<IdlAccount>,
    args: Vec<IdlField>,
    #[serde(skip_serializing_if = "Option::is_none")]
    returns: Option<IdlType>,
}

#[derive(Serialize)]
struct IdlAccount {
    name: String,
    #[serde(rename = "isMut")]
    is_mut: bool,
    #[serde(rename = "isSigner")]
    is_signer: bool,
}

#[derive(Serialize)]
struct IdlField {
    name: String,
    #[serde(rename = "type")]
    ty: IdlType,
}

#[derive(Serialize)]
struct IdlAccountDef {
    name: String,
    #[serde(rename = "type")]
    ty: IdlAccountType,
}

#[derive(Serialize)]
struct IdlAccountType {
    kind: String,
    fields: Vec<IdlField>,
}

#[derive(Serialize)]
struct IdlTypeDef {
    name: String,
    #[serde(rename = "type")]
    ty: IdlTypeDefType,
}

#[derive(Serialize)]
#[serde(untagged)]
enum IdlTypeDefType {
    Struct { fields: Vec<IdlField> },
    Enum { variants: Vec<IdlEnumVariant> },
}

#[derive(Serialize)]
struct IdlEnumVariant {
    name: String,
}

#[derive(Serialize)]
struct IdlEvent {
    name: String,
    fields: Vec<IdlEventField>,
}

#[derive(Serialize)]
struct IdlEventField {
    name: String,
    #[serde(rename = "type")]
    ty: IdlType,
    index: bool,
}

#[derive(Serialize)]
struct IdlError {
    code: u32,
    name: String,
    msg: String,
}

#[derive(Serialize)]
#[serde(untagged)]
enum IdlType {
    Primitive(String),
    Defined(String),
    Array {
        array: (Box<IdlType>, usize),
    },
    Vec {
        vec: Box<IdlType>,
    },
    Option {
        option: Box<IdlType>,
    },
}

// Helper functions
fn to_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;

    for c in s.chars() {
        if c == '_' || c == '-' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }

    result
}

fn to_camel_case_lower(s: &str) -> String {
    let camel = to_camel_case(s);
    let mut chars = camel.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_ascii_lowercase().to_string() + chars.as_str(),
    }
}

fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
            result.push(c.to_ascii_lowercase());
        } else {
            result.push(c.to_ascii_lowercase());
        }
    }
    result
}
