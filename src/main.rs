use anyhow::Context;
use clap::Parser;
use std::{
    collections::HashMap, fs::{metadata, read_dir}, io::{BufWriter, prelude::*}, path::PathBuf
};
use integrity_check::{compute_hash, store_hashes, load_hashes, compare_hash, update_hash};

const HASH_FILE: &str = ".hashes";

#[derive(Parser)]
struct Cli {
    command: String,
    path: PathBuf,
}

// Initialize integrity checking by computing and storing hashes of logs into a file
fn init(path: &PathBuf, handle: &mut BufWriter<std::io::Stdout>) -> anyhow::Result<()> {
    // Read metadata to determine if path is file or directory
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let mut hashes: HashMap<PathBuf, String> = HashMap::new();

    if md.is_dir() {
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();

            let hash = compute_hash(&entry_path)?;
            hashes.insert(entry_path, hash.clone());
        }

        store_hashes(&hashes, HASH_FILE)?;
        
        writeln!(handle, "Hashes stored succesfully")
            .context("Could not write to stdout")?;
        Ok(())
    } else if md.is_file() {
        let hash = compute_hash(path)?;
        hashes.insert(path.clone(), hash.clone());
        store_hashes(&hashes, HASH_FILE)?;

        writeln!(handle, "Hashes stored succesfully")
            .context("Could not write to stdout")?;
        Ok(())
    } else {
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
}

// Check integrity of files by comparing current hashes with stored hashes
fn check(path: &PathBuf) -> anyhow::Result<()> {
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let hashes = load_hashes(HASH_FILE)?;

    if md.is_dir() {
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();
            
            if compare_hash(&entry_path, &hashes)? {
                println!("File {:?} is unchanged", entry_path.display());
            } else {
                println!("File {:?} has been modified", entry_path.display());
            }
        }
        Ok(())
    } else if md.is_file() {
        if compare_hash(path, &hashes)? {
            println!("File {:?} is unchanged", path.display());
        } else {
            println!("File {:?} has been modified", path.display());
        }
        Ok(())
    } else {
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
}

// Update stored hashes after changes
fn update(path: &PathBuf) -> anyhow::Result<()> {
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let mut hashes: HashMap<PathBuf, String> = load_hashes(HASH_FILE)?;

    if md.is_dir() {
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();
            update_hash(&entry_path, &mut hashes)?;
        }

        store_hashes(&hashes, HASH_FILE)?;

        println!("Hashes updated successfully");
        Ok(())
    } else if md.is_file() {
        update_hash(path, &mut hashes)?;
        store_hashes(&hashes, HASH_FILE)?;

        println!("Hash updated successfully");
        Ok(())
    } else {
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
}

fn main() -> anyhow::Result<()> {
    let stdout = std::io::stdout();
    let mut handle = BufWriter::new(stdout);

    let args = Cli::parse();

    match args.command.as_str() {
        "init" => {
            init(&args.path, &mut handle)
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
