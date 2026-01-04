//! Hover information for the language server

use tower_lsp::lsp_types::*;
use crate::Document;

/// Get hover information at a position
pub fn get_hover(doc: &Document, position: Position) -> Option<Hover> {
    let word = doc.word_at(position.line, position.character)?;

    // Check built-in objects first
    if let Some(hover) = get_builtin_hover(&word) {
        return Some(hover);
    }

    // Check types
    if let Some(hover) = get_type_hover(&word) {
        return Some(hover);
    }

    // Check keywords
    if let Some(hover) = get_keyword_hover(&word) {
        return Some(hover);
    }

    // Check symbols in AST
    if let Some(ast) = &doc.ast {
        if let Some(hover) = get_symbol_hover(ast, &word) {
            return Some(hover);
        }
    }

    None
}

fn get_builtin_hover(word: &str) -> Option<Hover> {
    let (contents, detail) = match word {
        "msg" => (
            "**msg** - Transaction message context",
            "Properties:\n- `sender`: Address of the transaction signer\n- `value`: Amount of SOL sent (in lamports)",
        ),
        "sender" => (
            "**msg.sender** - Transaction signer",
            "`address` - The public key of the account that signed the transaction",
        ),
        "block" => (
            "**block** - Block information",
            "Properties:\n- `timestamp`: Current block timestamp\n- `number`: Current block number (slot)",
        ),
        "clock" => (
            "**clock** - Solana Clock sysvar",
            "Properties:\n- `unix_timestamp`: Unix timestamp (i64)\n- `slot`: Current slot (u64)\n- `epoch`: Current epoch (u64)",
        ),
        "rent" => (
            "**rent** - Solana Rent sysvar",
            "Methods:\n- `minimumBalance(dataSize: uint64)`: Get minimum rent-exempt balance\n- `isExempt(lamports: uint64, dataSize: uint64)`: Check if account is rent-exempt",
        ),
        "token" => (
            "**token** - SPL Token operations",
            "Methods:\n- `transfer(from, to, authority, amount)`: Transfer tokens\n- `mint(mint, to, authority, amount)`: Mint tokens\n- `burn(from, mint, authority, amount)`: Burn tokens",
        ),
        "require" => (
            "**require** - Condition check",
            "```solscript\nrequire(condition, \"error message\");\n```\nReverts if condition is false.",
        ),
        "revert" => (
            "**revert** - Abort execution",
            "```solscript\nrevert(\"error message\");\nrevert CustomError(args);\n```\nAborts execution with an error.",
        ),
        "emit" => (
            "**emit** - Event emission",
            "```solscript\nemit EventName(arg1, arg2);\n```\nEmits an event to the transaction log.",
        ),
        "assert" => (
            "**assert** - Test assertion",
            "```solscript\nassert(condition);\nassert(condition, \"message\");\n```\nAsserts condition is true (for tests).",
        ),
        "assertEq" => (
            "**assertEq** - Equality assertion",
            "```solscript\nassertEq(left, right);\nassertEq(left, right, \"message\");\n```\nAsserts left equals right (for tests).",
        ),
        "assertNe" => (
            "**assertNe** - Inequality assertion",
            "```solscript\nassertNe(left, right);\nassertNe(left, right, \"message\");\n```\nAsserts left does not equal right (for tests).",
        ),
        "assertGt" | "assertGe" | "assertLt" | "assertLe" => {
            return Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!("**{}** - Comparison assertion\n\nAsserts comparison is true (for tests).", word),
                }),
                range: None,
            });
        }
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("{}\n\n{}", contents, detail),
        }),
        range: None,
    })
}

