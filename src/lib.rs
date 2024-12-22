// 各ネットワーク層の実装をモジュールとして分割
pub(crate) mod layer1;  // 物理層の実装
pub(crate) mod layer2;  // データリンク層の実装
pub(crate) mod layer3;  // ネットワーク層の実装

use layer1::component::EthernetCable;
use layer1::PhysicalLayerCallback;
// 必要なクレートをインポート
use wasm_bindgen::prelude::*;      // WebAssembly関連の機能
use wasm_bindgen::JsValue;         // JavaScript値との相互運用
use js_sys::Uint8Array;            // JavaScript配列型との相互運用

// 必要な型をインポート
use crate::layer1::packets::PhysicalLayerFrame; // 物理層フレーム
use crate::layer2::packets::EthernetFrame;      // イーサネットフレーム
use crate::layer2::address::MacAddress;         // MACアドレス
use crate::layer3::address::IPv4Address;        // IPv4アドレス
use crate::layer3::address::IPv6Address;        // IPv6アドレス


//////////////////////////////////////////////
// MACアドレスのWebAssembly対応ラッパー構造体
//////////////////////////////////////////////

/// WebAssemblyからMACアドレスを扱うためのラッパー構造体
/// inner_mac: 内部に保持する実際のMacAddressインスタンス
#[wasm_bindgen]
pub struct WasmMacAddress {
    inner_mac: MacAddress  // MacAddressインスタンスを明示的な名前で保持
}

#[wasm_bindgen]
impl WasmMacAddress {
    /// 新しいMACアドレスインスタンスを作成
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let mac = new WasmMacAddress();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // 内部のMacAddressインスタンスを作成して保持
        WasmMacAddress {
            inner_mac: MacAddress::new()
        }
    }

    /// 文字列からMACアドレスを生成
    /// 
    /// ### 引数
    /// * `mac_str` - "00:11:22:33:44:55" 形式のMACアドレス文字列
    /// 
    /// ### 戻り値
    /// * `Result<WasmMacAddress, JsValue>` - 成功時はWasmMacAddress、失敗時はエラーメッセージ
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let mac = WasmMacAddress.from_string("00:11:22:33:44:55");
    /// ```
    #[wasm_bindgen]
    pub fn from_string(mac_str: &str) -> Result<WasmMacAddress, JsValue> {
        // 文字列をMacAddressに変換を試みる
        match MacAddress::from_string(mac_str) {
            // 変換成功時は新しいWasmMacAddressインスタンスを作成
            Ok(mac_address) => Ok(WasmMacAddress {
                inner_mac: mac_address
            }),
            // 変換失敗時はエラーメッセージをJavaScript用の値に変換
            Err(error_message) => Err(JsValue::from_str(error_message))
        }
    }

    /// MACアドレスを文字列形式で取得
    /// 
    /// ### 戻り値
    /// * `String` - "00:11:22:33:44:55" 形式のMACアドレス文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のMacAddressインスタンスの文字列表現を取得
        self.inner_mac.to_string()
    }

    /// MACアドレスをバイト配列として取得
    /// 
    /// ### 戻り値
    /// * `Uint8Array` - 6バイトのMACアドレスデータ
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Uint8Array {
        // 内部のMacAddressインスタンスからバイト配列を取得
        let mac_bytes = self.inner_mac.to_array();
        // バイト配列をJavaScript用のUint8Arrayに変換
        Uint8Array::from(&mac_bytes[..])
    }
}

//////////////////////////////////////////////
// IPv4/IPv6アドレスのWebAssembly対応ラッパー構造体
//////////////////////////////////////////////

/// WebAssemblyからIPv4アドレスを扱うためのラッパー構造体
/// inner_ip: 内部に保持する実際のIPv4Addressインスタンス
#[wasm_bindgen]
pub struct WasmIPv4Address {
    inner_ip: IPv4Address  // IPv4Addressインスタンスを明示的な名前で保持
}

