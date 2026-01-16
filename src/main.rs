use anyhow::Context;
use clap::Parser;
use std::{
    fs::metadata,
    io::prelude::*,
};
use integrity_check::{compute_hash, store_hash};

const HASH_FILE: &str = ".hashes";

#[derive(Parser)]
struct Cli {
    command: String,
    path: std::path::PathBuf,
}

fn init(path: &std::path::PathBuf) -> anyhow::Result<()> {
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    if md.is_dir() {
        for entry in std::fs::read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();
            let hash = compute_hash(&entry_path)?;
            store_hash(&hash, HASH_FILE)?;
        }
        Ok(())
    } else if md.is_file() {
        let hash = compute_hash(path)?;
        store_hash(&hash, HASH_FILE)
    } else {
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
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
