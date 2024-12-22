import  init,
        {
            WasmMacAddress,
            WasmIPv4Address, 
            WasmIPv6Address, 
            WasmEthernetFrame, 
            WasmPhysicalLayerFrame,
            WasmEthernetCable
        } from "../pkg/packet_pilot.js";


// CommandProcessorクラス - コマンドの処理を担当
export class CommandProcessor {
    // コンストラクタ - terminalManagerインスタンスを受け取る
    constructor(terminalManager){
        this.terminal = terminalManager;  // ターミナルへの参照を保持
        
        // ネットワーク関連の状態管理
        this.currentMac = null;
        this.currentIPv4 = null;
        this.currentIPv6 = null;
        this.currentEthernetFrame = null;
        this.currentPhysicalLayerFrame = null;
        this.currentEthernetCable = {}; // 2-4.追加({id:xxx,cable:本体}で現在配置されているケーブルを持つ)
        this.wasm = this.initWasm();
    }

    async initWasm() {
        try {
            const wasmInstance = await init();
            console.log("WASM initialized successfully");
            return wasmInstance;
        } catch (error) {
            console.error("Error initializing WASM:", error);
            throw error;
        }
    }
    // -- コマンド定義 ------------------------------------
    // 今後テストとしてコマンドを追加したいときはここに追加していく
    // --------------------------------------------------
    commands = {
        // helpコマンド - 利用可能なコマンドの一覧を表示
        help: () => {
            this.terminal.writeln('Available commands:');
            this.terminal.writeln(' mac([address]) - Create MAC address (random if no address)');
            this.terminal.writeln(' ipv4([address]) - Create IPv4 address (random if no address)');
            this.terminal.writeln(' ipv6([address]) - Create IPv6 address (random if no address)');
            this.terminal.writeln(' eth() - Create frame');
            this.terminal.writeln(' send() - Mock send: create physical layer frame');
            this.terminal.writeln(' show() - Show current packet&address&cable details');
            this.terminal.writeln(' cable([id]) - Create Cable (random if no id)')
            this.terminal.writeln(' remove_cable([id]) - Remove the Cable')
            this.terminal.writeln(' is_valid_cable([id]) - Valid check the Cable')
            this.terminal.writeln(' connect_cable(cable_id,component_id1,component_id2) - Connect cable(id) to component id1&2')
            this.terminal.writeln(' get_connect_id1(cable_id) - Get Id from cable id\'s endpoint1')
            this.terminal.writeln(' get_connect_id2(cable_id) - Get Id from cable id\'s endpoint2')
            this.terminal.writeln(' clear - Clear terminal');
            this.terminal.writeln(' help - Show this help message');
        },

        // macコマンド - MACアドレスの生成/設定
        mac: (addr = null) => {
            try {
                if (addr) {
                    addr = addr.replace(/['"]/g, '');
                    this.currentMac = WasmMacAddress.from_string(addr).to_string();
                } else {
                    this.currentMac = new WasmMacAddress().to_string();
                }
                this.terminal.writeln(`${this.currentMac}`);
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // ipv4コマンド - IPv4アドレスの生成/設定
        ipv4: (addr = null) => {
            try {
                if (addr) {
                    addr = addr.replace(/['"]/g, '');
                    this.currentIPv4 = WasmIPv4Address.from_string(addr).to_string();;
                } else {
                    this.currentIPv4 = new WasmIPv4Address().to_string();
                }
                this.terminal.writeln(`${this.currentIPv4}`);
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // ipv6コマンド - IPv6アドレスの生成/設定
        ipv6: (addr = null) => {
            try {
                if (addr) {
                    addr = addr.replace(/['"]/g, '');
                    this.currentIPv6 = WasmIPv6Address.from_string(addr).to_string();
                } else {
                    this.currentIPv6 = new WasmIPv6Address().to_string();
                }
                this.terminal.writeln(`${this.currentIPv6}`);
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // ethコマンド - イーサーネットフレームPacketの生成/確認
        eth: () => {
            try {
                let dstMac = new WasmMacAddress();
                let srcMac = new WasmMacAddress();
                let type = '0x0800';
                let data = [];

                let frame = new WasmEthernetFrame(dstMac,srcMac,type,data);
                let frameString = frame.to_string();
                let modifiedFrameString = frameString.replace(
                    /#ethertype\s+:\s+([0-9A-Fa-f]+)/i,  // `ethertype`の値をキャプチャ
                    (match, p1) => `#ethertype   : 0x${p1.toUpperCase()}`  // 0x+元の値に置き換える
                );
                
                this.currentEthernetFrame = modifiedFrameString;

                this.terminal.write(`${this.currentEthernetFrame}`);
                this.terminal.writeln('');
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // sendコマンド - 物理層のイーサーネットフレームPacketの生成/確認
        send:() =>{
            try{
                let dstMac = new WasmMacAddress();
                let srcMac = new WasmMacAddress();
                let frame = new WasmEthernetFrame(dstMac,srcMac,'0x0800',[]);

                let physicalFrame = new WasmPhysicalLayerFrame(frame);
                let frameString = physicalFrame.to_string();
                let modifiedFrameString = frameString.replace(
                    /#ethertype\s+:\s+([0-9A-Fa-f]+)/i,  // `ethertype`の値をキャプチャ
                    (match, p1) => `#ethertype   : 0x${p1.toUpperCase()}`  // 0x+元の値に置き換える
                );
                let totalLength = physicalFrame.total_length();

                this.currentPhysicalLayerFrame = modifiedFrameString;
                 
                this.terminal.write(`${this.currentPhysicalLayerFrame}`);
                
                this.terminal.writeln(`total length = ${totalLength}`);

                this.terminal.writeln('');
            }catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // cableコマンド - イーサーネットケーブルの生成
        cable: (id = null) => {
            try {
                var cable;
                if (id) {
                    id = id.replace(/['"]/g, '');
                    cable = new WasmEthernetCable(id);
                } else {
                    cable = new WasmEthernetCable();
                }
                

                this.currentEthernetCable = {
                    ...this.currentEthernetCable,
                    [cable.get_id()]: { id: cable.get_id(), cable: cable }
                };

                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // remove_cableコマンド - イーサーネットケーブルの削除
        remove_cable: (id = null) => {
            try {
                if(id === null){
                    this.terminal.writeln('idが指定されていません');
                }
                id = id.replace(/['"]/g, '');

                const cable = this.currentEthernetCable[id]?.cable;
                if(cable !== undefined){
                    cable.remove();
                    this.terminal.writeln(`id:${id}のケーブルを削除しました。`);
                }else{
                    message = `id:${id}は無効なケーブルIdです。`;
                }
                
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // is_valid_cableコマンド - イーサーネットケーブルが有効かどうか？
        is_valid_cable: (id = null) => {
            try {
                if(id === null){
                    this.terminal.writeln('idが指定されていません');
                }
                id = id.replace(/['"]/g, '');

                const cable = this.currentEthernetCable[id]?.cable;
                if(cable !== undefined){
                    var message = "";
                    message= cable.is_valid() === true ? `id:${id}のケーブルは、有効です。` :  `id:${id}のケーブルは、無効です。`;
                }else{
                    message = `id:${id}は無効なケーブルIdです。`;
                }
                this.terminal.writeln(message);
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // connect_cableコマンド - イーサーネットケーブルをコンポーネントId1、Id2に接続する
        connect_cable: (cable_id = null,id1 = null,id2 = null) => {
            try {
                if(cable_id === null){
                    this.terminal.writeln('ケーブルidが指定されていません');
                }
                if(id1 === null||id2 === null){
                    this.terminal.writeln('接続するコンポーネントのid1が指定されていません');
                }
                if(id2 === null){
                    this.terminal.writeln('接続するコンポーネントのid2が指定されていません');
                }
                cable_id = cable_id.replace(/['"]/g, '');
                id1 = id1.replace(/['"]/g, '');
                id2 = id2.replace(/['"]/g, '');
                var message = "";

                const cable = this.currentEthernetCable[cable_id]?.cable;
                if(cable !== undefined){
                    cable.connect(id1,id2);
                    message = `ケーブル:${cable.get_id()}を、${id1}と${id2}につなげました`
                }else{
                    message = `id:${cable_id}は無効なケーブルIdです。`;
                }
                this.terminal.writeln(message);
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // get_connect_id1コマンド - イーサーネットケーブルのendpoint1に繋がっているコンポーネントIdの取得
        get_connect_id1: (cable_id = null) => {
            try {
                if(cable_id === null){
                    this.terminal.writeln('ケーブルidが指定されていません');
                }
                cable_id = cable_id.replace(/['"]/g, '');
                var message = "";

                const cable = this.currentEthernetCable[cable_id]?.cable;
                if(cable !== undefined){
                    const endpoint1_id = cable.get_endpoint1_component_id();
                    message = `ケーブル:${cable.get_id()}のendpoint1には、${endpoint1_id}がつながっています`
                }else{
                    message = `id:${cable_id}は無効なケーブルIdです。`;
                }
                this.terminal.writeln(message);
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // get_connect_id2コマンド - イーサーネットケーブルのendpoint2に繋がっているコンポーネントIdの取得
        get_connect_id2: (cable_id = null) => {
            try {
                if(cable_id === null){
                    this.terminal.writeln('ケーブルidが指定されていません');
                }
                cable_id = cable_id.replace(/['"]/g, '');
                var message = "";

                const cable = this.currentEthernetCable[cable_id]?.cable;
                if(cable !== undefined){
                    const endpoint2_id = cable.get_endpoint2_component_id();
                    message = `ケーブル:${cable.get_id()}のendpoint2には、${endpoint2_id}がつながっています`
                }else{
                    message = `id:${cable_id}は無効なケーブルIdです。`;
                }
                this.terminal.writeln(message);
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
                
            } catch (e) {
                this.terminal.writeln(`Error: ${e.message}`);
            }
        },
        // clear - 画面のクリアー
        clear: () => {
            this.terminal.clear();
        },

        // show - 現在のそれぞれの設定値を表示する
        show: () => {
            if (this.currentMac) this.terminal.writeln(`${this.currentMac}`);
            if (this.currentIPv4) this.terminal.writeln(`${this.currentIPv4}`);
            if (this.currentIPv6) this.terminal.writeln(`${this.currentIPv6}`);
            if (this.currentEthernetFrame) this.terminal.writeln(`Ethernet Frame:\r\n${this.currentEthernetFrame}`);
            if (this.currentPhysicalLayerFrame) this.terminal.writeln(`Physical Layer Frame:\r\n${this.currentPhysicalLayerFrame}`);
            if (this.currentEthernetCable){
                Object.values(this.currentEthernetCable).map((one) => {
                    this.terminal.writeln(`${one.cable.to_string()}`);
                });
            }
            
        }
    };

    // コマンドの解析と実行
    parseCommand(input) {
        input = input.trim();
        if (!input) return;

        let match;
        // コマンドのパターンマッチング
        if (input === 'help' || input === 'clear' || input === 'show()') {
            const cmd = input.replace('()', '');
            this.commands[cmd]();
        }
        // 引数付きコマンドのパターンマッチング（例：mac('00:11:22:33:44:55')） 
        else if ((match = input.match(/^(\w+)\((.*)\)$/))) {
            const [, cmd, argsStr] = match;  // コマンド名と引数を分離
            if (this.commands[cmd]) {
                try {
                    // 引数の文字列をJavaScriptの配列に変換
                    const args = argsStr.trim() ? eval(`[${argsStr}]`) : [];
                    this.commands[cmd](...args);  // コマンドを実行
                } catch (e) {
                    this.terminal.writeln(`Error: ${e.message}`);
                }
            } else {
                this.terminal.writeln(`Unknown command: ${cmd}`);
            }
        } else {
            this.terminal.writeln(`Invalid command: ${input}`);
        }
    }
}