use anyhow::Context;
use clap::Parser;
use std::{
    collections::HashMap, fs::{metadata, read_dir}, io::{BufWriter, prelude::*}, path::PathBuf
};
use integrity_check::{compute_hash, store_hashes, load_hashes, compare_hash, update_hash};

const DATABASE_FILE: &str = "hashes.db";

#[derive(Parser)]
struct Cli {
    command: String,
    path: PathBuf,
}

/// Initialize integrity checking by computing and storing hashes of logs into a file
/// # Arguments
/// * `path`: Path to the directory or file to initialize
/// * `handle`: Handle to stdout
/// # Returns
/// * `Ok(())`: If the hashes are stored successfully
/// * `Err()`: If the hashes cannot be stored
fn init(path: &PathBuf, handle: &mut BufWriter<std::io::Stdout>) -> anyhow::Result<()> {
    // Read metadata to determine if path is file or directory
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let mut hashes: HashMap<PathBuf, String> = HashMap::new();

    if md.is_dir() { // Initialize hashes of directory
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            // Initialize hashes of each file in directory
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();

            let hash = compute_hash(&entry_path)?;
            hashes.insert(entry_path, hash.clone());
        }

        store_hashes(&hashes, DATABASE_FILE)?;
        
        writeln!(handle, "Hashes stored succesfully")
            .context("Could not write to stdout")?;
        Ok(())
    } else if md.is_file() { // Initialize hashes of file
        let hash = compute_hash(path)?;
        hashes.insert(path.clone(), hash.clone());
        store_hashes(&hashes, DATABASE_FILE)?;

        writeln!(handle, "Hashes stored succesfully")
            .context("Could not write to stdout")?;
        Ok(())
    } else { // Path is neither a file nor a directory
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
}

/// Check integrity of files by comparing current hashes with stored hashes
/// # Arguments
/// * `path`: Path to the directory or file to check
/// # Returns
/// * `Ok(())`: If the integrity is checked successfully
/// * `Err()`: If the integrity cannot be checked
fn check(path: &PathBuf) -> anyhow::Result<()> {
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let hashes = load_hashes(DATABASE_FILE)?;

    if md.is_dir() { // Check integrity of directory
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        { 
            // Check integrity of each file in directory
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();
            
            if compare_hash(&entry_path, &hashes)? {
                println!("File {:?} is unchanged", entry_path.display());
            } else {
                println!("File {:?} has been modified", entry_path.display());
            }
        }
        Ok(())
    } else if md.is_file() { // Check integrity of file
        if compare_hash(path, &hashes)? {
            println!("File {:?} is unchanged", path.display());
        } else {
            println!("File {:?} has been modified", path.display());
        }
        Ok(())
    } else { // Path is neither a file nor a directory
        Err(anyhow::anyhow!("Path is neither a file nor a directory"))
    }
}

/// Update stored hashes after changes
/// # Arguments
/// * `path`: Path to the directory or file to update
/// # Returns
/// * `Ok(())`: If the hashes are updated successfully
/// * `Err()`: If the hashes cannot be updated
fn update(path: &PathBuf) -> anyhow::Result<()> {
    let md = metadata(path)
        .context(format!("Could not read metadata of file {:?}", path.display()))?;

    let mut hashes: HashMap<PathBuf, String> = load_hashes(DATABASE_FILE)?;

    if md.is_dir() { // Update hashes of directory
        for entry in read_dir(path)
            .context(format!("Could not read directory {:?}", path.display()))?
        {
            // Update hashes of each file in directory
            let entry = entry.context("Could not get directory entry")?;
            let entry_path = entry.path();
            update_hash(&entry_path, &mut hashes)?;
        }

        store_hashes(&hashes, DATABASE_FILE)?;

        println!("Hashes updated successfully");
        Ok(())
    } else if md.is_file() { // Update hashes of file
        update_hash(path, &mut hashes)?;
        store_hashes(&hashes, DATABASE_FILE)?;

        println!("Hash updated successfully");
        Ok(())
    } else { // Path is neither a file nor a directory
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
