use std::collections::HashMap;

use crate::mount::git::NodeStorage;

pub mod git;

#[derive(Clone)]
pub struct StoragePool {
    pub(crate) node : HashMap<String, NodeStorage>,
}

impl Default for StoragePool {
    fn default() -> Self {
        Self::new()
    }
}

impl StoragePool {
    pub fn new() -> Self {
        StoragePool {
            node : HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name : String, storage : NodeStorage) {
        self.node.insert(name, storage);
    }
}
