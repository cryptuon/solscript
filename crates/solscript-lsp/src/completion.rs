//! Autocompletion for the language server

use tower_lsp::lsp_types::*;
use crate::Document;

/// Get completions at a position
pub fn get_completions(doc: &Document, position: Position) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get the line text to determine context
    let line_text = match doc.line_text(position.line) {
        Some(text) => text,
        None => return items,
    };

    let prefix = &line_text[..position.character as usize];

    // Check if we're after a dot (member access)
    if prefix.ends_with('.') {
        // Find what's before the dot
        let trimmed = prefix.trim_end_matches('.');
        let word_start = trimmed.rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);
        let object_name = &trimmed[word_start..];

        // Add member completions based on the object
        items.extend(get_member_completions(doc, object_name));
    } else {
        // Add keyword completions
        items.extend(get_keyword_completions());

        // Add type completions
        items.extend(get_type_completions());

        // Add symbol completions from AST
        if let Some(ast) = &doc.ast {
            items.extend(get_symbol_completions(ast));
        }

        // Add built-in completions
        items.extend(get_builtin_completions());
    }

    items
}

fn get_keyword_completions() -> Vec<CompletionItem> {
    let keywords = vec![
        ("contract", "contract ${1:Name} {\n\t$0\n}", "Define a contract"),
        ("function", "function ${1:name}(${2:params}) ${3:public} {\n\t$0\n}", "Define a function"),
        ("constructor", "constructor(${1:params}) {\n\t$0\n}", "Define a constructor"),
        ("modifier", "modifier ${1:name}(${2:params}) {\n\t$0\n\t_;\n}", "Define a modifier"),
        ("event", "event ${1:Name}(${2:params});", "Define an event"),
        ("error", "error ${1:Name}(${2:params});", "Define a custom error"),
        ("struct", "struct ${1:Name} {\n\t$0\n}", "Define a struct"),
        ("enum", "enum ${1:Name} {\n\t$0\n}", "Define an enum"),
        ("interface", "interface ${1:Name} {\n\t$0\n}", "Define an interface"),
        ("if", "if (${1:condition}) {\n\t$0\n}", "If statement"),
        ("else", "else {\n\t$0\n}", "Else clause"),
        ("for", "for (${1:uint256 i = 0}; ${2:i < n}; ${3:i += 1}) {\n\t$0\n}", "For loop"),
        ("while", "while (${1:condition}) {\n\t$0\n}", "While loop"),
        ("return", "return ${0};", "Return statement"),
        ("require", "require(${1:condition}, \"${2:message}\");", "Require statement"),
        ("revert", "revert(\"${1:message}\");", "Revert statement"),
        ("emit", "emit ${1:EventName}(${2:args});", "Emit event"),
        ("mapping", "mapping(${1:KeyType} => ${2:ValueType})", "Mapping type"),
        ("public", "public", "Public visibility"),
        ("private", "private", "Private visibility"),
        ("internal", "internal", "Internal visibility"),
        ("external", "external", "External visibility"),
        ("view", "view", "View function modifier"),
        ("pure", "pure", "Pure function modifier"),
        ("payable", "payable", "Payable function modifier"),
    ];

    keywords
        .into_iter()
        .map(|(label, insert, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some(detail.to_string()),
            insert_text: Some(insert.to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

fn get_type_completions() -> Vec<CompletionItem> {
    let types = vec![
        ("uint8", "8-bit unsigned integer"),
        ("uint16", "16-bit unsigned integer"),
        ("uint32", "32-bit unsigned integer"),
        ("uint64", "64-bit unsigned integer"),
        ("uint128", "128-bit unsigned integer"),
        ("uint256", "256-bit unsigned integer"),
        ("int8", "8-bit signed integer"),
        ("int16", "16-bit signed integer"),
        ("int32", "32-bit signed integer"),
        ("int64", "64-bit signed integer"),
        ("int128", "128-bit signed integer"),
        ("int256", "256-bit signed integer"),
        ("bool", "Boolean type"),
        ("string", "String type"),
        ("address", "Address type (Pubkey)"),
        ("bytes", "Dynamic byte array"),
        ("bytes32", "Fixed 32-byte array"),
    ];

    types
        .into_iter()
        .map(|(label, detail)| CompletionItem {
            label: label.to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            detail: Some(detail.to_string()),
            ..Default::default()
        })
        .collect()
}

fn get_builtin_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "msg.sender".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Address of the transaction signer".to_string()),
            insert_text: Some("msg.sender".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "block.timestamp".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Current block timestamp".to_string()),
            insert_text: Some("block.timestamp".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "clock.unix_timestamp".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Unix timestamp from Clock sysvar".to_string()),
            insert_text: Some("clock.unix_timestamp".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "clock.slot".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Current slot from Clock sysvar".to_string()),
            insert_text: Some("clock.slot".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "clock.epoch".to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some("Current epoch from Clock sysvar".to_string()),
            insert_text: Some("clock.epoch".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "rent.minimumBalance".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Get minimum rent-exempt balance".to_string()),
            insert_text: Some("rent.minimumBalance(${1:dataSize})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "rent.isExempt".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Check if account is rent-exempt".to_string()),
            insert_text: Some("rent.isExempt(${1:lamports}, ${2:dataSize})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "assert".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Assert a condition (test function)".to_string()),
            insert_text: Some("assert(${1:condition})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "assertEq".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            detail: Some("Assert equality (test function)".to_string()),
            insert_text: Some("assertEq(${1:left}, ${2:right})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

fn get_member_completions(doc: &Document, object_name: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    match object_name {
        "msg" => {
            items.push(CompletionItem {
                label: "sender".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Address of the transaction signer".to_string()),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "value".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Amount of SOL sent".to_string()),
                ..Default::default()
            });
        }
        "block" => {
            items.push(CompletionItem {
                label: "timestamp".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Current block timestamp".to_string()),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "number".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Current block number".to_string()),
                ..Default::default()
            });
        }
        "clock" => {
            items.push(CompletionItem {
                label: "unix_timestamp".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Unix timestamp".to_string()),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "slot".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Current slot".to_string()),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "epoch".to_string(),
                kind: Some(CompletionItemKind::PROPERTY),
                detail: Some("Current epoch".to_string()),
                ..Default::default()
            });
        }
        "rent" => {
            items.push(CompletionItem {
                label: "minimumBalance".to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Get minimum rent-exempt balance".to_string()),
                insert_text: Some("minimumBalance(${1:dataSize})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "isExempt".to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Check if account is rent-exempt".to_string()),
                insert_text: Some("isExempt(${1:lamports}, ${2:dataSize})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }
        "token" => {
            items.push(CompletionItem {
                label: "transfer".to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Transfer SPL tokens".to_string()),
                insert_text: Some("transfer(${1:from}, ${2:to}, ${3:authority}, ${4:amount})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "mint".to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Mint SPL tokens".to_string()),
                insert_text: Some("mint(${1:mint}, ${2:to}, ${3:authority}, ${4:amount})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
            items.push(CompletionItem {
                label: "burn".to_string(),
                kind: Some(CompletionItemKind::METHOD),
                detail: Some("Burn SPL tokens".to_string()),
                insert_text: Some("burn(${1:from}, ${2:mint}, ${3:authority}, ${4:amount})".to_string()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }
        _ => {
            // Try to find symbols in the AST
            if let Some(ast) = &doc.ast {
                items.extend(get_struct_member_completions(ast, object_name));
            }
        }
    }

    items
}

fn get_symbol_completions(ast: &solscript_ast::Program) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for item in &ast.items {
        match item {
            solscript_ast::Item::Contract(c) => {
                items.push(CompletionItem {
                    label: c.name.name.to_string(),
                    kind: Some(CompletionItemKind::CLASS),
                    detail: Some("Contract".to_string()),
                    ..Default::default()
                });

                // Add state variables and functions
                for member in &c.members {
                    match member {
                        solscript_ast::ContractMember::StateVar(v) => {
                            items.push(CompletionItem {
                                label: v.name.name.to_string(),
                                kind: Some(CompletionItemKind::FIELD),
                                detail: Some(format!("State variable: {}", v.ty.name())),
                                ..Default::default()
                            });
                        }
                        solscript_ast::ContractMember::Function(f) => {
                            items.push(CompletionItem {
                                label: f.name.name.to_string(),
                                kind: Some(CompletionItemKind::FUNCTION),
                                detail: Some("Function".to_string()),
                                ..Default::default()
                            });
                        }
                        _ => {}
                    }
                }
            }
            solscript_ast::Item::Struct(s) => {
                items.push(CompletionItem {
                    label: s.name.name.to_string(),
                    kind: Some(CompletionItemKind::STRUCT),
                    detail: Some("Struct".to_string()),
                    ..Default::default()
                });
            }
            solscript_ast::Item::Enum(e) => {
                items.push(CompletionItem {
                    label: e.name.name.to_string(),
                    kind: Some(CompletionItemKind::ENUM),
                    detail: Some("Enum".to_string()),
                    ..Default::default()
                });
            }
            solscript_ast::Item::Event(e) => {
                items.push(CompletionItem {
                    label: e.name.name.to_string(),
                    kind: Some(CompletionItemKind::EVENT),
                    detail: Some("Event".to_string()),
                    ..Default::default()
                });
            }
            solscript_ast::Item::Error(e) => {
                items.push(CompletionItem {
                    label: e.name.name.to_string(),
                    kind: Some(CompletionItemKind::CONSTANT),
                    detail: Some("Error".to_string()),
                    ..Default::default()
                });
            }
            _ => {}
        }
    }

    items
}

fn get_struct_member_completions(ast: &solscript_ast::Program, struct_name: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    for item in &ast.items {
        if let solscript_ast::Item::Struct(s) = item {
            if s.name.name == struct_name {
                for field in &s.fields {
                    items.push(CompletionItem {
                        label: field.name.name.to_string(),
                        kind: Some(CompletionItemKind::FIELD),
                        detail: Some(format!("{}", field.ty.name())),
                        ..Default::default()
                    });
                }
            }
        }
    }

    items
}
