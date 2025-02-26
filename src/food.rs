use std::fmt::Debug;

use derive_more::derive::Display;
use postgres_types::ToSql;

#[derive(Debug, Display)]
pub struct FoodCode(pub String);

#[derive(Debug, Display, ToSql)]
pub enum Unit {
    #[display("kcal")]
    Kcal,
    #[display("g")]
    Gram,
    #[display("mg")]
    Milligram,
}

#[derive(Debug, Display)]
#[display("{_0}{_1}")]
pub struct Quantity(pub f64, pub Unit);

#[derive(Debug)]
pub struct Food {
    pub code: FoodCode,
    pub name: String,
    pub energy: Quantity,
    pub fiber: Quantity,
    pub carbohydrates: Quantity,
    pub lipids: Quantity,
    pub protein: Quantity,
}
