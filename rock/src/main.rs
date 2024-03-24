use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    match config.command {
        Command::Format => todo!(),
        Command::Build => build(&config),
        Command::Run => todo!(),
        Command::Test => todo!(),
    }

    Ok(())
}

fn build(config: &Config) {
    let entry_file = "src/main.rk";

    println!("compiling crate");
    let out = std::process::Command::new("rockc")
        .arg("--entry-file")
        .arg(entry_file)
        .output()
        .expect("failed to execute process");

    println!("{}", String::from_utf8_lossy(&out.stdout));
    println!("{}", String::from_utf8_lossy(&out.stderr));
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Format,
    Build,
    Run,
    Test,
}
