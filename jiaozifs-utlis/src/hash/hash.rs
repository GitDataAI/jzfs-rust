use std::str::FromStr;
use serde::{Serialize, Deserialize, de::{self, Visitor}};
use serde::ser::Serializer;
use serde::de::Deserializer;
use hex::{FromHex, ToHex};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Hash(Vec<u8>);

impl Hash {
    pub fn from_hex(hex_str: &str) -> Result<Self, hex::FromHexError> {
        if hex_str.is_empty() {
            return Ok(Self::default());
        }
        let bytes = Vec::from_hex(hex_str)?;
        Ok(Self(bytes))
    }

    pub fn to_hex(&self) -> String {
        if self.0.is_empty() {
            return "".to_string();
        }
        self.0.encode_hex::<String>()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_hex())
    }
}

impl<'de> Deserialize<'de> for Hash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct HashVisitor;

        impl<'de> Visitor<'de> for HashVisitor {
            type Value = Hash;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a hex string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Hash::from_hex(value).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(HashVisitor)
    }
}

pub fn hashes_of_hex_array(hashes_str: &[&str]) -> Result<Vec<Hash>, hex::FromHexError> {
    hashes_str.iter().map(|&s| Hash::from_hex(s)).collect()
}

pub fn hex_array_of_hashes(hashes: &[Hash]) -> Vec<String> {
    hashes.iter().map(|h| h.to_hex()).collect()
}

#[cfg(test)]
mod hash_test{
    use super::*;
    #[test]
    fn test_hash_from_hex(){
        let hex_str = "1234567890abcdef";
        let hash = Hash::from_hex(hex_str).unwrap();
        assert_eq!(hash.to_hex(), hex_str);
    }
    #[test]
    fn hex_array_of_hashes_test(){
        let hashes = vec![Hash::from_hex("1234567890abcdef").unwrap(), Hash::from_hex("abcdef1234567890").unwrap()];
        let hex_array = hex_array_of_hashes(&hashes);
        assert_eq!(hex_array, vec!["1234567890abcdef", "abcdef1234567890"]);
    }
}