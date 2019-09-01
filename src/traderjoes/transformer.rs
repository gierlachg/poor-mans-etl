use std::str::FromStr;

use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use crate::order::{Order, Quantity};
use crate::record::Record;
use crate::transformer::{DiscardedRecord, Transformer};

const ORDER_NUMBER: &str = "Order Number";
const YEAR: &str = "Year";
const MONTH: &str = "Month";
const DAY: &str = "Day";
const PRODUCT_NUMBER: &str = "Product Number";
const PRODUCT_NAME: &str = "Product Name";
const COUNT: &str = "Count";

const INVALID_ORDER_NUMBER: &str = "Invalid order number";
const INVALID_DATE: &str = "Invalid date.";
const INVALID_PRODUCT_NUMBER: &str = "Invalid product number.";
const INVALID_PRODUCT_NAME: &str = "Invalid product name.";
const INVALID_COUNT: &str = "Invalid count.";

pub struct TraderJoesTransformer {}

impl TraderJoesTransformer {
    pub fn new() -> Self {
        TraderJoesTransformer {}
    }
}

impl<R: Record> Transformer<R> for TraderJoesTransformer {
    fn transform(&self, mut record: R) -> Result<Order, DiscardedRecord> {
        let order_number = record.value_for(ORDER_NUMBER)
            .and_then(|value| value.parse::<u64>().ok())
            .filter(|value| value > &0);
        if order_number.is_none() {
            return Err(DiscardedRecord::new(record.id(), INVALID_ORDER_NUMBER.to_string()));
        }

        let date = parse_date(&mut record);
        if date.is_none() {
            return Err(DiscardedRecord::new(record.id(), INVALID_DATE.to_string()));
        }

        let product_number = record.value_for(PRODUCT_NUMBER)
            .filter(|value| value.chars().all(|x| x.is_alphanumeric()));
        if product_number.is_none() {
            return Err(DiscardedRecord::new(record.id(), INVALID_PRODUCT_NUMBER.to_string()));
        }

        let product_name = record.value_for(PRODUCT_NAME)
            .filter(|value| value.chars().all(|x| x.is_alphabetic()));
        if product_name.is_none() {
            return Err(DiscardedRecord::new(record.id(), INVALID_PRODUCT_NAME.to_string()));
        }

        let count = record.value_for(COUNT)
            .and_then(|value| Decimal::from_str(&value).ok())
            .filter(|value| value > &Decimal::zero());
        if count.is_none() {
            return Err(DiscardedRecord::new(record.id(), INVALID_COUNT.to_string()));
        }

        let order = Order::builder()
            .with_id(order_number.unwrap())
            .with_date(date.unwrap())
            .with_product_id(product_number.unwrap().to_owned())
            .with_product_name(product_name.unwrap().to_owned())
            .with_quantity(Quantity::builder()
                .with_quantity(count.unwrap())
                .build())
            .build();

        Ok(order)
    }
}

pub fn parse_date<R: Record>(record: &mut R) -> Option<NaiveDate> {
    let year = record.value_for(YEAR)?;
    let month = record.value_for(MONTH)?;
    let day = record.value_for(DAY)?;
    NaiveDate::parse_from_str(&format!("{}-{}-{}", year, month, day), "%Y-%m-%d").ok()
}

#[cfg(test)]
mod tests {
    use crate::record::MapRecord;

    use super::*;

    #[test]
    fn order_number_should_be_positive() {
        let map = vec![
            (ORDER_NUMBER.to_string(), "0".to_string()),
        ].into_iter().collect();
        let record = MapRecord::new(1, map);
        let transformer = TraderJoesTransformer::new();

        let result = transformer.transform(record);

        assert_eq!(result.err().unwrap().error_message(), INVALID_ORDER_NUMBER);
    }

    #[test]
    fn count_should_be_positive() {
        let map = vec![
            (ORDER_NUMBER.to_string(), "1".to_string()),
            (YEAR.to_string(), "2019".to_string()),
            (MONTH.to_string(), "8".to_string()),
            (DAY.to_string(), "24".to_string()),
            (PRODUCT_NUMBER.to_string(), "12345".to_string()),
            (PRODUCT_NAME.to_string(), "Jam".to_string()),
            (COUNT.to_string(), "0".to_string()),
        ].into_iter().collect();
        let record = MapRecord::new(1, map);
        let transformer = TraderJoesTransformer::new();

        let result = transformer.transform(record);

        assert_eq!(result.err().unwrap().error_message(), INVALID_COUNT);
    }

    #[test]
    fn should_transform() {
        let map = vec![
            (ORDER_NUMBER.to_string(), "1".to_string()),
            (YEAR.to_string(), "2019".to_string()),
            (MONTH.to_string(), "8".to_string()),
            (DAY.to_string(), "24".to_string()),
            (PRODUCT_NUMBER.to_string(), "12345".to_string()),
            (PRODUCT_NAME.to_string(), "Jam".to_string()),
            (COUNT.to_string(), "100.20".to_string()),
        ].into_iter().collect();
        let record = MapRecord::new(1, map);
        let transformer = TraderJoesTransformer::new();

        let result = transformer.transform(record);

        assert_eq!(result.ok().unwrap(), Order::builder()
            .with_id(1)
            .with_date(NaiveDate::from_ymd(2019, 8, 24))
            .with_product_id("12345".to_string())
            .with_product_name("Jam".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(10020, 2)).build())
            .build());
    }
}