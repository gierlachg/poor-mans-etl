use std::error::Error;
use std::fs::File;

use csv::Writer;

use crate::loader::{DiscardedOrder, Loader};
use crate::order::Order;

const HEADERS: [&str; 6] = ["Order Id", "Date Time", "Product Id", "Product Name", "Quantity", "Unit"];

pub struct CsvLoader {
    writer: Writer<File>,
}

impl CsvLoader {
    pub fn to(file: File) -> Result<Self, Box<dyn Error>> {
        let mut writer = Writer::from_writer(file);
        writer.write_record(&HEADERS)?;
        Ok(CsvLoader { writer })
    }
}

impl Loader for CsvLoader {
    fn load(&mut self, order: Order) -> Result<(), DiscardedOrder> {
        if let Err(e) = self.writer.write_record(vec!(
            order.id().to_string(),
            order.date().to_string(),
            order.product_id().to_owned(),
            order.product_name().to_owned(),
            order.quantity().quantity().to_string(),
            format!("{:?}", order.quantity().unit()))
        ) {
            return Err(DiscardedOrder::new(order, e.description().to_string()));
        }
        self.writer.flush().or_else(|e| Err(DiscardedOrder::new(order, e.description().to_string())))
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Seek, SeekFrom};

    use chrono::NaiveDate;
    use rust_decimal::Decimal;

    use tempfile::tempfile;

    use crate::order::Quantity;

    use super::*;

    #[test]
    fn should_extract() {
        let order = Order::builder()
            .with_id(12)
            .with_date(NaiveDate::from_ymd(2019, 8, 27))
            .with_product_id("123456789".to_string())
            .with_product_name("Nuts".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1220, 2)).build())
            .build();

        let file = tempfile().unwrap();
        let mut cloned = file.try_clone().unwrap();
        let mut loader = CsvLoader::to(file).unwrap();
        loader.load(order).unwrap();

        let mut loaded_content = String::new();
        cloned.seek(SeekFrom::Start(0)).unwrap();
        cloned.read_to_string(&mut loaded_content).unwrap();
        assert_eq!(loaded_content, "Order Id,Date Time,Product Id,Product Name,Quantity,Unit\n\
                    12,2019-08-27,123456789,Nuts,12.20,KG\n");
    }
}