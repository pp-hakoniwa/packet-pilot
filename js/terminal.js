import { CommandProcessor } from "./commands.js";

// TerminalManagerクラス - ターミナルの表示と入力管理を担当
export class TerminalManager {
    // コンストラクタ - クラスの初期化時に呼ばれる
    constructor() {
        // インスタンス変数の初期化
        this.term            = null;    // xterm.jsのインスタンス
        this.currentInput    = '';      // 現在の入力文字列
        this.currentPosition = 0;       // カーソル位置
        this.commandHistory  = [];      // コマンド履歴
        this.historyPosition = -1;      // 履歴内の現在位置
        this.commandProc     = null;    // Command処理クラス
    }

    // ターミナルの初期化メソッド
    initialize() {
        // xtermの設定とインスタンス化
        this.term = new Terminal({
            cursorBlink: true,      // カーソルの点滅
            cursorStyle: 'block',   // カーソルのスタイル
            fontSize: 12,           // フォントサイズ
            fontFamily: 'Consolas, "Liberation Mono", Courier, monospace',
            theme: {
                background: '#000000',
                foreground: '#ffffff'
            },
            cols: 80,              // 列数
            rows: 24               // 行数
        });

        // DOMにターミナルを追加
        this.term.open(document.getElementById('terminal'));
        
        // 初期メッセージの表示
        this.term.writeln('Packet Pilot Terminal v1.0');
        this.term.writeln('Type "help" for available commands\n');
        this.term.write('$ ');
        this.commandProc = new CommandProcessor(this.term);

        // キーボードハンドラのセットアップ
        this.setupKeyboardHandler();

        //デバッグでterminalにWASM側からのメッセージを表示するために追加。2-4.
        window.showTerminal = this.showTerminal.bind(this);

    }

    // キーボード入力のハンドリング設定
    setupKeyboardHandler() {
        this.term.onKey(({ key, domEvent }) => {
            const printable = !domEvent.altKey && !domEvent.ctrlKey && !domEvent.metaKey;
    
            // Ctrl+C の処理
            if ((domEvent.ctrlKey || domEvent.metaKey) && domEvent.key === 'c') {
                domEvent.preventDefault(); // デフォルトの挙動を防止（特にコピー操作）
                const selection = window.getSelection()?.toString();
                if (selection) {
                    navigator.clipboard.writeText(selection).then(() => {
                        console.log('Copied to clipboard:', selection);
                    }).catch(err => {
                        console.error('Failed to copy:', err);
                    });
                }
            }
            // Ctrl+V の処理
            else if ((domEvent.ctrlKey || domEvent.metaKey) && domEvent.key === 'v') {
                domEvent.preventDefault(); // デフォルトの挙動を防止（特にペースト操作）
                navigator.clipboard.readText().then(text => {
                    if (text) {
                        this.currentInput += text;
                        this.currentPosition += text.length;
                        this.term.write(text); // ターミナルにペーストした内容を表示
                    }
                }).catch(err => {
                    console.error('Failed to paste:', err);
                });
            }
            else if (domEvent.keyCode === 13) {         // Enterキー
                this.term.writeln('');
                this.commandProc.parseCommand(this.currentInput);
                this.commandHistory.push(this.currentInput);
                this.historyPosition = -1;
                this.currentInput = '';
                this.term.write('$ ');
            } 
            else if (domEvent.keyCode === 8) {     // Backspaceキー
                if (this.currentPosition > 0) {
                    this.currentInput = this.currentInput.slice(0, -1);
                    this.currentPosition--;
                    this.term.write('\b \b');
                }
            }
            // 上矢印キーの処理を追加
            else if (domEvent.keyCode === 38) {    // ↑キー
                if (this.commandHistory.length > 0) {
                    // 現在の行をクリア
                    this.clearCurrentLine();
                    // 履歴位置を更新
                    if (this.historyPosition < this.commandHistory.length - 1) {
                        this.historyPosition++;
                    }
                    // プロンプトを描画
                    this.term.write('$ ');
                    // 履歴からコマンドを取得して表示
                    this.currentInput = this.commandHistory[this.commandHistory.length - 1 - this.historyPosition];
                    this.term.write(this.currentInput);
                    this.currentPosition = this.currentInput.length;
                }
            }
            // 下矢印キーの処理を追加
            else if (domEvent.keyCode === 40) {    // ↓キー
                // 現在の行を完全にクリア
                this.clearCurrentLine();
                // 履歴位置を更新
                if (this.historyPosition > -1) {
                    this.historyPosition--;
                }
                // プロンプトを再描画
                this.term.write('$ ');
                // 履歴からコマンドを取得して表示（または空）
                if (this.historyPosition === -1) {
                    this.currentInput = '';
                } else {
                    this.currentInput = this.commandHistory[this.commandHistory.length - 1 - this.historyPosition];
                }
                this.term.write(this.currentInput);
                this.currentPosition = this.currentInput.length;
            }
            else if (printable) {                  // 通常の文字入力
                this.currentInput += key;
                this.currentPosition++;
                this.term.write(key);
            }
        });
    }

    // ユーティリティメソッド
    writeln(text) { this.term.writeln(text); }    // 改行付きで書き込み
    clear() { this.term.clear(); }                // 画面クリア
    write(text) { this.term.write(text); }        // 改行なしで書き込み
    
    // 現在の行をクリア
    clearCurrentLine() {
        // カーソルを行の先頭に移動
        this.term.write('\r');
        // 現在の行を空白で埋めて消去
        this.term.write(' '.repeat(this.term.cols));
        // カーソルを再度行の先頭に戻す
        this.term.write('\r');
        // 入力内容とカーソル位置をリセット
        this.currentInput = '';
        this.currentPosition = 0;
    }

    showTerminal(message) {
        if (this.term) {
            this.term.writeln(message);
        } else {
            console.log('Terminal not initialized:', message);
        }
    }
}

// Optional: Ensure the method is available even if no TerminalManager is created
if (!window.showTerminal) {
    window.showTerminal = (message) => {
        console.log('Fallback showTerminal:', message);
    };
}else{
    console.log('window.showTerminal is not availe:', message);
}