use std::io;
use std::io::{Read, Write};
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct NodeTree{
    pub local: PathBuf,
    pub file_name: String,
    pub is_dir: bool,
    pub children: Vec<Box<NodeTree>>,
    pub hash: String,
}

impl NodeTree {
    pub fn create(local: PathBuf, name:String, hash: String,is_dir: bool) -> NodeTree {
        Self{
            local,
            file_name: name,
            children: vec![],
            hash,
            is_dir,
        }
    }
    pub fn save(&mut self, data: Vec<u8>) -> io::Result<()> {
        if std::fs::read_dir(self.local.as_path()).is_err() {
            std::fs::create_dir_all(self.local.as_path())?;
        };
        let mut fs = std::fs::File::options()
            .write(true)
            .create(true)
            .read(true)
            .open(format!("{}/{}_{}",self.local.as_path().display(),self.file_name,self.hash))?;
        fs.write_all(&data)?;
        Ok(())
    }
    pub fn read(&self) -> io::Result<Vec<u8>> {
        if !self.local.exists(){
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }
        let mut fs = std::fs::File::open(format!("{}/{}_{}",self.local.as_path().display(),self.file_name,self.hash))?;
        let mut buffer = Vec::new();
        fs.read_to_end(&mut buffer)?;
        Ok(buffer)
    }    
    pub fn delete(&mut self) -> io::Result<()> {
        let path = format!("{}/{}_{}",self.local.as_path().display(),self.file_name,self.hash);
        std::fs::remove_file(path)?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::io;
    use std::path::PathBuf;
    use crate::node::NodeTree;

    pub fn new() -> NodeTree {
        let path = PathBuf::from("/home/zhenyi/文档/jzfs-rs/test/data");
        let hash = "555".to_string();
        let name = String::from("test.txt");
        NodeTree{
            local: path,
            file_name: name,
            children: vec![],
            hash,
        }
    }
    #[test]
    pub fn a_save_test(){
        let mut node = new();
        let r = r#"
            Hello FileTree!
        "#;
        node.save(r.as_bytes().to_vec()).expect("Save File");
    }
    #[test]
    pub fn b_read_test() -> io::Result<()>{
        let node = new();
        let data = node.read()?;
        let r = r#"
            Hello FileTree!
        "#;
        assert_eq!(data, r.as_bytes());
        Ok(())
    }
    #[test]
    pub fn c_delete_test() -> io::Result<()>{
        let mut node = new();
        let data = node.delete();
        assert_eq!(data.is_ok(), true);
        assert_eq!(node.read().is_ok(), false);
        Ok(())
    }
}