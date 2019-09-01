use std::collections::HashMap;

pub trait Record {
    fn id(&self) -> u64;

    fn value_for(&self, name: &str) -> Option<&String>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct MapRecord {
    id: u64,
    map: HashMap<String, String>,
}

impl MapRecord {
    pub fn new(id: u64, map: HashMap<String, String>) -> Self {
        MapRecord { id, map }
    }
}

impl Record for MapRecord {
    fn id(&self) -> u64 {
        self.id
    }

    fn value_for(&self, name: &str) -> Option<&String> {
        self.map.get(name)
    }
}
