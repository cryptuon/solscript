import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export function activate(context: vscode.ExtensionContext) {
    console.log('SolScript extension activating...');

    // Get the server path from configuration
    const config = vscode.workspace.getConfiguration('solscript');
    const serverPath = config.get<string>('server.path') || 'solscript';

    // Server options - use the SolScript LSP
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: ['lsp', '--stdio'],
        transport: TransportKind.stdio
    };

    // Client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [
            { scheme: 'file', language: 'solscript' }
        ],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.sol')
        },
        outputChannelName: 'SolScript'
    };

    // Create the language client
    client = new LanguageClient(
        'solscript',
        'SolScript Language Server',
        serverOptions,
        clientOptions
    );

    // Start the client
    client.start();

    // Register commands
    context.subscriptions.push(
        vscode.commands.registerCommand('solscript.build', buildCommand),
        vscode.commands.registerCommand('solscript.check', checkCommand),
        vscode.commands.registerCommand('solscript.restartServer', restartServerCommand)
    );

    console.log('SolScript extension activated');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function buildCommand() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'solscript') {
        vscode.window.showErrorMessage('Not a SolScript file');
        return;
    }

    // Save the file first
    await document.save();

    const filePath = document.uri.fsPath;
    const config = vscode.workspace.getConfiguration('solscript');
    const serverPath = config.get<string>('server.path') || 'solscript';

    const terminal = vscode.window.createTerminal('SolScript Build');
    terminal.show();
    terminal.sendText(`${serverPath} build "${filePath}"`);
}

async function checkCommand() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) {
        vscode.window.showErrorMessage('No active editor');
        return;
    }

    const document = editor.document;
    if (document.languageId !== 'solscript') {
        vscode.window.showErrorMessage('Not a SolScript file');
        return;
    }

    // Save the file first
    await document.save();

    const filePath = document.uri.fsPath;
    const config = vscode.workspace.getConfiguration('solscript');
    const serverPath = config.get<string>('server.path') || 'solscript';

    const terminal = vscode.window.createTerminal('SolScript Check');
    terminal.show();
    terminal.sendText(`${serverPath} check "${filePath}"`);
}

async function restartServerCommand() {
    if (client) {
        await client.stop();
        await client.start();
        vscode.window.showInformationMessage('SolScript Language Server restarted');
    }
}
