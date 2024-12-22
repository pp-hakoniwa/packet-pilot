import { TerminalManager } from './terminal.js';

document.addEventListener('DOMContentLoaded', () => {
    const terminalManager = new TerminalManager();
        
    terminalManager.initialize();

    // クリアボタンの取得とクリックイベントリスナーの追加
    const clearButton = document.getElementById('clearButton');
    if (clearButton) {
        clearButton.addEventListener('click', () => {
            terminalManager.clear();  // ターミナルをクリア
        });
    }
});