fn get_type_hover(word: &str) -> Option<Hover> {
    let info = match word {
        "uint8" => ("8-bit unsigned integer", "Range: 0 to 255"),
        "uint16" => ("16-bit unsigned integer", "Range: 0 to 65,535"),
        "uint32" => ("32-bit unsigned integer", "Range: 0 to 4,294,967,295"),
        "uint64" => ("64-bit unsigned integer", "Range: 0 to 18,446,744,073,709,551,615"),
        "uint128" => ("128-bit unsigned integer", "Very large positive numbers"),
        "uint256" => ("256-bit unsigned integer", "Maximum precision integer"),
        "int8" => ("8-bit signed integer", "Range: -128 to 127"),
        "int16" => ("16-bit signed integer", "Range: -32,768 to 32,767"),
        "int32" => ("32-bit signed integer", "Range: -2,147,483,648 to 2,147,483,647"),
        "int64" => ("64-bit signed integer", "Large signed numbers"),
        "int128" => ("128-bit signed integer", "Very large signed numbers"),
        "int256" => ("256-bit signed integer", "Maximum precision signed integer"),
        "bool" => ("Boolean type", "Values: `true` or `false`"),
        "string" => ("String type", "UTF-8 encoded text"),
        "address" => ("Address type", "32-byte Solana public key (Pubkey)"),
        "bytes" => ("Dynamic byte array", "Variable-length byte sequence"),
        "bytes32" => ("Fixed 32-byte array", "32-byte fixed-length array"),
        "signer" => ("Signer type", "An address parameter that must sign the transaction"),
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("**{}** - {}\n\n{}", word, info.0, info.1),
        }),
        range: None,
    })
}

fn get_keyword_hover(word: &str) -> Option<Hover> {
    let info = match word {
        "contract" => (
            "Contract declaration",
            "```solscript\ncontract Name {\n    // state and functions\n}\n```",
        ),
        "function" => (
            "Function declaration",
            "```solscript\nfunction name(params) public returns (Type) {\n    // body\n}\n```",
        ),
        "constructor" => (
            "Constructor declaration",
            "```solscript\nconstructor(params) {\n    // initialization\n}\n```\nCalled once when the contract is deployed.",
        ),
        "modifier" => (
            "Modifier declaration",
            "```solscript\nmodifier onlyOwner() {\n    require(msg.sender == owner);\n    _;\n}\n```\nModifiers add conditions to functions.",
        ),
        "event" => (
            "Event declaration",
            "```solscript\nevent Transfer(address indexed from, address indexed to, uint256 amount);\n```\nEvents are logged in transactions.",
        ),
        "error" => (
            "Custom error declaration",
            "```solscript\nerror InsufficientBalance(uint256 available, uint256 required);\n```\nCustom errors for revert.",
        ),
        "struct" => (
            "Struct declaration",
            "```solscript\nstruct Point {\n    uint256 x;\n    uint256 y;\n}\n```",
        ),
        "enum" => (
            "Enum declaration",
            "```solscript\nenum Status { Pending, Active, Completed }\n```",
        ),
        "interface" => (
            "Interface declaration",
            "```solscript\ninterface IERC20 {\n    function transfer(address to, uint256 amount) external;\n}\n```\nInterfaces for cross-program invocation.",
        ),
        "mapping" => (
            "Mapping type",
            "```solscript\nmapping(address => uint256) balances;\n```\nKey-value storage (stored as PDAs).",
        ),
        "public" => ("Public visibility", "Function can be called externally."),
        "private" => ("Private visibility", "Function can only be called internally."),
        "internal" => ("Internal visibility", "Function can be called internally or by derived contracts."),
        "external" => ("External visibility", "Function can only be called from outside."),
        "view" => ("View modifier", "Function does not modify state (read-only)."),
        "pure" => ("Pure modifier", "Function does not read or modify state."),
        "payable" => ("Payable modifier", "Function can receive SOL."),
        "abstract" => ("Abstract contract", "Cannot be deployed directly, must be inherited."),
        "is" => ("Inheritance", "```solscript\ncontract Child is Parent { }\n```"),
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!("**{}** - {}\n\n{}", word, info.0, info.1),
        }),
        range: None,
    })
}

