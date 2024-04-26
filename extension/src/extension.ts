// The module 'vscode' contains the VS Code extensibility API
// Import the module and reference it with the alias vscode in your code below
import * as vscode from 'vscode';
import {
	ServerOptions,
	LanguageClientOptions,
	LanguageClient
} from "vscode-languageclient/node";

let client: LanguageClient;

export function activate(context: vscode.ExtensionContext) {
	console.log('Congratulations, your extension "badlang" is now active!');

	const disposable = vscode.commands.registerCommand('badlang.helloWorld', () => {
		vscode.window.showInformationMessage('Hello World!!');
	});

	context.subscriptions.push(disposable);

    // This line of code will only be executed once when your extension is activated

    // TODO: Start server exe and communicate with it
    const serverExe = process.env.SERVER_PATH || "lsp";

	const exe = {command: serverExe, args:['-lsp'], options: {env: {RUST_LOG: "debug"}}};
    const ServerOptions: ServerOptions = {
        run: exe,
        debug: exe
    };

    const clientOptions: LanguageClientOptions = {
        // Register the server for plain text documents
        documentSelector: [
            {
                pattern: '**/*.txt',
            }
        ],

    };

    client = new LanguageClient("Hello LSP", ServerOptions, clientOptions);

    // For debugging only
    //lspClient.trace = Trace.Verbose;

    //add all disposables here
    client.start();
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}