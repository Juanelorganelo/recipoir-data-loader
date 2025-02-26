use crate::food::Food;
use anyhow::Result;

pub trait Source {
    fn load_foods(&self) -> Result<impl Iterator<Item = Result<Food>>>;
}
