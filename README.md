# File Integrity Checker

The tool is capable of the following:
- Accept a directory or a single log file as input.
- Utilize a cryptographic hashing algorithm, such as SHA-256, to compute hashes for each log file provided.
- On first use, store the computed hashes in a secure location.
- For subsequent uses, compare the newly computed hashes against the previously stored ones.
- Clearly report any discrepancies found as a result of the hash comparison, indicating possible file tampering.
- Allow for manual re-initialization of log file integrity.

## How to use
Program can be run using
```bash
./integrity-check { init | check | update } { path }
```

Use the `init` argument to initialize and store hashes of a directory or file in `path` into `.hashes`:
```bash
./integrity-check init logs/file.log # Stores hash of a file
./integrity-check init /logs # Stores hash of a directory
```

Use the `check` argument to check the integrity of the directory or file in `path`:
```bash
./integrity-check check logs/file.log # Checks integrity of file by comparing hashes to init
./integrity-check check /logs # Checks integrity of directory by comparing hashes of files inside to init
```

Use the `update` argument to update the hashes of the directory or file in `path`:
```bash
./integrity-check update logs/file.log # Update hashes of file
./integrity-check check /logs # Update hashes of directory
```