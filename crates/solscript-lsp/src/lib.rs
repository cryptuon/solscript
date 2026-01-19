//! SolScript Language Server Protocol Implementation
//!
//! Provides IDE features like diagnostics, go-to-definition, hover, and autocomplete.

mod completion;
mod definition;
mod diagnostics;
mod document;
mod hover;

use dashmap::DashMap;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

pub use document::Document;

/// The SolScript language server
pub struct SolScriptLanguageServer {
    /// LSP client for sending notifications
    client: Client,
    /// Open documents indexed by URI
    documents: DashMap<Url, Document>,
}

impl SolScriptLanguageServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DashMap::new(),
        }
    }

    /// Get a document by URI
    fn get_document(&self, uri: &Url) -> Option<dashmap::mapref::one::Ref<'_, Url, Document>> {
        self.documents.get(uri)
    }

    /// Analyze a document and publish diagnostics
    async fn analyze_document(&self, uri: &Url) {
        if let Some(doc) = self.documents.get(uri) {
            let diagnostics = diagnostics::get_diagnostics(&doc);
            self.client
                .publish_diagnostics(uri.clone(), diagnostics, Some(doc.version))
                .await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for SolScriptLanguageServer {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec![".".to_string(), ":".to_string()]),
                    resolve_provider: Some(false),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                document_formatting_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "solscript-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "SolScript language server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;
        let text = params.text_document.text;

        let doc = Document::new(text, version);
        self.documents.insert(uri.clone(), doc);
        self.analyze_document(&uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let version = params.text_document.version;

        if let Some(change) = params.content_changes.into_iter().last() {
            if let Some(mut doc) = self.documents.get_mut(&uri) {
                doc.update(change.text, version);
            }
        }

        self.analyze_document(&uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        self.documents.remove(&params.text_document.uri);
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        self.analyze_document(&params.text_document.uri).await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(doc) = self.get_document(uri) {
            let items = completion::get_completions(&doc, position);
            return Ok(Some(CompletionResponse::Array(items)));
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.get_document(uri) {
            return Ok(hover::get_hover(&doc, position));
        }

        Ok(None)
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(doc) = self.get_document(uri) {
            if let Some(location) = definition::get_definition(&doc, position, uri) {
                return Ok(Some(GotoDefinitionResponse::Scalar(location)));
            }
        }

        Ok(None)
    }

    async fn formatting(&self, params: DocumentFormattingParams) -> Result<Option<Vec<TextEdit>>> {
        let uri = &params.text_document.uri;

        if let Some(doc) = self.get_document(uri) {
            // Parse and format using the AST
            if let Ok(program) = solscript_parser::parse(&doc.text) {
                let formatted = format_program(&program);
                let edit = TextEdit {
                    range: Range {
                        start: Position::new(0, 0),
                        end: Position::new(u32::MAX, u32::MAX),
                    },
                    new_text: formatted,
                };
                return Ok(Some(vec![edit]));
            }
        }

        Ok(None)
    }
}

/// Format a program AST back to source code
fn format_program(program: &solscript_ast::Program) -> String {
    use solscript_ast::*;

    let mut output = String::new();

    for item in &program.items {
        match item {
            Item::Contract(c) => {
                if c.is_abstract {
                    output.push_str("abstract ");
                }
                output.push_str("contract ");
                output.push_str(&c.name.name);

                if !c.bases.is_empty() {
                    output.push_str(" is ");
                    let bases: Vec<_> = c.bases.iter().map(|b| b.name().to_string()).collect();
                    output.push_str(&bases.join(", "));
                }

                output.push_str(" {\n");

                for member in &c.members {
                    match member {
                        ContractMember::StateVar(v) => {
                            output.push_str("    ");
                            output.push_str(&format_type(&v.ty));
                            if let Some(vis) = &v.visibility {
                                output.push_str(&format!(" {}", format_visibility(vis)));
                            }
                            output.push_str(&format!(" {};\n", v.name.name));
                        }
                        ContractMember::Function(f) => {
                            output.push_str(&format_function(f, 1));
                        }
                        ContractMember::Constructor(c) => {
                            output.push_str("    constructor(");
                            let params: Vec<_> = c
                                .params
                                .iter()
                                .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
                                .collect();
                            output.push_str(&params.join(", "));
                            output.push_str(") {\n");
                            // TODO: format body
                            output.push_str("    }\n\n");
                        }
                        _ => {}
                    }
                }

                output.push_str("}\n\n");
            }
            Item::Event(e) => {
                output.push_str(&format!("event {}(", e.name.name));
                let params: Vec<_> = e
                    .params
                    .iter()
                    .map(|p| {
                        let indexed = if p.indexed { "indexed " } else { "" };
                        format!("{} {}{}", format_type(&p.ty), indexed, p.name.name)
                    })
                    .collect();
                output.push_str(&params.join(", "));
                output.push_str(");\n\n");
            }
            Item::Error(e) => {
                output.push_str(&format!("error {}(", e.name.name));
                let params: Vec<_> = e
                    .params
                    .iter()
                    .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
                    .collect();
                output.push_str(&params.join(", "));
                output.push_str(");\n\n");
            }
            Item::Interface(i) => {
                output.push_str(&format!("interface {} {{\n", i.name.name));
                for sig in &i.members {
                    output.push_str(&format_fn_sig(sig, 1));
                }
                output.push_str("}\n\n");
            }
            _ => {}
        }
    }

    output
}

fn format_type(ty: &solscript_ast::TypeExpr) -> String {
    ty.name().to_string()
}

fn format_visibility(vis: &solscript_ast::Visibility) -> &'static str {
    match vis {
        solscript_ast::Visibility::Public => "public",
        solscript_ast::Visibility::Private => "private",
        solscript_ast::Visibility::Internal => "internal",
        solscript_ast::Visibility::External => "external",
    }
}

fn format_fn_sig(sig: &solscript_ast::FnSig, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    let mut output = String::new();

    output.push_str(&format!("{}function {}(", ind, sig.name.name));

    let params: Vec<_> = sig
        .params
        .iter()
        .map(|p| format!("{} {}", p.ty.name(), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push(')');

    if let Some(vis) = &sig.visibility {
        output.push_str(&format!(" {}", format_visibility(vis)));
    }

    for m in &sig.state_mutability {
        match m {
            solscript_ast::StateMutability::View => output.push_str(" view"),
            solscript_ast::StateMutability::Pure => output.push_str(" pure"),
            solscript_ast::StateMutability::Payable => output.push_str(" payable"),
        }
    }

    if !sig.return_params.is_empty() {
        output.push_str(" returns (");
        let returns: Vec<_> = sig.return_params.iter().map(|p| p.ty.name()).collect();
        output.push_str(&returns.join(", "));
        output.push(')');
    }

    output.push_str(";\n");
    output
}

fn format_function(f: &solscript_ast::FnDef, indent: usize) -> String {
    let ind = "    ".repeat(indent);
    let mut output = String::new();

    // Attributes
    for attr in &f.attributes {
        output.push_str(&format!("{}#[{}]\n", ind, attr.name.name));
    }

    output.push_str(&format!("{}function {}(", ind, f.name.name));

    let params: Vec<_> = f
        .params
        .iter()
        .map(|p| format!("{} {}", format_type(&p.ty), p.name.name))
        .collect();
    output.push_str(&params.join(", "));
    output.push(')');

    // Visibility
    if let Some(vis) = &f.visibility {
        output.push_str(&format!(" {}", format_visibility(vis)));
    }

    // State mutability
    for m in &f.state_mutability {
        match m {
            solscript_ast::StateMutability::View => output.push_str(" view"),
            solscript_ast::StateMutability::Pure => output.push_str(" pure"),
            solscript_ast::StateMutability::Payable => output.push_str(" payable"),
        }
    }

    // Returns
    if !f.return_params.is_empty() {
        output.push_str(" returns (");
        let returns: Vec<_> = f.return_params.iter().map(|p| format_type(&p.ty)).collect();
        output.push_str(&returns.join(", "));
        output.push(')');
    }

    if f.body.is_some() {
        output.push_str(" {\n");
        // TODO: format body statements
        output.push_str(&format!("{}    // ...\n", ind));
        output.push_str(&format!("{}}}\n\n", ind));
    } else {
        output.push_str(";\n\n");
    }

    output
}
