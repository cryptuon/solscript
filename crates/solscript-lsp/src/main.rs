//! SolScript Language Server
//!
//! Run with: solscript-lsp
//! Or configure your editor to use this as the language server for .sol files.

use tower_lsp::{LspService, Server};
use solscript_lsp::SolScriptLanguageServer;

#[tokio::main]
async fn main() {
    // Create the language server
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| SolScriptLanguageServer::new(client));

    Server::new(stdin, stdout, socket).serve(service).await;
}
