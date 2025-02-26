mod db;
mod food;
mod sources;

use crate::db::queries;
use crate::food::Food;
use crate::sources::{bam::Bam, source::Source};
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use itertools::Itertools;
use postgres::NoTls;
use std::env;

#[derive(Debug, Subcommand)]
enum Commands {
    Bam {
        #[arg(short, long)]
        file: String,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(short, long)]
    chunk_size: Option<usize>,
}

fn required_env_var(key: &str) -> anyhow::Result<String> {
    env::var(key).with_context(|| format!("unable to read var {}", key))
}

const DEFAULT_CHUNK_SIZE: usize = 100;

fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let cli = Cli::parse();

    let user = required_env_var("POSTGRES_USER")?;
    let dbname = required_env_var("POSTGRES_DB")?;
    let host = required_env_var("POSTGRES_HOST")?;
    let password = required_env_var("POSTGRES_PASSWORD")?;

    let config = format!("user={user} host={host} password={password} dbname={dbname}");
    let mut client = postgres::Client::connect(&config, NoTls)
        .with_context(|| format!("failed to connect to the database: config: {config}"))?;

    queries::create_unit_type(&mut client)?;
    queries::create_quantity_type(&mut client)?;
    queries::create_foods_table(&mut client)?;

    let source = match &cli.command {
        Commands::Bam { file } => Bam::new(file),
    };

    let chunk_size = cli.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE);
    let chunks = source.load_foods()?.chunks(chunk_size);
    let foods = chunks.into_iter().flat_map(|chunk| chunk.collect::<Result<Vec<Food>>>());
    let errors = chunks.into_iter().flat_map(|chunk| {
        chunk.filter_map(|res| match res {
            Ok(_) => None,
            Err(cause) => Some(cause)
        })
    });

    for error in errors {
        return Err(error);
    }

    for chunk in foods {
        queries::insert_foods(&mut client, &chunk)?;
    }

    Ok(())
}
