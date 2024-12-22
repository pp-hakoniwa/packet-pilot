use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IPv6Address(pub [u8; 16]);

impl fmt::Display for IPv6Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#IPv6 address={:02X}{:02X}:{:02X}{:02X}:{:02X}{:02X}:{:02X}{:02X}:\
             {:02X}{:02X}:{:02X}{:02X}:{:02X}{:02X}:{:02X}{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3],
            self.0[4], self.0[5], self.0[6], self.0[7],
            self.0[8], self.0[9], self.0[10], self.0[11],
            self.0[12], self.0[13], self.0[14], self.0[15],
        )
    }
}

impl IPv6Address {
    /// "2001:0db8:xx:xx:xx:xx:xx:xx"のIPv6アドレスをランダムに生成
    pub fn new() -> IPv6Address {
        let mut rng = rand::thread_rng();
        let mut address = [0u8; 16];
        address[0] = 0x20;
        address[1] = 0x01;
        address[2] = 0x0d;
        address[3] = 0xb8;
        
        for i in 4..16 {
            address[i] = rng.gen();
        }
        
        IPv6Address(address)
    }

    /// ":"区切りの文字列からIPv6アドレスを生成する関数
    pub fn from_string(s: &str) -> Result<IPv6Address, &'static str> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 8 {
            return Err("Invalid IPv6 address format");
        }

        let mut addr = [0u8; 16];
        for (i, part) in parts.iter().enumerate() {
            if part.len() > 4 {
                return Err("Invalid segment in IPv6 address");
            }
            let value = match u16::from_str_radix(part, 16) {
                Ok(num) => num,
                Err(_) => return Err("Invalid number in IPv6 address"),
            };
            addr[i * 2] = (value >> 8) as u8; // 高位バイト
            addr[i * 2 + 1] = (value & 0xFF) as u8; // 低位バイト
        }

        Ok(IPv6Address(addr))
    }

    /// バイト配列からIPv6アドレスを生成する関数
    pub fn from_array(array: [u8; 16]) -> IPv6Address {
        IPv6Address(array)
    }

    /// IPv6アドレスをバイトスライスとして取得
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// IPv6アドレスをバイト配列として取得
    pub fn to_array(&self) -> [u8; 16] {
        self.0
    }

     
    /// セパレータを指定してIPv6アドレスを文字列に変換
    pub fn to_string_with_separator(&self, separator: char) -> String {
        format!(
            "{:02X}{:02X}{}{:02X}{:02X}{}{:02X}{:02X}{}{:02X}{:02X}{}\
             {:02X}{:02X}{}{:02X}{:02X}{}{:02X}{:02X}{}{:02X}{:02X}",
            self.0[0], self.0[1], separator,
            self.0[2], self.0[3], separator,
            self.0[4], self.0[5], separator,
            self.0[6], self.0[7], separator,
            self.0[8], self.0[9], separator,
            self.0[10], self.0[11], separator,
            self.0[12], self.0[13], separator,
            self.0[14], self.0[15]
        )
    }

}
