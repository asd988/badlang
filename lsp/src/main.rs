use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use dashmap::DashMap;
use ropey::Rope;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
    error: Arc<AtomicBool>,
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

    async fn did_change_workspace_folders(&self, _: DidChangeWorkspaceFoldersParams) {
        self.client
            .log_message(MessageType::INFO, "workspace folders changed!")
            .await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.client
            .log_message(MessageType::INFO, "configuration changed!")
            .await;
    }

    async fn did_change_watched_files(&self, _: DidChangeWatchedFilesParams) {
        self.client
            .log_message(MessageType::INFO, "watched files have changed!")
            .await;
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
        self.files.insert(doc.text_document.uri, doc.text_document.text.as_str().into());
        self.client
            .log_message(MessageType::INFO, format!("file opened: {:?}", self.files))
            .await;
    }

    async fn did_change(&self, change: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, format!("file changed: {}", change.text_document.uri))
            .await;

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

        self.client
            .log_message(MessageType::INFO, format!("file changed: {}", text))
            .await;
    }

    async fn did_save(&self, doc: DidSaveTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file saved!")
            .await;

        match self.error.as_ref().load(Ordering::Relaxed) {
            true => {
                self.client.publish_diagnostics(doc.text_document.uri, [Diagnostic {
                    range: Range { 
                        start: Position { line: 10, character: 0 },
                        end: Position { line: 10, character: 10 }
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    message: "Example error message".to_string(),
                    ..Default::default()
                }].into(), None).await;
            }
            false => {
                self.client.publish_diagnostics(doc.text_document.uri, [].into(), None).await;
            }
        }
        // flip the error state
        self.error.as_ref().store(!self.error.as_ref().load(Ordering::Relaxed), Ordering::Relaxed);
    }

    async fn did_close(&self, doc: DidCloseTextDocumentParams) {
        self.files.remove(&doc.text_document.uri);
    }

    async fn completion(&self, _: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(Some(CompletionResponse::Array(vec![
            CompletionItem::new_simple("Hello".to_string(), "Some detail".to_string()),
            CompletionItem::new_simple("Bye".to_string(), "More detail".to_string()),
        ])))
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
            error: Arc::new(AtomicBool::new(false)),
            files: DashMap::new()
        }
    );
    Server::new(stdin, stdout, socket).serve(service).await;
}