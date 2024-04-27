use dashmap::DashMap;
use pest::error::LineColLocation;
use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    files: DashMap<Url, String>
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    ..Default::default()
                }),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec!["dummy.do_something".to_string()],
                    work_done_progress_options: Default::default(),
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                //position_encoding: Some(PositionEncodingKind::UTF8),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }


    async fn execute_command(&self, _: ExecuteCommandParams) -> Result<Option<Value>> {
        self.client
            .log_message(MessageType::INFO, "command executed!")
            .await;

        match self.client.apply_edit(WorkspaceEdit::default()).await {
            Ok(res) if res.applied => self.client.log_message(MessageType::INFO, "applied").await,
            Ok(_) => self.client.log_message(MessageType::INFO, "rejected").await,
            Err(err) => self.client.log_message(MessageType::ERROR, err).await,
        }

        Ok(None)
    }

    async fn did_open(&self, doc: DidOpenTextDocumentParams) {
        self.files.insert(doc.text_document.uri.clone(), doc.text_document.text.as_str().into());
        self.on_change(doc.text_document.uri).await;
    }

    async fn did_change(&self, change: DidChangeTextDocumentParams) {
        {
            let mut get_mut = self.files.get_mut(&change.text_document.uri).unwrap();
            let text = get_mut.value_mut();

            for change in change.content_changes {
                let mut before_rope = Rope::from_str(text);

                if let Some(range) = change.range {
                    let start = before_rope.line_to_char(range.start.line as usize);
                    let mut inside_rope = before_rope.split_off(start);
                    let start_byte = inside_rope.char_to_byte(inside_rope.utf16_cu_to_char(range.start.character as usize));
                    let end = inside_rope.line_to_char((range.end.line - range.start.line) as usize);
                    let after_rope = inside_rope.split_off(end);
                    let end_byte = after_rope.char_to_byte(after_rope.utf16_cu_to_char(range.end.character as usize));

                    let before_bytes = before_rope.len_bytes();
                    text.replace_range((before_bytes + start_byte)..(before_bytes + inside_rope.len_bytes() + end_byte), &change.text);
                }
            }
        }

        self.on_change(change.text_document.uri).await;
    }

    async fn did_save(&self, _: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;
    }

    async fn did_close(&self, doc: DidCloseTextDocumentParams) {
        self.files.remove(&doc.text_document.uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array([
            CompletionItem {
                label: "return".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "jmp".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "if".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "invert".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "delete".to_string(),
                kind: Some(CompletionItemKind::KEYWORD),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "max=".to_string(),
                kind: Some(CompletionItemKind::OPERATOR),
                ..CompletionItem::default()
            },
            CompletionItem {
                label: "min=".to_string(),
                kind: Some(CompletionItemKind::OPERATOR),
                ..CompletionItem::default()
            },
        ].into())))
    }
}

impl Backend {
    async fn on_change(&self, uri: Url) {
        let mut diags = Vec::new();
        {
            match badlang::Program::default().compile_str(&self.files.get(&uri).unwrap().as_str()) {
                Ok(_program) => {
                },
                Err(e) => {
                    let range = match e.line_col {
                        LineColLocation::Pos((line, col)) => Range {
                            start: Position {
                                line: line as u32 - 1,
                                character: col as u32,
                            },
                            end: Position {
                                line: line as u32 - 1,
                                character: col as u32,
                            },
                        },
                        LineColLocation::Span((start_line, start_col), (end_line, end_col)) => Range {
                            start: Position {
                                line: start_line as u32,
                                character: start_col as u32,
                            },
                            end: Position {
                                line: end_line as u32,
                                character: end_col as u32,
                            },
                        },
                    };
                    
                    diags.push(Diagnostic {
                        range,
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!("Syntax Error\n{}", e.to_string()),
                        ..Diagnostic::default()
                    });
                }
            }
        }
        //self.client.log_message(MessageType::INFO, &this).await;
        self.client.publish_diagnostics(uri, diags, None).await;
    }
}

#[tokio::main]
async fn main() {
    #[cfg(feature = "runtime-agnostic")]
    use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

    tracing_subscriber::fmt().init();

    let (stdin, stdout) = (tokio::io::stdin(), tokio::io::stdout());
    #[cfg(feature = "runtime-agnostic")]
    let (stdin, stdout) = (stdin.compat(), stdout.compat_write());

    let (service, socket) = LspService::new(|client| 
        Backend { 
            client, 
            files: DashMap::new()
        }
    );
    Server::new(stdin, stdout, socket).serve(service).await;
}