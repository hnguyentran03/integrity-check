use integrity_check::{
    compute_hash, store_hashes, load_hashes, compare_hash, update_hash};

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;
    use std::collections::HashMap;

    #[test]
    fn test_file_integrity_check() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test.txt");

        // Create a test file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;
        file.sync_all()?;

        // Store initial hash
        let mut hashes = HashMap::new();
        let hash = compute_hash(&file_path)?;
        hashes.insert(file_path.clone(), hash);

        assert!(compare_hash(&file_path, &hashes)?);

        // Modify the file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Goodbye, world!")?;
        file.sync_all()?;

        // Check integrity (should be modified)
        assert!(!compare_hash(&file_path, &hashes)?);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_update_hash() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_update.txt");

        // Create a test file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Initial content")?;
        file.sync_all()?;

        // Store initial hash
        let mut hashes = HashMap::new();
        let initial_hash = compute_hash(&file_path)?;
        hashes.insert(file_path.clone(), initial_hash);

        // Modify the file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Updated content")?;
        file.sync_all()?;

        // Hashes should not match now
        assert!(!compare_hash(&file_path, &hashes)?);

        // Update the hash in the hashmap
        update_hash(&file_path, &mut hashes)?;

        // Now the hash should match
        assert!(compare_hash(&file_path, &hashes)?);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_store_and_load_hashes() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_store_load.txt");
        let hash_file = dir.path().join("hashes.txt");

        // Create a test file
        let mut file = File::create(&file_path)?;
        writeln!(file, "Some content")?;
        file.sync_all()?;

        // Compute hash and store it
        let hash = compute_hash(&file_path)?;
        let mut hashes = HashMap::new();
        hashes.insert(file_path.clone(), hash.clone());
        store_hashes(&hashes, hash_file.to_str().unwrap())?;

        // Load hashes and verify
        let loaded_hashes = load_hashes(hash_file.to_str().unwrap())?;
        assert_eq!(loaded_hashes.get(&file_path).unwrap(), &hash);

        dir.close()?;
        Ok(())
    }
}