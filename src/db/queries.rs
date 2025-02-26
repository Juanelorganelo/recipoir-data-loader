use anyhow::{Context, Error, Result};
use itertools::Itertools;

use crate::food::{Food, Unit};

fn format_food(f: &Food) -> String {
    format!(
        "({}\t{}\t{}\t{}\t{}\t{}\t{}\t)",
        f.code, f.name, f.energy, f.carbohydrates, f.lipids, f.protein, f.fiber
    )
}

pub fn insert_foods(client: &mut postgres::Client, foods: &Vec<Food>) -> Result<()> {
    let values = foods.iter().map(format_food).join(",\n");
    client
        .execute(
            "INSERT INTO foods (id, name, energy, carbohydrates, protein, lipids, fiber)
                VALUES $1",
            &[&values],
        )
        .context(format!("failed to insert food rows: {values}"))?;
    Ok(())
}

pub fn create_foods_table(client: &mut postgres::Client) -> Result<u64> {
    client
        .execute(
            "CREATE TABLE IF NOT EXISTS foods (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            energy DOUBLE PRECISION NOT NULL,
            fiber quantity NOT NULL,
            carbohydrates quantity NOT NULL,
            lipids quantity NOT NULL,
            protein quantity NOT NULL
        )",
            &[],
        )
        .context("unable to create table foods")
}

pub fn create_unit_type(client: &mut postgres::Client) -> Result<u64> {
    #[cfg(feature = "nuke")]
    client.execute("DROP TYPE IF EXISTS food_unit CASCADE", &[])?;
    client
        .execute(
            format!(
                "CREATE TYPE food_unit as ENUM ('{}', '{}', '{}')",
                Unit::Gram,
                Unit::Milligram,
                Unit::Kcal
            )
            .as_str(),
            &[],
        )
        .map_err(Error::new)
}

pub fn create_quantity_type(client: &mut postgres::Client) -> Result<u64> {
    #[cfg(feature = "nuke")]
    client.execute("DROP TYPE IF EXISTS quantity CASCADE", &[])?;
    client
        .execute(
            "CREATE TYPE quantity AS (
        value DOUBLE PRECISION,
        unit_of_measure food_unit
    )",
            &[],
        )
        .map_err(Error::new)
}
