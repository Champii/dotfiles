use std::{error::Error, path::PathBuf};

use clap::Parser;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    rock_lib::compile(&config.into());

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(long)]
    entry_file: PathBuf,
    #[arg(long)]
    output_dir: PathBuf,
    #[arg(long, default_value = "")]
    debug_print: String,
    #[arg(value_parser = parse_meta_files)]
    meta_files: Vec<(String, PathBuf)>, // crate name, path
}

fn parse_meta_files(s: &str) -> Result<(String, PathBuf), String> {
    let parsed = s.split('=').collect::<Vec<_>>();

    let [name, path] = parsed.as_slice() else {
        return Err(format!("`{}` isn't in the form `name=path`", s));
    };

    let path = PathBuf::from(path);

    Ok((name.to_string(), path))
}

impl From<Config> for rock_lib::Config {
    fn from(config: Config) -> Self {
        Self {
            entry_file: config.entry_file,
            output_dir: config.output_dir,
            debug_print: config.debug_print.split(',').map(String::from).collect(),
            meta_files: config.meta_files,
        }
    }
}
