import * as monaco from 'monaco-editor'

export function registerSolScriptLanguage() {
  monaco.languages.register({ id: 'solscript' })

  monaco.languages.setMonarchTokensProvider('solscript', {
    keywords: [
      'if', 'else', 'for', 'while', 'do', 'break', 'continue', 'return',
      'try', 'catch', 'throw', 'emit', 'revert', 'require', 'assert',
      'contract', 'interface', 'library', 'struct', 'enum', 'event', 'error',
      'function', 'modifier', 'constructor', 'fallback', 'receive',
      'public', 'private', 'internal', 'external', 'view', 'pure', 'payable',
      'virtual', 'override', 'abstract', 'constant', 'immutable',
      'memory', 'storage', 'calldata',
      'is', 'using', 'import', 'pragma',
      'new', 'delete', 'this', 'super', 'type', 'indexed',
      'returns', 'mapping',
    ],
    typeKeywords: [
      'uint', 'int', 'uint8', 'uint16', 'uint32', 'uint64', 'uint128', 'uint256',
      'int8', 'int16', 'int32', 'int64', 'int128', 'int256',
      'bool', 'address', 'string', 'bytes',
      'bytes1', 'bytes2', 'bytes4', 'bytes8', 'bytes16', 'bytes32',
    ],
    builtins: [
      'msg', 'block', 'tx', 'now',
      'keccak256', 'sha256', 'sha3', 'ripemd160', 'ecrecover',
      'addmod', 'mulmod', 'blockhash', 'selfdestruct',
    ],
    operators: [
      '=', '>', '<', '!', '~', '?', ':',
      '==', '<=', '>=', '!=', '&&', '||',
      '++', '--', '+', '-', '*', '/', '&', '|', '^', '%',
      '<<', '>>', '+=', '-=', '*=', '/=', '&=', '|=',
      '^=', '%=', '<<=', '>>=', '=>', '**',
    ],
    symbols: /[=><!~?:&|+\-*/^%]+/,

    tokenizer: {
      root: [
        // Comments
        [/\/\/.*$/, 'comment'],
        [/\/\*\*/, 'comment.doc', '@doccomment'],
        [/\/\*/, 'comment', '@comment'],

        // Strings
        [/"([^"\\]|\\.)*$/, 'string.invalid'],
        [/"/, 'string', '@string'],

        // Numbers
        [/0[xX][0-9a-fA-F_]+/, 'number.hex'],
        [/[0-9][0-9_]*(?:\.[0-9_]+)?(?:[eE][+-]?[0-9_]+)?/, 'number'],

        // Identifiers and keywords
        [/[a-zA-Z_]\w*/, {
          cases: {
            '@typeKeywords': 'type',
            '@keywords': 'keyword',
            '@builtins': 'variable.predefined',
            '@default': 'identifier',
          },
        }],

        // Whitespace
        { include: '@whitespace' },

        // Delimiters and operators
        [/[{}()[\]]/, '@brackets'],
        [/[<>](?!@symbols)/, '@brackets'],
        [/@symbols/, {
          cases: {
            '@operators': 'operator',
            '@default': '',
          },
        }],

        // Separator
        [/[;,.]/, 'delimiter'],

        // Attributes
        [/#\[/, 'annotation', '@attribute'],
      ],

      comment: [
        [/[^/*]+/, 'comment'],
        [/\*\//, 'comment', '@pop'],
        [/[/*]/, 'comment'],
      ],

      doccomment: [
        [/[^/*]+/, 'comment.doc'],
        [/\*\//, 'comment.doc', '@pop'],
        [/[/*]/, 'comment.doc'],
      ],

      string: [
        [/[^\\"]+/, 'string'],
        [/\\./, 'string.escape'],
        [/"/, 'string', '@pop'],
      ],

      whitespace: [
        [/[ \t\r\n]+/, 'white'],
      ],

      attribute: [
        [/[^\]]+/, 'annotation'],
        [/\]/, 'annotation', '@pop'],
      ],
    },
  })

  monaco.languages.setLanguageConfiguration('solscript', {
    comments: {
      lineComment: '//',
      blockComment: ['/*', '*/'],
    },
    brackets: [
      ['{', '}'],
      ['[', ']'],
      ['(', ')'],
    ],
    autoClosingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
    surroundingPairs: [
      { open: '{', close: '}' },
      { open: '[', close: ']' },
      { open: '(', close: ')' },
      { open: '"', close: '"' },
      { open: "'", close: "'" },
    ],
    indentationRules: {
      increaseIndentPattern: /^\s*(contract|function|modifier|constructor|if|else|for|while|struct|enum|event|error)\b.*\{\s*$/,
      decreaseIndentPattern: /^\s*\}/,
    },
  })

  // Define a dark theme for SolScript
  monaco.editor.defineTheme('solscript-dark', {
    base: 'vs-dark',
    inherit: true,
    rules: [
      { token: 'keyword', foreground: 'c586c0' },
      { token: 'type', foreground: '4ec9b0' },
      { token: 'variable.predefined', foreground: '9cdcfe' },
      { token: 'comment', foreground: '6a9955' },
      { token: 'comment.doc', foreground: '6a9955', fontStyle: 'italic' },
      { token: 'string', foreground: 'ce9178' },
      { token: 'number', foreground: 'b5cea8' },
      { token: 'number.hex', foreground: 'b5cea8' },
      { token: 'operator', foreground: 'd4d4d4' },
      { token: 'annotation', foreground: 'dcdcaa' },
      { token: 'identifier', foreground: 'd4d4d4' },
    ],
    colors: {
      'editor.background': '#1e1e2e',
      'editor.foreground': '#cdd6f4',
      'editorLineNumber.foreground': '#6c7086',
      'editorCursor.foreground': '#f5e0dc',
      'editor.selectionBackground': '#45475a',
      'editor.lineHighlightBackground': '#313244',
    },
  })
}

export function setDiagnostics(
  editor: monaco.editor.IStandaloneCodeEditor,
  diagnostics: Array<{ severity: string; message: string; start: number; end: number }>
) {
  const model = editor.getModel()
  if (!model) return

  const markers: monaco.editor.IMarkerData[] = diagnostics.map(d => {
    const startPos = model.getPositionAt(d.start)
    const endPos = model.getPositionAt(d.end || d.start + 1)

    return {
      severity: d.severity === 'error'
        ? monaco.MarkerSeverity.Error
        : monaco.MarkerSeverity.Warning,
      message: d.message,
      startLineNumber: startPos.lineNumber,
      startColumn: startPos.column,
      endLineNumber: endPos.lineNumber,
      endColumn: endPos.column,
    }
  })

  monaco.editor.setModelMarkers(model, 'solscript', markers)
}