#[wasm_bindgen]
impl WasmIPv4Address {
    /// 新しいIPv4アドレスインスタンスを作成
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let ipv4 = new WasmIPv4Address();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // 内部のIPv4Addressインスタンスを作成して保持
        WasmIPv4Address {
            inner_ip: IPv4Address::new()
        }
    }

    /// 文字列からIPv4アドレスを生成
    /// 
    /// ###引数
    /// * `ip_str` - "192.168.1.1" 形式のIPv4アドレス文字列
    /// 
    /// ### 戻り値
    /// * `Result<WasmIPv4Address, JsValue>` - 成功時はWasmIPv4Address、失敗時はエラーメッセージ
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let ipv4 = WasmIPv4Address.from_string("192.168.1.1");
    /// ```
    #[wasm_bindgen]
    pub fn from_string(ip_str: &str) -> Result<WasmIPv4Address, JsValue> {
        // 文字列をIPv4Addressに変換を試みる
        match IPv4Address::from_string(ip_str) {
            // 変換成功時は新しいWasmIPv4Addressインスタンスを作成
            Ok(ip_address) => Ok(WasmIPv4Address {
                inner_ip: ip_address
            }),
            // 変換失敗時はエラーメッセージをJavaScript用の値に変換
            Err(error_message) => Err(JsValue::from_str(error_message))
        }
    }

    /// IPv4アドレスを文字列形式で取得
    /// 
    /// ### 戻り値
    /// * `String` - "192.168.1.1" 形式のIPv4アドレス文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のIPv4Addressインスタンスの文字列表現を取得
        self.inner_ip.to_string()
    }

    /// IPv4アドレスをバイト配列として取得
    /// 
    /// ### 戻り値
    /// * `Uint8Array` - 4バイトのIPv4アドレスデータ
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Uint8Array {
        // 内部のIPv4Addressインスタンスからバイト配列を取得
        let ip_bytes = self.inner_ip.to_array();
        // バイト配列をJavaScript用のUint8Arrayに変換
        Uint8Array::from(&ip_bytes[..])
    }
}

/// WebAssemblyからIPv6アドレスを扱うためのラッパー構造体
/// inner_ip: 内部に保持する実際のIPv6Addressインスタンス
#[wasm_bindgen]
pub struct WasmIPv6Address {
    inner_ip: IPv6Address  // IPv6Addressインスタンスを明示的な名前で保持
}

#[wasm_bindgen]
impl WasmIPv6Address {
    /// 新しいIPv6アドレスインスタンスを作成
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let ipv6 = new WasmIPv6Address();
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // 内部のIPv6Addressインスタンスを作成して保持
        WasmIPv6Address {
            inner_ip: IPv6Address::new()
        }
    }

    /// 文字列からIPv6アドレスを生成
    /// 
    /// ### 引数
    /// * `ip_str` - "2001:db8::1" 形式のIPv6アドレス文字列
    /// 
    /// ### 戻り値
    /// * `Result<WasmIPv6Address, JsValue>` - 成功時はWasmIPv6Address、失敗時はエラーメッセージ
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let ipv6 = WasmIPv6Address.from_string("2001:db8::1");
    /// ```
    #[wasm_bindgen]
    pub fn from_string(ip_str: &str) -> Result<WasmIPv6Address, JsValue> {
        // 文字列をIPv6Addressに変換を試みる
        match IPv6Address::from_string(ip_str) {
            // 変換成功時は新しいWasmIPv6Addressインスタンスを作成
            Ok(ip_address) => Ok(WasmIPv6Address {
                inner_ip: ip_address
            }),
            // 変換失敗時はエラーメッセージをJavaScript用の値に変換
            Err(error_message) => Err(JsValue::from_str(error_message))
        }
    }

    /// IPv6アドレスを文字列形式で取得
    /// 
    /// ### 戻り値
    /// * `String` - "2001:db8::1" 形式のIPv6アドレス文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のIPv6Addressインスタンスの文字列表現を取得
        self.inner_ip.to_string()
    }

    /// IPv6アドレスをバイト配列として取得
    /// 
    /// ### 戻り値
    /// * `Uint8Array` - 16バイトのIPv6アドレスデータ
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Uint8Array {
        // 内部のIPv6Addressインスタンスからバイト配列を取得
        let ip_bytes = self.inner_ip.to_array();
        // バイト配列をJavaScript用のUint8Arrayに変換
        Uint8Array::from(&ip_bytes[..])
    }
}