fn get_symbol_hover(ast: &solscript_ast::Program, word: &str) -> Option<Hover> {
    for item in &ast.items {
        match item {
            solscript_ast::Item::Contract(c) => {
                if c.name.name == word {
                    let mut desc = format!("**contract {}**", word);
                    if !c.bases.is_empty() {
                        let bases: Vec<_> = c.bases.iter().map(|b| b.name().to_string()).collect();
                        desc.push_str(&format!(" is {}", bases.join(", ")));
                    }

                    let state_count = c.members.iter()
                        .filter(|m| matches!(m, solscript_ast::ContractMember::StateVar(_)))
                        .count();
                    let fn_count = c.members.iter()
                        .filter(|m| matches!(m, solscript_ast::ContractMember::Function(_)))
                        .count();

                    desc.push_str(&format!("\n\n{} state variable(s), {} function(s)", state_count, fn_count));

                    return Some(Hover {
                        contents: HoverContents::Markup(MarkupContent {
                            kind: MarkupKind::Markdown,
                            value: desc,
                        }),
                        range: None,
                    });
                }

                // Check members
                for member in &c.members {
                    match member {
                        solscript_ast::ContractMember::StateVar(v) if v.name.name == word => {
                            let vis = v.visibility.as_ref()
                                .map(|v| format!("{:?}", v).to_lowercase())
                                .unwrap_or_default();
                            return Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: format!("**{}** `{} {}`\n\nState variable in contract `{}`",
                                        word, v.ty.name(), vis, c.name.name),
                                }),
                                range: None,
                            });
                        }
                        solscript_ast::ContractMember::Function(f) if f.name.name == word => {
                            let params: Vec<_> = f.params.iter()
                                .map(|p| format!("{} {}", p.ty.name(), p.name.name))
                                .collect();
                            let returns = if f.return_params.is_empty() {
                                String::new()
                            } else {
                                let ret_types: Vec<_> = f.return_params.iter()
                                    .map(|p| p.ty.name().to_string())
                                    .collect();
                                format!(" returns ({})", ret_types.join(", "))
                            };
                            let vis = f.visibility.as_ref()
                                .map(|v| format!(" {:?}", v).to_lowercase())
                                .unwrap_or_default();

                            return Some(Hover {
                                contents: HoverContents::Markup(MarkupContent {
                                    kind: MarkupKind::Markdown,
                                    value: format!("```solscript\nfunction {}({}){}{}\n```\n\nFunction in contract `{}`",
                                        word, params.join(", "), vis, returns, c.name.name),
                                }),
                                range: None,
                            });
                        }
                        _ => {}
                    }
                }
            }
            solscript_ast::Item::Struct(s) if s.name.name == word => {
                let fields: Vec<_> = s.fields.iter()
                    .map(|f| format!("    {} {};", f.ty.name(), f.name.name))
                    .collect();
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```solscript\nstruct {} {{\n{}\n}}\n```", word, fields.join("\n")),
                    }),
                    range: None,
                });
            }
            solscript_ast::Item::Enum(e) if e.name.name == word => {
                let variants: Vec<_> = e.variants.iter()
                    .map(|v| v.name.name.to_string())
                    .collect();
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```solscript\nenum {} {{ {} }}\n```", word, variants.join(", ")),
                    }),
                    range: None,
                });
            }
            solscript_ast::Item::Event(e) if e.name.name == word => {
                let params: Vec<_> = e.params.iter()
                    .map(|p| {
                        let indexed = if p.indexed { "indexed " } else { "" };
                        format!("{} {}{}", p.ty.name(), indexed, p.name.name)
                    })
                    .collect();
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```solscript\nevent {}({});\n```", word, params.join(", ")),
                    }),
                    range: None,
                });
            }
            solscript_ast::Item::Error(e) if e.name.name == word => {
                let params: Vec<_> = e.params.iter()
                    .map(|p| format!("{} {}", p.ty.name(), p.name.name))
                    .collect();
                return Some(Hover {
                    contents: HoverContents::Markup(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format!("```solscript\nerror {}({});\n```", word, params.join(", ")),
                    }),
                    range: None,
                });
            }
            _ => {}
        }
    }

    None
}
