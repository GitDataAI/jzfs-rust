mod hash;
mod hash_reader;

pub use hash::{
    Hash,
    hashes_of_hex_array,
    hex_array_of_hashes
};

pub use hash_reader::{
    Hasher,HashingReader,HashType
};