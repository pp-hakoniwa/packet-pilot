use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IPv4Address(pub [u8; 4]);

impl fmt::Display for IPv4Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "#IPv4 address={}.{}.{}.{}",
            self.0[0], self.0[1], self.0[2], self.0[3]
        )
    }
}

impl IPv4Address {

    /// 192.168.0.xのIPv4アドレスをランダムに生成
    pub fn new() -> IPv4Address {
        let mut rng = rand::thread_rng();
        let mut addr = [0;4];

        addr = [192, 168, 0, rng.gen_range(1..=254)];
        IPv4Address(addr)
    }
    /// "."区切りの文字列からMACアドレスを生成する関数
    pub fn from_string(s: &str) -> Result<IPv4Address, &'static str> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 4 {
            return Err("Invalid IPv4 address format");
        }

        let mut addr = [0u8; 4];
        for i in 0..4 {
            addr[i] = match parts[i].parse::<u8>() {
                Ok(num) => num,
                Err(_) => return Err("Invalid number in IPv4 address"),
            };
        }

        Ok(IPv4Address(addr))
    }
    /// バイト配列からMACアドレスを生成する関数
    pub fn from_array(array: [u8; 4]) -> IPv4Address {
        IPv4Address(array)
    }
    /// IPv4アドレスをバイトスライスとして取得
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
    /// IPv4アドレスをバイト配列として取得
    pub fn to_array(&self) -> [u8; 4] {
        self.0
    }

}