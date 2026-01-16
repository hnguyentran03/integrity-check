use anyhow::Context;
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, prelude::*},
    collections::HashMap,
    path,
};
use sha2::{Sha256, Digest};

pub fn compute_hash(path: &path::PathBuf) -> anyhow::Result<String> {
    let file = File::open(path).context(format!("Could not open file {:?}", path.display()))?;
    let mut hasher = Sha256::new();
    let reader = BufReader::new(file);

    for byte in reader.bytes() {
        let byte = byte.context("Could not read byte")?;
        hasher.update(&[byte]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn store_hashes(hashes: &HashMap<path::PathBuf, String>, hash_file: &str) -> anyhow::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(hash_file)
        .context(format!("Could not open hash file {:?}", hash_file))?;

    for (path, hash) in hashes {
        file.write_all(format!("{}: {}\n", path.display(), hash).as_bytes())
            .context("Could not write hash to file")?;
    }
    Ok(())
}

pub fn load_hashes(hash_file: &str) -> anyhow::Result<HashMap<path::PathBuf, String>> {
    let file = File::open(hash_file)
        .context(format!("Could not open hash file {:?}", hash_file))?;
    let reader = BufReader::new(file);
    let mut hashes = HashMap::new();

    for line in reader.lines() {
        let line = line.context("Could not read line from hash file")?;
        let parts: Vec<&str> = line.splitn(2, ": ").collect();
        if parts.len() == 2 {
            let path = path::PathBuf::from(parts[0]);
            let hash = parts[1].to_string();
            hashes.insert(path, hash);
        }
    }

    Ok(hashes)
}

pub fn compare_hash(path: &path::PathBuf, hashes: &HashMap<path::PathBuf, String>) -> anyhow::Result<bool> {
    let current_hash = &compute_hash(path)?;
    let stored_hash = hashes.get(path)
        .ok_or_else(|| anyhow::anyhow!("No stored hash for file {:?}", path.display()))?;

    Ok(current_hash == stored_hash)
}

pub fn update_hash(path: &path::PathBuf, hashes: &mut HashMap<path::PathBuf, String>) -> anyhow::Result<()> {
    let new_hash = compute_hash(path)?;
    hashes.insert(path.clone(), new_hash);
    Ok(())
}