use anyhow::Context;
use std::{
    fs::File,
    io::{BufReader, prelude::*}, path,
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

pub fn store_hash(hash: &str, hash_file: &str) -> anyhow::Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(hash_file)
        .context(format!("Could not open hash file {:?}", hash_file))?;

    file.write_all(hash.as_bytes())
        .context(format!("Could not write hash to file {:?}", hash_file))
}