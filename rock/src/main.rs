use std::{error::Error, path::PathBuf};

use clap::{Parser, Subcommand};
use rock_lib::ast::{
    visit::{walk_module, Visitor},
    Module, Program,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();

    match config.command {
        Command::Format => format(&config),
        Command::Build => build(&config),
        Command::Run => todo!(),
        Command::Test => todo!(),
        Command::Expand => expand(&config),
    }

    Ok(())
}

fn build(_config: &Config) {
    let entry_file = "src/main.rk";

    let out = std::process::Command::new("rockc")
        .arg("--entry-file")
        .arg(entry_file)
        .output()
        .expect("failed to execute process");

    if !out.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&out.stdout));
    }
    if !out.stderr.is_empty() {
        println!("{}", String::from_utf8_lossy(&out.stderr));
    }
}

fn format(_config: &Config) {
    let entry_file = "src/main.rk";
    let mut rockc_config = rock_lib::Config::default();
    rockc_config.entry_file = PathBuf::from(entry_file);

    let program: Program = rock_lib::parser::parse_root_file(&rockc_config).unwrap();

    program.visit(&mut AstFormater);
}

fn expand(_config: &Config) {
    let entry_file = "src/main.rk";
    let mut rockc_config = rock_lib::Config::default();
    rockc_config.entry_file = PathBuf::from(entry_file);

    let program: Program = rock_lib::parser::parse_root_file(&rockc_config).unwrap();
    let expanded = rock_lib::macro_expansion::expand_macros(program).unwrap();

    expanded.visit(&mut ExpandedPrint);
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
    Expand,
}

struct AstFormater;

impl<'a> Visitor<'a> for AstFormater {
    fn visit_module(&mut self, module: &'a Module) {
        if let Some(path) = &module.filepath {
            std::fs::write(path, module.to_string()).unwrap();
        }

        walk_module(self, module);
    }
}

struct ExpandedPrint;

impl<'a> Visitor<'a> for ExpandedPrint {
    fn visit_module(&mut self, module: &'a Module) {
        if let Some(name) = &module.name {
            println!("### {}: ###\n", name.to_string());
        }
        println!("{}", module.to_string());
        walk_module(self, module);
    }
}