//////////////////////////////////////////////
// イーサネットフレームと物理層フレームのWebAssembly対応ラッパー構造体
//////////////////////////////////////////////

/// WebAssemblyからイーサネットフレームを扱うためのラッパー構造体
/// inner_frame: 内部に保持する実際のEthernetFrameインスタンス
#[wasm_bindgen]
pub struct WasmEthernetFrame {
    inner_frame: EthernetFrame
}

#[wasm_bindgen]
impl WasmEthernetFrame {
    /// 新しいイーサネットフレームを作成
    /// 
    /// ### 引数
    /// * `dst_mac` - 宛先MACアドレス
    /// * `src_mac` - 送信元MACアドレス
    /// * `ethertype` - イーサタイプ (例: 0x0800 for IPv4)
    /// * `data` - ペイロードデータのバイト配列
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let frame = new WasmEthernetFrame(dstMac, srcMac, 0x0800, new Uint8Array([...]));
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(dst_mac: &WasmMacAddress, src_mac: &WasmMacAddress, ethertype: u16, data: &[u8]) -> Self {
        // 新しいEthernetFrameインスタンスを作成
        WasmEthernetFrame {
            inner_frame: EthernetFrame::new(
                Some(dst_mac.inner_mac.clone()),  // 宛先MACアドレスをクローン
                Some(src_mac.inner_mac.clone()),  // 送信元MACアドレスをクローン
                Some(ethertype),                  // イーサタイプ
                Some(data.to_vec())               // データをベクターにコピー
            )
        }
    }
    /// イーサーネットフレームを文字列形式で取得
    /// 
    /// ### 戻り値
    /// * `String` - "#dst_mac,#src_mac,#type,#data" 形式の文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のEthernetFrameインスタンスの文字列表現を取得
        self.inner_frame.to_string().replace("\n","\r\n")
    }

    /// フレームの合計長を取得（バイト単位）
    /// 
    /// ### 戻り値
    /// * `usize` - イーサーネットフレームの総バイト長
    #[wasm_bindgen]
    pub fn total_length(&self) -> usize {
        // 内部のEthernetFrameインスタンスの長さを取得
        self.inner_frame.total_length()
    }

    /// イーサネットフレーム全体をバイト配列として取得
    /// 
    /// ### 戻り値
    /// * `Uint8Array` - フレーム全体のバイトデータ
    /// （宛先MAC + 送信元MAC + イーサタイプ + データ）
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Uint8Array {
        let mut bytes = Vec::new();
        
        // フレームの各フィールドをバイト配列に追加
        bytes.extend_from_slice(&self.inner_frame.dst_mac.to_array());  // 宛先MAC
        bytes.extend_from_slice(&self.inner_frame.src_mac.to_array());  // 送信元MAC
        bytes.extend_from_slice(&self.inner_frame.ethertype.to_be_bytes());  // イーサタイプ
        bytes.extend_from_slice(&self.inner_frame.data);  // ペイロードデータ
        
        // バイト配列をJavaScript用のUint8Arrayに変換
        Uint8Array::from(&bytes[..])
    }
}

/// WebAssemblyから物理層フレームを扱うためのラッパー構造体
/// inner_frame: 内部に保持する実際のPhysicalLayerFrameインスタンス
#[wasm_bindgen]
pub struct WasmPhysicalLayerFrame {
    inner_frame: PhysicalLayerFrame
}

#[wasm_bindgen]
impl WasmPhysicalLayerFrame {
    /// 新しい物理層フレームを作成
    /// 
    /// ### 引数
    /// * `ethernet_frame` - カプセル化するイーサネットフレーム
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let phyFrame = new WasmPhysicalLayerFrame(ethernetFrame);
    /// ```
    #[wasm_bindgen(constructor)]
    pub fn new(ethernet_frame: &WasmEthernetFrame) -> Self {
        // 新しいPhysicalLayerFrameインスタンスを作成
        WasmPhysicalLayerFrame {
            inner_frame: PhysicalLayerFrame::new(
                Some(ethernet_frame.inner_frame.clone())  // イーサネットフレームをクローン
            )
        }
    }

