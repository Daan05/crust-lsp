use crate::parser;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
pub struct Backend {
    client: Client,
}

impl Backend {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(
        &self,
        _: InitializeParams,
    ) -> Result<InitializeResult, tower_lsp::jsonrpc::Error> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Crust LSP initialized")
            .await;
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let text = params.text_document.text;
        let uri = params.text_document.uri;

        let diagnostics = parser::parse(&text);
        self.client
            .publish_diagnostics(uri, diagnostics, None)
            .await;
    }

    async fn shutdown(&self) -> Result<(), tower_lsp::jsonrpc::Error> {
        Ok(())
    }

    async fn completion(
        &self,
        _: CompletionParams,
    ) -> Result<Option<CompletionResponse>, tower_lsp::jsonrpc::Error> {
        let items = vec![
            CompletionItem {
                label: "print".to_string(),
                kind: Some(CompletionItemKind::FUNCTION),
                detail: Some("Built-in function".to_string()),
                documentation: Some(Documentation::String("Prints to stdout.".into())),
                insert_text: Some("print ".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "let".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                detail: Some("Variable declaration".to_string()),
                insert_text: Some("let ".to_string()),
                ..Default::default()
            },
            CompletionItem {
                label: "while".into(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text: Some("while ${1:condition} {\n\t$0\n}".into()),
                detail: Some("While loop snippet".into()),
                documentation: Some(Documentation::String("Expands to a while loop.".into())),
                ..Default::default()
            },
            CompletionItem {
                label: "for".into(),
                kind: Some(CompletionItemKind::SNIPPET),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                insert_text: Some(
                    "for (let ${1:i} = ${2:0}; ${1} < ${4:n}; ${1}++) {\n\t$0\n}".into(),
                ),
                detail: Some("For loop snippet".into()),
                documentation: Some(Documentation::String("Expands to a for loop.".into())),
                ..Default::default()
            },
        ];

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>, tower_lsp::jsonrpc::Error> {
        let position = params.text_document_position_params.position;
        let hover = Hover {
            contents: HoverContents::Scalar(MarkedString::String(format!(
                "You hovered at line {}, character {}",
                position.line + 1,
                position.character + 1
            ))),
            range: None,
        };
        Ok(Some(hover))
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>, tower_lsp::jsonrpc::Error> {
        let _position = params.text_document_position_params.position;
        let uri = params.text_document_position_params.text_document.uri;

        // TODO: Real lookup logic here — for now return dummy
        let location = Location {
            uri: uri.clone(), // Same file
            range: Range {
                start: Position {
                    line: 0,
                    character: 0,
                },
                end: Position {
                    line: 0,
                    character: 5,
                },
            },
        };

        Ok(Some(GotoDefinitionResponse::Scalar(location)))
    }
}
