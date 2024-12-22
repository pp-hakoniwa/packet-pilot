use serde::{Deserialize, Serialize};
use std::fmt;
use crate::layer2::packets::EthernetFrame;


#[derive(Clone, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PhysicalLayerFrame {
    pub preamble: [u8; 7],             // プリアンブル (7バイト)
    pub sfd: u8,                       // スタートフレームデリミタ (1バイト)
    pub ethernet_frame: EthernetFrame, // データリンク層のイーサネットフレーム
}

impl fmt::Display for PhysicalLayerFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#preamble       : {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}\n\
             #sfd            : {:02X}\n\
             #ethernet_frame : \n{}\n",
            self.preamble[0],
            self.preamble[1],
            self.preamble[2],
            self.preamble[3],
            self.preamble[4],
            self.preamble[5],
            self.preamble[6],
            self.sfd,
            self.ethernet_frame,
        )
    }
}

impl PhysicalLayerFrame {
    /// 新しいフレームを生成
    pub fn new(frame: Option<EthernetFrame>) -> Self {
        Self {
            preamble: [0xAA; 7],
            sfd: 0xAB,
            ethernet_frame: frame.unwrap_or_else(EthernetFrame::default),
        }
    }

    /// RAWデータからPhysicalLayerFrameを構築
    pub fn from_raw(
        preamble: [u8; 7],
        sfd: u8,
        ethernet_frame: EthernetFrame,
    ) -> Self {
        Self {
            preamble,
            sfd,
            ethernet_frame,
        }
    }

    /// フレーム全体のバイト長を計算する
    pub fn total_length(&self) -> usize {
        8 + self.ethernet_frame.total_length() // プリアンブル + SFD + イーサネットフレーム長
    }

    /// バイト配列に変換
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.preamble);
        bytes.push(self.sfd);
        bytes.extend_from_slice(&self.ethernet_frame.dst_mac.to_array());
        bytes.extend_from_slice(&self.ethernet_frame.src_mac.to_array());
        bytes.extend_from_slice(&self.ethernet_frame.ethertype.to_be_bytes());
        bytes.extend_from_slice(&self.ethernet_frame.data);
        bytes
    }
}
