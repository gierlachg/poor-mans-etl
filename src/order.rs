use chrono::NaiveDate;
use inflections::case::to_title_case;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use crate::order::Unit::KG;

#[derive(Debug, Eq, PartialEq)]
pub struct Order {
    id: u64,
    date: NaiveDate,
    product_id: String,
    product_name: String,
    quantity: Quantity,
}

impl Order {
    pub fn builder() -> OrderBuilder {
        OrderBuilder {
            id: None,
            date: None,
            product_id: None,
            product_name: None,
            quantity: None,
        }
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn product_id(&self) -> &str {
        &self.product_id
    }

    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &Quantity {
        &self.quantity
    }
}

pub struct OrderBuilder {
    id: Option<u64>,
    date: Option<NaiveDate>,
    product_id: Option<String>,
    product_name: Option<String>,
    quantity: Option<Quantity>,
}

impl OrderBuilder {
    pub fn with_id(mut self, id: u64) -> Self {
        assert!(id > 0, "id should be > 0");
        self.id = Some(id);
        self
    }

    pub fn with_date(mut self, date: NaiveDate) -> Self {
        self.date = Some(date);
        self
    }

    pub fn with_product_id(mut self, product_id: String) -> Self {
        self.product_id = Some(product_id);
        self
    }

    pub fn with_product_name(mut self, product_name: String) -> Self {
        self.product_name = Some(product_name);
        self
    }

    pub fn with_quantity(mut self, quantity: Quantity) -> Self {
        self.quantity = Some(quantity);
        self
    }

    pub fn build(self) -> Order {
        Order {
            id: self.id.expect("missing id"),
            date: self.date.expect("missing date"),
            product_id: self.product_id.expect("missing product id"),
            product_name: to_title_case(&self.product_name.expect("missing product name")),
            quantity: self.quantity.expect("missing quantity"),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Quantity {
    quantity: Decimal,
    unit: Unit,
}

pub struct QuantityBuilder {
    quantity: Option<Decimal>,
    unit: Option<Unit>,
}

impl Quantity {
    pub fn builder() -> QuantityBuilder {
        QuantityBuilder {
            quantity: None,
            unit: Some(KG),
        }
    }

    pub fn quantity(&self) -> &Decimal {
        &self.quantity
    }

    pub fn unit(&self) -> &Unit {
        &self.unit
    }
}

impl QuantityBuilder {
    pub fn with_quantity(mut self, quantity: Decimal) -> Self {
        assert!(quantity > Decimal::zero(), "quantity should be > 0");
        self.quantity = Some(quantity);
        self
    }

    pub fn with_unit(mut self, unit: Unit) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn build(self) -> Quantity {
        Quantity {
            quantity: self.quantity.expect("missing quantity"),
            unit: self.unit.expect("missing unit"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Unit {
    KG,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn order_builder_requires_id() {
        Order::builder()
            .with_date(date())
            .with_product_id("product-id".to_string())
            .with_product_name("product-name".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();
    }

    #[test]
    #[should_panic]
    fn order_builder_requires_id_greater_than_0() {
        Order::builder()
            .with_id(0)
            .with_date(date())
            .with_product_id("product-id".to_string())
            .with_product_name("product-name".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();
    }

    #[test]
    #[should_panic]
    fn order_builder_requires_date() {
        Order::builder()
            .with_id(1)
            .with_product_id("product-id".to_string())
            .with_product_name("product-name".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();
    }

    #[test]
    #[should_panic]
    fn order_builder_requires_product_id() {
        Order::builder()
            .with_id(1)
            .with_date(date())
            .with_product_name("product-name".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();
    }

    #[test]
    #[should_panic]
    fn order_builder_requires_product_name() {
        Order::builder()
            .with_id(1)
            .with_date(date())
            .with_product_id("product-id".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();
    }

    #[test]
    #[should_panic]
    fn order_builder_requires_quantity() {
        Order::builder()
            .with_id(1)
            .with_date(date())
            .with_product_id("product-id".to_string())
            .with_product_name("product-name".to_string())
            .build();
    }

    #[test]
    #[should_panic]
    fn quantity_builder_requires_quantity() {
        Quantity::builder()
            .build();
    }

    #[test]
    #[should_panic]
    fn quantity_builder_requires_quantity_greater_than_0() {
        Quantity::builder()
            .with_quantity(Decimal::new(0, 0))
            .build();
    }

    #[test]
    fn should_title_case_product_name() {
        let order = Order::builder()
            .with_id(1)
            .with_date(date())
            .with_product_id("product-id".to_string())
            .with_product_name("product name".to_string())
            .with_quantity(Quantity::builder().with_quantity(Decimal::new(1, 0)).build())
            .build();

        assert_eq!(order.product_name, "Product Name");
    }

    fn date() -> NaiveDate {
        NaiveDate::from_ymd(2019, 8, 22)
    }
}
