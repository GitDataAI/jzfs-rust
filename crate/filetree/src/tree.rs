use crate::node::NodeTree;
use std::path::PathBuf;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone,PartialEq,Eq)]
pub struct FileTree{
    pub root: PathBuf,
    pub hash: String,
    pub branches: String,
    pub children: Vec<Box<NodeTree>>,
}
impl FileTree{
    pub fn open(root: PathBuf,hash: String, branches: String,children: Vec<Box<NodeTree>>) -> FileTree {
        Self{
            root,
            hash,
            branches,
            children,
        }
    }
    pub fn get_root(&self) -> PathBuf {
        self.root.clone()
    }
    pub fn get_hash(&self) -> String {
        self.hash.clone()
    }
    pub fn get_branches(&self) -> String {
        self.branches.clone()
    }
    pub fn add_child(&mut self, node: Box<NodeTree>){
        let mut node = node.clone();
        let path = self.root.join(&node.hash);
        node.local = path;
        self.children.push(node);
    }
}