    /// 物理層フレームを文字列形式で取得
    /// 
    /// ### 戻り値
    /// * `String` - "#preamble,#sfd,#ethernet_frame" 形式の文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のEthernetFrameインスタンスの文字列表現を取得
        self.inner_frame.to_string().replace("\n","\r\n")
    }

    /// フレームの合計長を取得（バイト単位）
    /// プリアンブル、SFD、イーサネットフレーム、FCSを含む
    /// 
    /// ### 戻り値
    /// * `usize` - フレームの総バイト長
    #[wasm_bindgen]
    pub fn total_length(&self) -> usize {
        // 内部のPhysicalLayerFrameインスタンスの長さを取得
        self.inner_frame.total_length()
    }

    /// 物理層フレーム全体をバイト配列として取得
    /// 
    /// ### 戻り値
    /// * `Uint8Array` - フレーム全体のバイトデータ
    /// （プリアンブル + SFD + イーサネットフレーム + FCS）
    #[wasm_bindgen]
    pub fn to_bytes(&self) -> Uint8Array {
        // 内部のPhysicalLayerFrameインスタンスからバイト配列を取得
        let bytes = self.inner_frame.to_bytes();
        // バイト配列をJavaScript用のUint8Arrayに変換
        Uint8Array::from(&bytes[..])
    }
}

/// wasm-bindgenの初期化関数
/// このマクロは、WebAssemblyモジュールが読み込まれた時に自動的に実行される関数を指定します
#[wasm_bindgen(start)]
pub fn init() {
    // Rustのパニック時のエラーメッセージをブラウザのコンソールに表示するように設定
    // これにより、デバッグが容易になります
    console_error_panic_hook::set_once();
}

/// JS側にデバッグ表示のために用意された関数を呼び出す
#[wasm_bindgen]
extern "C" {
    pub fn showTerminal(s: &str);
}


//////////////////////////////////////////////
// イーサネットケーブルのWebAssembly対応ラッパー構造体
//////////////////////////////////////////////

/// WebAssemblyからイーサネットケーブルを扱うためのラッパー構造体
/// inner_cable: 内部に保持する実際のEthernetCableインスタンス
#[wasm_bindgen]
pub struct WasmEthernetCable {
    inner_cable: Option<EthernetCable>,
}

#[wasm_bindgen]
impl WasmEthernetCable {
    /// 新しいイーサネットケーブルを作成
    /// 
    /// ### 引数
    /// * `id` - ケーブルのId（タグのようなものケーブルを識別できるように。なくても良い）
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let cable = new WasmEthernetCable(id);
    /// ```
    /// 
    #[wasm_bindgen(constructor)]
    pub fn new(id:Option<String>) -> Self {
        // 新しいEthernetCableインスタンスを作成
        WasmEthernetCable {
            inner_cable: Some(EthernetCable::new(id))
        }
    }
    /// イーサーネットケーブルを削除する
    /// Noneを指定することで明示的に削除することになる
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// cable.remove();
    /// ```
    #[wasm_bindgen]
    pub fn remove(&mut self) {
        // Optionを使って、内部のケーブルリソースを明示的に破棄
        self.inner_cable = None;
    }
    /// イーサーネットケーブルが有効かどうかをチェック
    /// 
    /// ### 戻り値
    /// * `bool` - ケーブルが有効かどうか。
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// if(cable.is_valid()){...有効な時の処理...}else{...無効な時の処理...}
    /// ```
    /// 
    #[wasm_bindgen]
    pub fn is_valid(&self) -> bool {
        self.inner_cable.is_some()
    }
    /// そのイーサネットケーブルのIdを取得
    /// 
    /// ### 使用例（JavaScript）:
    /// ```javascript
    /// let id = cable.getId();
    /// ```
    /// 
    #[wasm_bindgen]
    pub fn get_id(&self) -> String {
         self.inner_cable.as_ref().map(|cable| cable.get_id()).unwrap_or_default()
    }

