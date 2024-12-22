use std::{fmt::{self, Debug, Formatter}, sync::{Arc, Mutex}};
use rand::Rng;

use crate::{layer1::{packets::PhysicalLayerFrame, receive_callback::PhysicalLayerCallback}, showTerminal};

/// EthernetCableの本体
#[derive(Clone)]
pub struct EthernetCableState {
    pub id                     : String,
    pub endpoint1_component_id : Option<String>,
    pub endpoint1_callback     : Option<PhysicalLayerCallback>,
    pub endpoint2_component_id : Option<String>,
    pub endpoint2_callback     : Option<PhysicalLayerCallback>,
    pub connected              : bool,
}
/// Display
/// ```rust
/// let endpoint1_callback_ptr = self.endpoint1_callback
///     .as_ref()  // ここが重要！
///     .map(|cb| cb.as_ref() as *const dyn Fn(PhysicalLayerFrame));
/// ```
/// 
/// この行が何をしているか分解していきます。
/// 1. `.as_ref()`
///    - `Option<T>`型から`Option<&T>`に変換します
///    - つまり、`Option<Arc<dyn Fn(PhysicalLayerFrame) + Send + Sync>>`から`Option<&Arc<dyn Fn(PhysicalLayerFrame) + Send + Sync>>`に変換
///    - これにより、元のデータを借用（借用参照）できます
///    - 所有権を移動させずに、データへの参照を取得できるということになります
/// 
/// 2. `.map()`の役割
///    - `Option`型の値を安全に変換する高階関数です
///    - `None`の場合はそのまま`None`を返します
///    - `Some`の場合は、指定したクロージャで値を変換します
/// 
/// 3. `cb.as_ref()`
///    - `Arc`から参照を取得します
///    - これにより、`Arc`の中身を参照できます
/// 
/// 4. `as *const dyn Fn(PhysicalLayerFrame)`
///    - 関数ポインタに型変換します
///    - 動的ディスパッチ（トレイトオブジェクト）のポインタを取得
/// 
/// 具体的な変換の流れを想像してみましょう：
/// 
/// ```
/// Option<Arc<dyn Fn(...)>>
/// ↓ as_ref()
/// Option<&Arc<dyn Fn(...)>>
/// ↓ map() + as_ref() + as *const
/// Option<*const dyn Fn(...)>
/// ```
/// 
/// なぜこのアプローチが必要なのでしょうか？
/// 
/// - `Option`型のままポインタを直接フォーマットできない
/// - 安全に`None`と`Some`の両方のケースを処理したい
/// - 型変換を段階的に行うことで、コンパイラのエラーを回避できる
/// 
/// 補足として、このコードは以下のことを実現しています：
/// - コールバックが存在する場合、そのメモリアドレスを表示
/// - コールバックがない場合、"None"と表示
/// - 型変換を安全かつ明示的に行う
/// 
/// メモリとポインタの扱いは複雑ですが、Rustの型システムと所有権規則により、非常に安全に実装できます。

impl fmt::Display for EthernetCableState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 各コールバックのポインタアドレスを取得
        let endpoint1_callback_ptr = self.endpoint1_callback
            .as_ref()
            .map(|cb| cb.as_ref() as *const dyn Fn(PhysicalLayerFrame));
        
        let endpoint2_callback_ptr = self.endpoint2_callback
            .as_ref()
            .map(|cb| cb.as_ref() as *const dyn Fn(PhysicalLayerFrame));

        write!(
            f,
            "###Ethernet Cable= \n\
            #id                     : {}\n\
            #endpoint1_component_id : {:?}\n\
            #endpoint1_callback     : {}\n\
            #endpoint2_component_id : {:?}\n\
            #endpoint2_callback     : {}\n\
            #connected              : {}\n",
            self.id,
            self.endpoint1_component_id,
            endpoint1_callback_ptr
                .map(|ptr| format!("{:p}", ptr))
                .unwrap_or_else(|| "None".to_string()),
            self.endpoint2_component_id,
            endpoint2_callback_ptr
                .map(|ptr| format!("{:p}", ptr))
                .unwrap_or_else(|| "None".to_string()),
            self.connected,
        )
    }
}


impl EthernetCableState {
    fn new(id:Option<String>) -> Self {
        let cable_id = id.unwrap_or_else(|| format!("cable-{}",rand::thread_rng().gen_range(9..9999)));

        EthernetCableState { 
            id                     : cable_id,
            endpoint1_component_id : None,
            endpoint1_callback     : None,
            endpoint2_component_id : None,
            endpoint2_callback     : None,
            connected              : false,
        }
    }
}

#[derive(Clone)]
pub struct EthernetCable {
    state : Arc<Mutex<EthernetCableState>>,
}

impl fmt::Display for EthernetCable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // ミューテックスをロックして状態にアクセス
        let state = self.state.lock().unwrap();
        
        // EthernetCableStateのDisplayの実装を再利用
        write!(f, "{}", *state)
    }
}

