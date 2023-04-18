use std::{env, fs};

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
    #[arg(short, long, default_value_t = String::from("./docker-compose.yml"))]
    compose_file: String,

    /// Output filepath
    #[arg(short, long, default_value_t = String::from("./Statefile"))]
    out_file: String,
}

fn read_compose_file(filepath: &str) -> Result<compose::ComposeFile, serde_yaml::Error> {
    dbg!(filepath);
    let compose_file = fs::read_to_string(filepath).expect("Could not open file.");

    match serde_yaml::from_str::<compose::ComposeFile>(&compose_file) {
        Ok(file) => Ok(file),
        Err(_e) => {
            // The top-level enum for Compose V2 and Compose V3 tends to swallow meaningful errors
            // so re-parse the file as Compose V3 and print the error
            if let Err(e) = serde_yaml::from_str::<compose::Compose>(&compose_file) {
                eprintln!("Can't read {filepath} : {e}");
                return Err(e);
            }
            Err(_e)
        }
    }
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

fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();

    let input_file = &args.compose_file;

    let output_file = &args.out_file;

    let compose_data = read_compose_file(input_file)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;

    let state = compose_data.into();
    write_compose_file(output_file.to_owned(), state)
}
