//! SolScript Language Server
//!
//! Run with: solscript-lsp
//! Or configure your editor to use this as the language server for .sol files.

use solscript_lsp::SolScriptLanguageServer;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    // Create the language server
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(SolScriptLanguageServer::new);

    Server::new(stdin, stdout, socket).serve(service).await;
}
