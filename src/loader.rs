use crate::order::Order;

pub trait Loader {
    fn load(&mut self, order: Order) -> Result<(), DiscardedOrder>;
}

#[derive(Debug)]
pub struct DiscardedOrder {
    order: Order,
    error_message: String,
}

impl DiscardedOrder {
    pub fn new(order: Order, error_message: String) -> DiscardedOrder {
        DiscardedOrder { order, error_message }
    }

    pub fn order(&self) -> &Order {
        &self.order
    }

    pub fn error_message(&self) -> &str {
        &self.error_message
    }
}