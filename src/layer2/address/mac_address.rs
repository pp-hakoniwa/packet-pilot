use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MacAddress(pub [u8; 6]);

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#MAC ADDRESS={:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl MacAddress {
    /// ランダムにMACアドレスを生成するs
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut addr = [0u8; 6];
        rng.fill(&mut addr);
        
        // MACアドレス生成ルールのU/Lビットの扱い
        // ローカル管理アドレス (LAA) を示すためにU/Lビットを1にしておく
        addr[0] |= 0x02;  // ローカル管理アドレス 00000010にしておく
        //addr[0] &= 0xFE; // グローバルユニキャストの場合は、00000000

        MacAddress(addr)
    }
    /// ":"区切りの文字列からMACアドレスを生成する関数
    pub fn from_string(mac_str: &str) -> Result<MacAddress, &'static str> {
        let bytes: Vec<u8> = mac_str.split(':')
                                    .map(|s| u8::from_str_radix(s, 16))
                                    .collect::<Result<Vec<u8>, _>>()
                                    .map_err(|_| "Invalid MAC address format")?;
    
        if bytes.len() == 6 {
            let mut mac_array = [0u8; 6];
            mac_array.copy_from_slice(&bytes);
            Ok(MacAddress(mac_array))
        } else {
            Err("MAC address must contain exactly 6 bytes")
        }
    }
    /// バイト配列からMACアドレスを生成する関数
    pub fn from_array(bytes: [u8; 6]) -> Self {
        MacAddress(bytes)
    }

    /// MACアドレスをバイトスライスとして取得
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
    /// MACアドレスをバイト配列として取得
    pub fn to_array(&self) -> [u8; 6] {
        self.0
    }

    /// broadcast用のMAC Addressを取得する関数
    pub fn get_broadcast_mac_addr() -> MacAddress {
        // IPv4では全てFFにすることでブロードキャストアドレスになる
        let mac:MacAddress = MacAddress([0xFF;6]);
        mac
    }

    /// ARP時の宛先MAC Addressを取得する関数
    pub fn get_arp_target_mac_addr() -> MacAddress {
        // ARPコマンドを送る時targetのMACアドレスは全て00にする
        let mac:MacAddress = MacAddress([0x00;6]);
        mac
    }
    

}
