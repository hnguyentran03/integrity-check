use anyhow::Context;
use std::{
    io::{BufReader, prelude::*},
    collections::HashMap,
    fs::File,
    path,
};
use sha2::{Sha256, Digest};
use rusqlite::Connection;

// Path must be unique, so it is the primary key
const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS file_hashes (
        path TEXT PRIMARY KEY NOT NULL,
        hash TEXT NOT NULL
    );
";

/// Compute the hash of a file
/// # Arguments
/// * `path`: Path to the file to compute the hash of
/// # Returns
/// * `Ok(String)`: If the hash is computed successfully
/// * `Err()`: If the hash cannot be computed
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

/// Store the hashes of a directory or file in the database
/// # Arguments
/// * `hashes`: HashMap of paths and their hashes
/// * `db_path`: Path to the database
/// # Returns
/// * `Ok(())`: If the hashes are stored successfully
/// * `Err()`: If the hashes cannot be stored
pub fn store_hashes(hashes: &HashMap<path::PathBuf, String>, db_path: &str) -> anyhow::Result<()> {
    let conn = Connection::open(db_path)
        .context(format!("Could not open database {:?}", db_path))?;
    conn.execute_batch(SCHEMA)
        .context("Could not create schema")?;

    for (path, hash) in hashes {
        // `to_string_lossy` returns `Cow<str>`; `&*` coerces it to `&str`
        conn.execute("INSERT INTO file_hashes (path, hash) VALUES (?1, ?2)", [&*path.to_string_lossy(), hash.as_str()])
            .context(format!("Could not insert hash for {:?}", path.display()))?;
    }
        
    Ok(())
}

/// Load the hashes of a directory or file from the database
/// # Arguments
/// * `db_path`: Path to the database
/// # Returns
/// * `Ok(HashMap<path::PathBuf, String>)`: If the hashes are loaded successfully
/// * `Err()`: If the hashes cannot be loaded
pub fn load_hashes(db_path: &str) -> anyhow::Result<HashMap<path::PathBuf, String>> {
    let conn = Connection::open(db_path)
        .context(format!("Could not open database {:?}", db_path))?;
    conn.execute_batch(SCHEMA)
        .context("Could not create schema")?;
        
    let mut stmt = conn.prepare("SELECT path, hash FROM file_hashes")?;
    let rows = stmt.query_map([], |row| {
        Ok((path::PathBuf::from(row.get::<_, String>(0)?), row.get::<_, String>(1)?))
    })
    .context("Could not query hashes")?;

    let mut hashes = HashMap::new();
    for row in rows {
        let (path, hash) = row.context("Failed to read row")?;
        hashes.insert(path, hash);
    }

    Ok(hashes)
}

/// Compare the hash of a file with the stored hash in the database
/// # Arguments
/// * `path`: Path to the file to compare the hash of
/// * `hashes`: HashMap of paths and their hashes
/// # Returns
/// * `Ok(bool)`: If the hash is compared successfully
/// * `Err()`: If the hash cannot be compared
pub fn compare_hash(path: &path::PathBuf, hashes: &HashMap<path::PathBuf, String>) -> anyhow::Result<bool> {
    let current_hash = &compute_hash(path)?;
    let stored_hash = hashes.get(path)
        .ok_or_else(|| anyhow::anyhow!("No stored hash for file {:?}", path.display()))?;

    Ok(current_hash == stored_hash)
}

/// Update the hash of a file in the database
/// # Arguments
/// * `path`: Path to the file to update the hash of
/// * `hashes`: HashMap of paths and their hashes
/// # Returns
/// * `Ok(())`: If the hash is updated successfully
/// * `Err()`: If the hash cannot be updated
pub fn update_hash(path: &path::PathBuf, hashes: &mut HashMap<path::PathBuf, String>) -> anyhow::Result<()> {
    let new_hash = compute_hash(path)?;
    hashes.insert(path.clone(), new_hash);
    Ok(())
}