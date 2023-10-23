use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use std::fs;
use std::io;

mod cargoes;
mod compose;
mod healthcheck;
mod ports;
mod resources;
mod statefile;
mod utils;

const DEFAULT_INPUT_FILE: &str = "./docker-compose.yml";
const DEFAULT_OUTPUT_FILE: &str = "./Statefile.yml";

/// C2ncl args
#[derive(Parser, Debug)]
#[clap(about, version, name = "c2ncl")]
struct Args {
    /// Path to docker-compose file
    #[arg(short = 'i', long, default_value_t = DEFAULT_INPUT_FILE.to_string())]
    in_file: String,

    /// Output filepath
    #[arg(short = 'o', long, default_value_t = DEFAULT_OUTPUT_FILE.to_string())]
    out_file: String,

    /// skip confirmation
    #[clap(short = 'y')]
    pub skip_confirm: bool,
}

/// ## Confirm
///
/// Ask for confirmation
///
/// ## Arguments
///
/// * [msg](str) The message to display
///
/// ## Return
///
/// * [Result](Result) The result of the operation
///   * [Ok](()) The operation is confirmed
///   * [Err](io::Error) An error occured
///
pub fn confirm(msg: &str) -> Result<()> {
    let result = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .default(false)
        .interact();
    match result {
        Ok(true) => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Interrupted, "interupted by user").into()),
    }
}

fn read_compose_file(filepath: &str) -> Result<compose::ComposeFile> {
    let compose_file = fs::read_to_string(filepath)
        .with_context(|| format!("Error while trying to read {filepath}"))?;
    let data = serde_yaml::from_str::<compose::Compose>(&compose_file)
        .with_context(|| format!("Error while trying to parse {filepath}"))?;
    Ok(compose::ComposeFile::V2Plus(data))
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
    let input_file = &args.in_file;
    let output_file = &args.out_file;
    if !args.skip_confirm && fs::metadata(output_file).is_ok() {
        confirm(&format!("Overwrite  {}?", output_file))?;
    }
    let compose_data = read_compose_file(input_file)?;
    let state = compose_data.into();
    write_compose_file(output_file.to_owned(), state)?;
    Ok(())
}
