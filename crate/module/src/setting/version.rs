use jzfs_entity::dto::version::Version;
use jzfs_entity::time;
use jzfs_entity::time::format_description;
use crate::Module;

impl Module {
    pub fn version(&self) -> Version{
        let format = format_description::parse(
            "[year]-[month]-[day] [hour]:[minute]:[second] [offset_hour sign:mandatory]:[offset_minute]:[offset_second]",
        ).unwrap();
        let time = format!("{}",time::OffsetDateTime::now_utc().format(&format).unwrap());
        Version{
            version: "0.1.0".to_string(),
            os: std::env::consts::OS.to_string(),
            time,
        }
    }
}