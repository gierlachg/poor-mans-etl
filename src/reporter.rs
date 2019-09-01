use crate::loader::DiscardedOrder;
use crate::transformer::DiscardedRecord;

pub trait Reporter {
    fn report_record(&self, discarded_record: DiscardedRecord);

    fn report_order(&self, discarded_order: DiscardedOrder);
}