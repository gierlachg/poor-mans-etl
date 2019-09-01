use crate::order::Order;
use crate::record::Record;

pub trait Transformer<R: Record> {
    fn transform(&self, record: R) -> Result<Order, DiscardedRecord>;
}

#[derive(Debug)]
pub struct DiscardedRecord {
    id: u64,
    error_message: String,
}

impl DiscardedRecord {
    pub fn new(id: u64, error_message: String) -> DiscardedRecord {
        DiscardedRecord { id, error_message }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn error_message(&self) -> &str {
        &self.error_message
    }
}
