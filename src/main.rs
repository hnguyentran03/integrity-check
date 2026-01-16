use anyhow::Context;
use clap::Parser;
use std::{
    fs::{File, metadata},
    io::{BufReader, prelude::*}, path,
};

#[derive(Parser)]
struct Cli {
    command: String,
    path: std::path::PathBuf,
}

fn init(path: &std::path::PathBuf) -> anyhow::Result<()> {
    todo!("Implement initialization logic");
}

fn check(path: &std::path::PathBuf) -> anyhow::Result<()> {
    todo!("Implement integrity checking");
}

fn update(path: &std::path::PathBuf) -> anyhow::Result<()> {
    todo!("Implement updating mechanism");
}

fn main() -> anyhow::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = std::io::BufWriter::new(stdout);

    let args = Cli::parse();

    match args.command.as_str() {
        "init" => {
            init(&args.path)
        }
        "check" => {
            check(&args.path)
        }
        "update" => {
            update(&args.path)
        }
        _ => {
            writeln!(handle, "Unknown command: {}", args.command).context("Could not write to stdout")
        }
    }
}
