mod lsp;
mod parser;

use lsp::Backend;
use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let (service, socket) = LspService::new(|client| Backend::new(client));
    Server::new(stdin(), stdout(), socket).serve(service).await;
}
