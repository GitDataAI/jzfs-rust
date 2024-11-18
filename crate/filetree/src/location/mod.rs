use lazy_static::lazy_static;

pub mod folder;
pub mod index;


lazy_static!{
    pub static ref DATAPATH: String = {
        let mut data = std::env::var("JZFS_HOME").unwrap_or_else(|_| "./jzfs".into());
        data.push_str("/data");
        data
    };
}