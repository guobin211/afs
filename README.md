# afs

[![Crates.io](https://img.shields.io/crates/v/afs.svg)](https://crates.io/crates/afs)
[![Documentation](https://docs.rs/afs/badge.svg)](https://docs.rs/afs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and powerful filesystem library for Rust, inspired by Node.js fs module.

[中文文档](./README.zh.md)

## Features

- Async/Sync API support
- Simple error handling with `AfsError`
- JSON file operations
- Path utilities (basename, dirname, normalize)
- File hash calculation (SHA256)
- Temporary file/directory creation
- Cross-platform support

## Installation

```toml
[dependencies]
afs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use afs::*;

#[tokio::main]
async fn main() -> AfsResult<()> {
    // Write file
    write_file("hello.txt", "Hello, World!").await?;
    
    // Read file
    let content = read_file("hello.txt").await?;
    println!("{}", content);
    
    // Check if file exists
    if file_exists("hello.txt").await {
        println!("File exists!");
    }
    
    Ok(())
}
```

## API Reference

### File Operations

| Function           | Description                              |
| ------------------ | ---------------------------------------- |
| `read_file`        | Async read file content to string        |
| `read_file_sync`   | Sync read file content to string         |
| `write_file`       | Async write string content to file       |
| `write_file_sync`  | Sync write string content to file        |
| `append_file`      | Async append string content to file      |
| `append_file_sync` | Sync append string content to file       |
| `create_file_sync` | Create file with parent directories      |
| `unlink_sync`      | Sync delete file                         |

### Directory Operations

| Function     | Description                  |
| ------------ | ---------------------------- |
| `mkdir`      | Async create directory       |
| `mkdir_sync` | Sync create directory        |
| `rmdir`      | Async remove directory       |
| `rmdir_sync` | Sync remove directory        |

### JSON Operations

| Function            | Description               |
| ------------------- | ------------------------- |
| `read_from_json<T>` | Read JSON file to struct  |
| `read_json`         | Read JSON file to Value   |
| `write_to_json<T>`  | Write struct to JSON file |

### Check Functions

| Function          | Description                      |
| ----------------- | -------------------------------- |
| `file_exists`     | Async check if file exists       |
| `dir_exists`      | Async check if directory exists  |
| `exists`          | Async check if path exists       |
| `exists_sync`     | Sync check if path exists        |
| `is_file`         | Async check if path is a file    |
| `is_file_sync`    | Sync check if path is a file     |
| `is_dir`          | Async check if path is a dir     |
| `is_dir_sync`     | Sync check if path is a dir      |
| `is_symlink`      | Async check if path is a symlink |
| `is_symlink_sync` | Sync check if path is a symlink  |

### Metadata Functions

| Function             | Description                     |
| -------------------- | ------------------------------- |
| `get_file_size`      | Get file size in bytes          |
| `get_file_real_size` | Get real size of symlinked file |
| `get_dir_size`       | Get total directory size        |
| `stat`               | Async get file metadata         |
| `stat_sync`          | Sync get file metadata          |

### System Functions

| Function    | Description          |
| ----------- | -------------------- |
| `diskusage` | Get disk usage       |
| `which`     | Find command in PATH |

### Temporary File/Directory

| Function         | Description              |
| ---------------- | ------------------------ |
| `create_tempdir` | Create temporary directory |
| `create_tempfile`| Create temporary file      |

### Permission and Link

| Function     | Description             |
| ------------ | ----------------------- |
| `chmod_sync` | Change file permissions |
| `soft_link`  | Create symbolic link    |

### Path Utilities

| Function         | Description                              |
| ---------------- | ---------------------------------------- |
| `resolve`        | Resolve path like Node.js                |
| `normalize_path` | Replace backslashes with forward slashes |
| `get_filepath`   | Get canonicalized file path              |
| `basename`       | Get base filename                        |
| `filename`       | Get filename with extension              |
| `dirname`        | Get directory part of path               |

### Hash Functions

| Function    | Description                 |
| ----------- | --------------------------- |
| `hash`      | Async calculate SHA256 hash |
| `hash_sync` | Sync calculate SHA256 hash  |

## Examples

### Read and Write JSON

```rust
use afs::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Config {
    name: String,
    version: u32,
}

#[tokio::main]
async fn main() -> AfsResult<()> {
    let config = Config {
        name: "my-app".to_string(),
        version: 1,
    };
    
    // Write JSON
    write_to_json("config.json", &config).await?;
    
    // Read JSON
    let loaded: Config = read_from_json("config.json").await?;
    println!("Name: {}, Version: {}", loaded.name, loaded.version);
    
    Ok(())
}
```

### Create Temporary Files

```rust
use afs::*;

#[tokio::main]
async fn main() -> AfsResult<()> {
    // Create temp directory
    let temp_dir = create_tempdir().await?;
    println!("Temp dir: {}", temp_dir);
    
    // Create temp file with extension
    let temp_file = create_tempfile(".txt").await?;
    println!("Temp file: {}", temp_file);
    
    Ok(())
}
```

### Path Operations

```rust
use afs::*;

fn main() -> AfsResult<()> {
    let path = "/home/user/documents/file.txt";
    
    println!("basename: {}", basename(path)?);  // file.txt
    println!("dirname: {}", dirname(path)?);    // /home/user/documents
    println!("filename: {}", filename(path)?);  // file.txt
    
    // Resolve relative path
    let resolved = resolve("/home/user", "../test.txt")?;
    println!("resolved: {}", resolved);  // /home/test.txt
    
    Ok(())
}
```

## License

MIT License

