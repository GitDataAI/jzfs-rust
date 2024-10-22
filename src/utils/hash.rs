use std::fmt::{Display, Formatter};
use sea_orm::{DeriveValueType, FromJsonQueryResult};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::ErrorKind;


#[derive(Clone,Debug,PartialEq,Eq,Serialize,Deserialize,FromJsonQueryResult)]
pub struct HashVec(pub Vec<Hasher>);



#[derive(Clone,Debug,PartialEq,Eq,DeriveValueType,Serialize,Deserialize)]
pub struct Hasher(pub Vec<u8>);

impl Hasher {
    pub fn new() -> Self {
        Hasher(Vec::new())
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn hex(&self) -> String {
        hex::encode(&self.0)
    }
    pub fn unmarshal_json(&mut self, byte: Vec<u8>) -> io::Result<&mut Self>{
        if byte.len() < 2 {
            return Err(io::Error::new(ErrorKind::InvalidData, "hash json must be string"));
        }
        if byte[0] == '"' as u8 || byte[byte.len() -1] == '"' as u8{
            return Err(io::Error::new(ErrorKind::InvalidData, "hash json must be string"));
        }
        if byte.len() == 2{
            return Err(io::Error::new(ErrorKind::InvalidData, "Invalid hash hash"));
        }
        self.0 = byte.to_vec();
        Ok(self)
    }
    pub fn marshal_json(&mut self) -> Vec<u8> {
        if self.is_empty() {
            r#""""#.as_bytes().to_vec()
        }else {
            format!("\"{}\"", self.hex()).as_bytes().to_vec()
        }
    }
    pub fn from_hex(hex_str: &str) -> io::Result<Self> {
        let hex =match hex::decode(hex_str){
            Ok(ok) => ok,
            Err(e) =>return Err(io::Error::new(ErrorKind::InvalidData, e))
        };
        Ok(Self(hex))
    }

}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HasherBuilder {
    hasher: Hasher,
}
impl HasherBuilder {
    pub fn new() -> Self {
        Self{
            hasher: Hasher::default(),
        }
    }
    pub fn write_string(&mut self, s: &str) -> &mut Self {
        self.hasher.0.extend_from_slice(s.as_bytes());
        self
    }
    pub fn write_bytes(&mut self, s: &[u8]) -> &mut Self {
        self.hasher.0.extend_from_slice(s);
        self
    }
    pub fn write_u8(&mut self, i: u8) -> &mut Self {
        self.hasher.0.extend_from_slice(&i.to_le_bytes());
        self
    }
    pub fn write_u16(&mut self, i: u16) -> &mut Self {
        self.hasher.0.extend_from_slice(&i.to_le_bytes());
        self
    }
    pub fn write_u32(&mut self, i: u32) -> &mut Self {
        self.hasher.0.extend_from_slice(&i.to_le_bytes());
        self
    }
    pub fn write_u64(&mut self, i: u64) -> &mut Self {
        self.hasher.0.extend_from_slice(&i.to_le_bytes());
        self
    }
    pub fn write_u128(&mut self, i: u128) -> &mut Self {
        self.hasher.0.extend_from_slice(&i.to_le_bytes());
        self
    }
    pub fn build(self) -> Hasher {
        self.hasher
    }
}

impl Display for Hasher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.hex())
    }
}