    /// イーサーネットケーブルの内容表示
    /// 
    /// ### 戻り値
    /// * `String` - ケーブルの情報を表す文字列
    #[wasm_bindgen]
    pub fn to_string(&self) -> String {
        // 内部のEthernetFrameインスタンスの文字列表現を取得
        self.inner_cable.as_ref().map(|cable| cable.to_string().replace("\n","\r\n")).unwrap_or_default()
    }
   
    /// イーサネットケーブルをつなげる
    /// 
    /// ### 引数
    /// * `ep1_connect_id` - 端1に繋げるコンポーネントのId
    /// * `ep2_connect_id` - 端2に繋げるコンポーネントのId
    ///
    #[wasm_bindgen]
    pub fn connect(&self, ep1_connect_id: Option<String>, ep2_connect_id: Option<String>) {
        self.inner_cable.as_ref().map(|cable| {
            cable.connect(ep1_connect_id, ep2_connect_id);
        }).unwrap_or_else( || showTerminal("このケーブルは無効です。"));
    }
    /// endpoint1の方にイーサネットケーブルをつなげる
    /// 
    /// ### 引数
    /// * `ep1_coonect_id` - 端1に繋げるコンポーネントId
    /// ケーブルの片方を別のポートに差し替えたり、別のコンポーネントに繋げ直すような時
    /// 
    #[wasm_bindgen]
    pub fn connect_endpoint1(&self, ep1_connect_id: Option<String>) {
        self.inner_cable.as_ref().map(|cable| {
            cable.connect_endpoint1(ep1_connect_id);
        }).unwrap_or_else( || showTerminal("このケーブルは無効です。"));
    }
    /// endpoint1に繋がっているコンポーネントのIdを取得
    /// 
    /// ### 戻り値
    /// * `Option<String>` - "endpoint1に繋がっているコンポーネントがあったらそのコンポーネントのIdが返される
    /// 
    #[wasm_bindgen]
    pub fn get_endpoint1_component_id(&self) -> Option<String> {
        //self.inner_cable.as_ref().?.get_endpoint1_component_id()
        self.inner_cable.as_ref().and_then(|cable| {
            cable.get_endpoint1_component_id()
        }).or_else(|| {
            showTerminal("このケーブルは無効です。");
            None
        })
    }

    /// endpoint2の方にイーサネットケーブルをつなげる
    /// 
    /// ### 引数
    /// * `ep2_coonect_id` - 端1に繋げるコンポーネントId
    /// ケーブルの片方を別のポートに差し替えたり、別のコンポーネントに繋げ直すような時
    /// 
    #[wasm_bindgen]
    pub fn connect_endpoint2(&self, ep2_connect_id: Option<String>) {
        self.inner_cable.as_ref().map(|cable| {
            cable.connect_endpoint2(ep2_connect_id);
        }).unwrap_or_else( || showTerminal("このケーブルは無効です。"));
    }
    /// endpoint2に繋がっているコンポーネントのIdを取得
    /// 
    /// ### 戻り値
    /// * `Option<String>` - "endpoint2に繋がっているコンポーネントがあったらそのコンポーネントのIdが返される
    /// 
    #[wasm_bindgen]
    pub fn get_endpoint2_component_id(&self) -> Option<String> {
        self.inner_cable.as_ref()?.get_endpoint2_component_id()
    }
    // /// いらなくなったケーブルを削除
    // /// 
    // #[wasm_bindgen]
    // pub fn drop(&mut self) {
    //     // Arc<Mutex>の参照カウントを減らし、リソースを解放するので、直接、inner_cable値を取得したいのでtake()している
    //     if let Some(cable) = self.inner_cable.take() {
    //         drop(cable); // 所有権を取得してリソースを解放
    //     }else{
    //         showTerminal("このケーブルは無効ですのでdropできません。")
    //     }
    // }
}

