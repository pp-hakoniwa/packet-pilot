use serde::{Deserialize, Serialize};
use std::fmt;

use crate::layer2::address::mac_address::MacAddress;

#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EthernetFrame {
    pub dst_mac: MacAddress,  // 宛先MACアドレス (6バイト)
    pub src_mac: MacAddress,  // 送信元MACアドレス (6バイト)
    pub ethertype: u16,       // イーサータイプ (2バイト)
    pub data: Vec<u8>,        // データリンク層のペイロード
}

impl fmt::Display for EthernetFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex_bytes: Vec<String> = self
            .data
            .iter()
            .map(|byte| format!("{:02X}", byte))
            .collect();
        let formatted_data = hex_bytes.join(" ");
        write!(
            f,
            "#dst_mac     : {}\n\
             #src_mac     : {}\n\
             #ethertype   : {:04X}\n\
             #data        : [{}]\n",
            self.dst_mac,
            self.src_mac,
            self.ethertype,
            formatted_data,
        )
    }
}

impl EthernetFrame {
    pub fn new(
        dst_mac: Option<MacAddress>,
        src_mac: Option<MacAddress>,
        ethertype: Option<u16>,
        data: Option<Vec<u8>>,
    ) -> Self {
        Self {
            dst_mac: dst_mac.unwrap_or_else(|| MacAddress::get_broadcast_mac_addr()),
            src_mac: src_mac.unwrap_or_else(|| MacAddress::new()),
            ethertype: ethertype.unwrap_or(0x0800), // デフォルトはIPv4
            data: data.unwrap_or_default(),
        }
    }

    /// 指定したデータからフレームを生成する
    pub fn from_raw(
        dst_mac: [u8; 6],
        src_mac: [u8; 6],
        ethertype: u16,
        data: Vec<u8>,
    ) -> Self {
        Self {
            dst_mac: MacAddress(dst_mac),
            src_mac: MacAddress(src_mac),
            ethertype,
            data,
        }
    }
    /// フレーム全体のバイト長を計算する
    pub fn total_length(&self) -> usize {
        14 + self.data.len() // 14バイト(=dst_mac+src_mac+ethertype) + ペイロード長
    }

}
