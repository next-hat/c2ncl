use anyhow::{Context, Result};
use std::fs;

mod cargoes;
mod compose;
mod healthcheck;
mod ports;
mod resources;
mod statefile;
mod utils;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author = "anonkey", version, about, long_about = None)]
struct Args {
    /// Path to docker-compose file
    #[arg(short, long, default_value = "./docker-compose.yml")]
    compose_file: String,

    /// Output filepath
    #[arg(short, long, default_value = "./StateFile")]
    out_file: String,
}

fn read_compose_file(filepath: &str) -> Result<compose::ComposeFile> {
    let compose_file = fs::read_to_string(filepath)
        .with_context(|| format!("Error while trying to read {filepath}"))?;
    let data = serde_yaml::from_str::<compose::ComposeFile>(&compose_file)
        .with_context(|| format!("Error while trying to parse {filepath}"))?;

    Ok(data)
}

fn write_compose_file(
    filepath: String,
    statefile: statefile::Statefile,
) -> Result<(), std::io::Error> {
    match serde_yaml::to_string(&statefile) {
        Ok(str) => fs::write(filepath, str),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_file = &args.compose_file;

    let output_file = &args.out_file;

    let compose_data = read_compose_file(input_file)?;
    let state = compose_data.into();
    write_compose_file(output_file.to_owned(), state)?;
    Ok(())
}
