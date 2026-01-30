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

        let mut file = File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;
        file.sync_all()?;

        let mut hashes = HashMap::new();
        let hash = compute_hash(&file_path)?;
        hashes.insert(file_path.clone(), hash);
        assert!(compare_hash(&file_path, &hashes)?);

        let mut file = File::create(&file_path)?;
        writeln!(file, "Goodbye, world!")?;
        file.sync_all()?;
        assert!(!compare_hash(&file_path, &hashes)?);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_update_hash() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_update.txt");

        let mut file = File::create(&file_path)?;
        writeln!(file, "Initial content")?;
        file.sync_all()?;

        let mut hashes = HashMap::new();
        let initial_hash = compute_hash(&file_path)?;
        hashes.insert(file_path.clone(), initial_hash);

        let mut file = File::create(&file_path)?;
        writeln!(file, "Updated content")?;
        file.sync_all()?;
        assert!(!compare_hash(&file_path, &hashes)?);

        update_hash(&file_path, &mut hashes)?;
        assert!(compare_hash(&file_path, &hashes)?);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_store_and_load_hashes() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("test_store_load.txt");
        let db_path = dir.path().join("hashes.db");

        let mut file = File::create(&file_path)?;
        writeln!(file, "Some content")?;
        file.sync_all()?;

        let hash = compute_hash(&file_path)?;
        let mut hashes = HashMap::new();
        hashes.insert(file_path.clone(), hash.clone());
        store_hashes(&hashes, db_path.to_str().unwrap())?;

        assert!(db_path.exists());

        let loaded = load_hashes(db_path.to_str().unwrap())?;
        assert_eq!(loaded.get(&file_path).unwrap(), &hash);

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_load_empty_database() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("empty.db");

        let hashes = load_hashes(db_path.to_str().unwrap())?;
        assert!(hashes.is_empty());

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_store_multiple_hashes_and_load() -> anyhow::Result<()> {
        let dir = tempdir()?;
        let db_path = dir.path().join("multi.db");
        let file_a = dir.path().join("a.txt");
        let file_b = dir.path().join("b.txt");

        File::create(&file_a)?.write_all(b"file a")?;
        File::create(&file_b)?.write_all(b"file b")?;

        let mut hashes = HashMap::new();
        hashes.insert(file_a.clone(), compute_hash(&file_a)?);
        hashes.insert(file_b.clone(), compute_hash(&file_b)?);
        store_hashes(&hashes, db_path.to_str().unwrap())?;

        let loaded = load_hashes(db_path.to_str().unwrap())?;
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded.get(&file_a), hashes.get(&file_a));
        assert_eq!(loaded.get(&file_b), hashes.get(&file_b));

        dir.close()?;
        Ok(())
    }

    #[test]
    fn test_non_existent_file() {
        let non_existent_path = std::path::PathBuf::from("non_existent_file.txt");
        let result = compute_hash(&non_existent_path);
        assert!(result.is_err());
    }
}