impl EthernetCable {
    /// 新規にケーブルを配置したとき
    pub fn new(id:Option<String>) -> Self {
        debug("EthernetCable::new([id]) called.");
        EthernetCable{
            state: Arc::new(Mutex::new(EthernetCableState::new(id))),
        }
    }
    /// そのケーブルのIdを取得
    pub fn get_id(&self) -> String {
        let state = self.state.lock().unwrap();
        state.id.clone()
    }

    /// ケーブルの接続どちらかの端がまずどちらかに繋がるのでOptionにしてコンポーネントのIdを渡す
    pub fn connect(&self, ep1_connect_id: Option<String>, ep2_connect_id: Option<String>) {
        debug("EthernetCable::connect() called.");
        let mut state = self.state.lock().unwrap();
        
        state.endpoint1_component_id = ep1_connect_id;

        state.endpoint2_component_id = ep2_connect_id;

        if state.endpoint1_component_id.is_some() && state.endpoint2_component_id.is_some() {
            state.connected = true;
        }
    }

    pub fn connect_endpoint1(&self, ep1_connect_id: Option<String>) {
        debug("EthernetCable::connect_endpoint1() called.");
        let mut state = self.state.lock().unwrap();
        state.endpoint1_component_id = ep1_connect_id;

        if state.endpoint1_component_id.is_some() && state.endpoint2_component_id.is_some() {
            debug(&format!("EthernetCable({})::bothe connected.",state.id));
            state.connected = true;
        }
    }
    pub fn get_endpoint1_component_id(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.endpoint1_component_id.clone()
    }
    pub fn connect_endpoint2(&self, ep2_connect_id: Option<String>) {
        debug("EthernetCable::connect_endpoint2() called.");
        let mut state = self.state.lock().unwrap();
        state.endpoint2_component_id = ep2_connect_id;

        if state.endpoint1_component_id.is_some() && state.endpoint2_component_id.is_some() {
            debug(&format!("EthernetCable({})::bothe connected.",state.id));
            state.connected = true;
        }
    }
    pub fn get_endpoint2_component_id(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.endpoint2_component_id.clone()
    }

    /// ケーブル接続時に、データきたらここに渡してねというcallbackをsetする
    /// これをケーブルに伝えておくことで、データきた時イーサーネットケーブルは指定されている
    /// PhysicalLayerCallbackを呼び出す
    pub fn set_callback(&self, id:String,callback:PhysicalLayerCallback){
        debug("EthernetCable::set_callback() called.");
        let mut state = self.state.lock().unwrap();
        

        if state.endpoint1_component_id.is_none() && state.endpoint2_component_id.is_none() {
            //両方ともまだつながっていないのでセットできません
            return;
        }

        if let Some(ep1_id) = &state.endpoint1_component_id{
            if *ep1_id == id{
                debug("EthernetCable::set_callback() set endpoint1 callback.");
                state.endpoint1_callback = Some(callback.clone());
            }
        }
        if let Some(ep2_id) = &state.endpoint2_component_id{
            if *ep2_id == id{
                debug("EthernetCable::set_callback() set endpoint2 callback.");
                state.endpoint2_callback = Some(callback.clone());
            }
        }
    }

    /// データを送信する。上位層から呼ばれる関数。このケーブルにPacketを流したい上位層のコンポーネントから
    /// この関数を呼び出すことで、 ケーブルの先に電気信号を流す
    pub fn transmit_signal(&self, from_id:String, frame: PhysicalLayerFrame) {
        debug("EthernetCable::transmit_signal() called.");
        debug(&format!("EthernetCable::transmit_signal() frame={:?}",frame));

        let state = self.state.lock().unwrap();
        // 両端がつながっていなかったら終了
        if !state.connected && !state.endpoint1_callback.is_none() && !state.endpoint2_callback.is_none(){
            debug(("both endpoint not connected."));
            return;
        }
        // 送られるデータはどちらのendpointから来たか探す
        let ep1 = state.endpoint1_component_id.clone().unwrap();
        let ep2 = state.endpoint2_component_id.clone().unwrap();
        debug(&format!("EthernetCable::transmit_signal() from_id={:?}",from_id));
        debug(&format!("EthernetCable::transmit_signal() ep1={:?}",ep1));
        debug(&format!("EthernetCable::transmit_signal() ep2={:?}",ep2));
        
        let other_endpoint = if from_id == ep1 {
            debug("from ep1 --> callback to ep2");
            state.endpoint2_callback.clone().unwrap()
        } else if from_id == ep2 {
            debug("from ep2 --> callback to ep1");
            state.endpoint1_callback.clone().unwrap()
        } else {
            // エラーハンドリング: どちらのエンドポイントにも一致しない場合
            debug("Unexpected endpoint ID");
            return;
        };
        // 送り先のデバイスのCallBackを呼び出し信号を送る
        other_endpoint(frame);

    }
}

// -- for WASM debug
pub fn debug(s: &str) {
    let message= format!("\r\n------------\r\n[Debug] {}\r\n------------\r\n",s);
    showTerminal(&message);
}