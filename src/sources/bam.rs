use crate::{
    food::{self, FoodCode, Quantity, Unit},
    sources::source::Source,
};
use anyhow::{Context, Result};
use serde::{
    de::{self, Visitor},
    Deserialize,
};

pub struct Amount(f64);

impl<'de> Deserialize<'de> for Amount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_f64(AmountVisitor)
    }
}

struct AmountVisitor;

impl<'de> Visitor<'de> for AmountVisitor {
    type Value = Amount;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expected a number, a '.' or an empty string")
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "." || v == "" {
            Ok(Amount(0.0))
        } else {
            v.parse::<f64>().map(Amount).map_err(de::Error::custom)
        }
    }
}

#[derive(Deserialize)]
pub struct Food {
    #[serde(alias = "codigomex2")]
    pub code: String,
    #[serde(alias = "nombre_del_alimento")]
    pub name: String,
    #[serde(alias = "energ_kcal")]
    pub energy_kcal: f64,
    #[serde(alias = "fiber_td")]
    pub fiber_in_milligrams: Amount,
    #[serde(alias = "carbohydrt")]
    pub carbohydrates_in_grams: Amount,
    #[serde(alias = "lipid_tot")]
    pub lipids_in_grams: Amount,
    #[serde(alias = "protein")]
    pub protein_in_grams: Amount,
}

pub struct Bam<'a> {
    file: &'a str,
}

impl<'a> Bam<'a> {
    pub fn new(file: &'a str) -> Self {
        Bam { file }
    }
}

impl Source for Bam<'_> {
    fn load_foods(&self) -> Result<impl Iterator<Item = Result<food::Food>>> {
        let reader = csv::Reader::from_path(&self.file).context("failed to read file")?;
        Ok(reader.into_deserialize::<Food>().map(|res| {
            res.map(|raw| food::Food {
                code: FoodCode(raw.code),
                name: raw.name,
                energy: Quantity(raw.energy_kcal, Unit::Kcal),
                fiber: Quantity(raw.fiber_in_milligrams.0, Unit::Milligram),
                carbohydrates: Quantity(raw.carbohydrates_in_grams.0, Unit::Gram),
                lipids: Quantity(raw.lipids_in_grams.0, Unit::Gram),
                protein: Quantity(raw.protein_in_grams.0, Unit::Gram),
            })
            .with_context(|| "deserialization of record failed")
        }))
    }
}
