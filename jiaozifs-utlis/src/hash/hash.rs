use hex::{FromHex, ToHex};

use serde::de::Deserializer;
use serde::ser::Serializer;
use serde::{de::{self, Visitor}, Deserialize, Serialize};
use sqlx::{Database, Decode, Encode, Postgres, Type};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgTypeInfo;

#[derive(Debug, PartialEq, Eq, Clone,Hash,Default)]
pub struct Hash(pub Vec<u8>);

impl <'q>Encode<'_, Postgres> for Hash {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer<'q>) -> Result<IsNull, BoxDynError> {
        buf.extend_from_slice(&self.0);
        Ok(IsNull::No)
    }
}
impl <'r>Decode<'r, Postgres> for Hash {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        
        value.as_bytes().map(|bytes| Self(bytes.to_vec()))
        .map_err(|e| e.into())
    }
}
impl Type<Postgres> for Hash {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        PgTypeInfo::with_name("BYTEA")
    }
}

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
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.clone()
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
impl From<Vec<u8>> for Hash{
    fn from(value: Vec<u8>) -> Self {
        Self{
            0: value,
        }
    }
}
impl From<Hash> for Vec<u8> {
    fn from(value: Hash) -> Self {
        value.0
    }
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