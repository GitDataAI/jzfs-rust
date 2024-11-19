use crate::location::index::FileIndex;
use crate::location::DATAPATH;
use rocksdb::{DBWithThreadMode, SingleThreaded, DB};
use std::io;
use std::io::ErrorKind;
use std::path::Path;
use uuid::Uuid;

pub struct LocalFileDB{
    uid: Uuid,
    db: Option<DBWithThreadMode<SingleThreaded>>
}

impl LocalFileDB {
    pub fn new(uid: Uuid) -> Self {
        LocalFileDB{uid, db: None }
    }
    pub fn open(&mut self) -> io::Result<()>{
        let binding = format!("{}/{}", DATAPATH.to_string(), self.uid);
        let path = Path::new(&binding);
        let db = DB::open_default(path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        self.db = Some(db);
        Ok(())
    }
    pub fn write(&mut self, index: FileIndex, data: Vec<u8>) -> io::Result<()>{
        if let Some(db) = self.db.as_mut() {
            db.put(index,data).unwrap();
            Ok(())
        }else {
            Err(io::Error::new(ErrorKind::Other,"IO Error"))
        }
    }
}