//! Go-to-definition for the language server

use tower_lsp::lsp_types::*;
use crate::Document;

/// Get definition location for a symbol
pub fn get_definition(doc: &Document, position: Position, uri: &Url) -> Option<Location> {
    let word = doc.word_at(position.line, position.character)?;
    let ast = doc.ast.as_ref()?;

    // Search for the symbol definition in the AST
    for item in &ast.items {
        match item {
            solscript_ast::Item::Contract(c) => {
                if c.name.name == word {
                    let range = span_to_range(&c.span, doc);
                    return Some(Location {
                        uri: uri.clone(),
                        range,
                    });
                }

                // Check contract members
                for member in &c.members {
                    match member {
                        solscript_ast::ContractMember::StateVar(v) if v.name.name == word => {
                            let range = span_to_range(&v.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        solscript_ast::ContractMember::Function(f) if f.name.name == word => {
                            let range = span_to_range(&f.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        solscript_ast::ContractMember::Constructor(c) if word == "constructor" => {
                            let range = span_to_range(&c.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        solscript_ast::ContractMember::Modifier(m) if m.name.name == word => {
                            let range = span_to_range(&m.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        solscript_ast::ContractMember::Event(e) if e.name.name == word => {
                            let range = span_to_range(&e.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        solscript_ast::ContractMember::Error(e) if e.name.name == word => {
                            let range = span_to_range(&e.span, doc);
                            return Some(Location {
                                uri: uri.clone(),
                                range,
                            });
                        }
                        _ => {}
                    }
                }
            }
            solscript_ast::Item::Struct(s) => {
                if s.name.name == word {
                    let range = span_to_range(&s.span, doc);
                    return Some(Location {
                        uri: uri.clone(),
                        range,
                    });
                }

                // Check struct fields
                for field in &s.fields {
                    if field.name.name == word {
                        let range = span_to_range(&field.span, doc);
                        return Some(Location {
                            uri: uri.clone(),
                            range,
                        });
                    }
                }
            }
            solscript_ast::Item::Enum(e) => {
                if e.name.name == word {
                    let range = span_to_range(&e.span, doc);
                    return Some(Location {
                        uri: uri.clone(),
                        range,
                    });
                }

                // Check enum variants
                for variant in &e.variants {
                    if variant.name.name == word {
                        let range = span_to_range(&variant.span, doc);
                        return Some(Location {
                            uri: uri.clone(),
                            range,
                        });
                    }
                }
            }
            solscript_ast::Item::Interface(i) => {
                if i.name.name == word {
                    let range = span_to_range(&i.span, doc);
                    return Some(Location {
                        uri: uri.clone(),
                        range,
                    });
                }

                // Check interface function signatures
                for sig in &i.members {
                    if sig.name.name == word {
                        let range = span_to_range(&sig.span, doc);
                        return Some(Location {
                            uri: uri.clone(),
                            range,
                        });
                    }
                }
            }
            solscript_ast::Item::Event(e) if e.name.name == word => {
                let range = span_to_range(&e.span, doc);
                return Some(Location {
                    uri: uri.clone(),
                    range,
                });
            }
            solscript_ast::Item::Error(e) if e.name.name == word => {
                let range = span_to_range(&e.span, doc);
                return Some(Location {
                    uri: uri.clone(),
                    range,
                });
            }
            solscript_ast::Item::Function(f) if f.name.name == word => {
                let range = span_to_range(&f.span, doc);
                return Some(Location {
                    uri: uri.clone(),
                    range,
                });
            }
            _ => {}
        }
    }

    None
}

fn span_to_range(span: &solscript_ast::Span, doc: &Document) -> Range {
    let start = doc.position_at(span.start);
    let end = doc.position_at(span.end);
    Range {
        start: Position::new(start.0, start.1),
        end: Position::new(end.0, end.1),
    }
}
