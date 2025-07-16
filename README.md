# afs

The first choice of fs library for Rust

[中文](./README.zh.md)

## afs Library Function Quick Reference

## File Operations

| Function Name      | Description                                  |
| ------------------ | -------------------------------------------- |
| `read_file_sync`   | Synchronously read file content to string    |
| `read_file`        | Asynchronously read file content to string   |
| `write_file_sync`  | Synchronously write string content to file   |
| `write_file`       | Asynchronously write string content to file  |
| `append_file_sync` | Synchronously append string content to file  |
| `append_file`      | Asynchronously append string content to file |
| `create_file_sync` | Create file (if not exists) with parent dirs |
| `unlink_sync`      | Synchronously delete file                    |

## Directory Operations

| Function Name | Description                     |
| ------------- | ------------------------------- |
| `mkdir_sync`  | Synchronously create directory  |
| `mkdir`       | Asynchronously create directory |
| `rmdir_sync`  | Synchronously remove directory  |
| `rmdir`       | Asynchronously remove directory |

## JSON File Operations

| Function Name       | Description               |
| ------------------- | ------------------------- |
| `read_from_json<T>` | Read JSON file to struct  |
| `read_json`         | Read JSON file to Value   |
| `write_to_json<T>`  | Write struct to JSON file |

## File/Directory Check Functions

| Function Name     | Description                            |
| ----------------- | -------------------------------------- |
| `file_exists`     | Check if file exists                   |
| `dir_exists`      | Check if directory exists              |
| `exists_sync`     | Synchronously check if path exists     |
| `exists`          | Asynchronously check if path exists    |
| `is_file`         | Check if path is a file                |
| `is_dir`          | Check if path is a directory           |
| `is_symlink`      | Check if path is a symbolic link       |
| `is_file_sync`    | Synchronously check if path is file    |
| `is_dir_sync`     | Synchronously check if path is dir     |
| `is_symlink_sync` | Synchronously check if path is symlink |

## File Size and Metadata Functions

| Function Name        | Description                      |
| -------------------- | -------------------------------- |
| `get_file_size`      | Get file size                    |
| `get_file_real_size` | Get real size of symlinked file  |
| `get_dir_size`       | Get directory size               |
| `stat_sync`          | Synchronously get file metadata  |
| `stat`               | Asynchronously get file metadata |

## System and Disk Functions

| Function Name | Description          |
| ------------- | -------------------- |
| `diskusage`   | Get disk usage       |
| `which`       | Find command in PATH |

## Temporary File and Directory Functions

| Function Name | Description                |
| ------------- | -------------------------- |
| `mktempdir`   | Create temporary directory |
| `mktempfile`  | Create temporary file      |

## Permission and Link Functions

| Function Name | Description             |
| ------------- | ----------------------- |
| `chmod_sync`  | Change file permissions |
| `soft_link`   | Create symbolic link    |

## Path Processing Functions

| Function Name    | Description                              |
| ---------------- | ---------------------------------------- |
| `resolve`        | Process path in Node.js style            |
| `normalize_path` | Replace backslashes with forward slashes |
| `get_filepath`   | Get canonicalized file path              |
| `basename`       | Get base filename                        |
| `filename`       | Get filename with extension              |
| `dirname`        | Get directory part of path               |

## Hash Functions

| Function Name | Description                     |
| ------------- | ------------------------------- |
| `hash_sync`   | Synchronously calculate SHA256  |
| `hash`        | Asynchronously calculate SHA256 |

## Quick Find by Category

### 📁 File I/O

- Read: `read_file`
- Write: `write_file`, `write_file_sync`
- Append: `append_file`, `append_file_sync`
- Create: `create_file_sync`
- Delete: `unlink_sync`

### 📂 Directory Operations

- Create: `mkdir`, `mkdir_sync`
- Remove: `rmdir`, `rmdir_sync`

### 🔍 Existence Checks

- Files: `file_exists`, `is_file`, `is_file_sync`
- Directories: `dir_exists`, `is_dir`, `is_dir_sync`
- Symlinks: `is_symlink`, `is_symlink_sync`
- General: `exists`, `exists_sync`

### 📊 Size and Information

- File size: `get_file_size`, `get_file_real_size`
- Directory size: `get_dir_size`
- Metadata: `stat`, `stat_sync`
- Disk usage: `diskusage`

### 🛣️ Path Processing

- Normalize: `normalize_path`, `get_filepath`
- Resolve: `resolve`
- Extract: `basename`, `filename`, `dirname`

### 🔧 System Functions

- Temporary files: `mktempdir`, `mktempfile`
- Permissions: `chmod_sync`
- Links: `soft_link`
- Command lookup: `which`
- Hashing: `hash`, `hash_sync`

### 📋 JSON Operations

- Read: `read_json`, `read_from_json`
- Write: `write_to_json`

## Usage Guide

### Async vs Sync

- Functions with `_sync` suffix are synchronous versions
- Functions without suffix are usually asynchronous (require `.await`)
- Selection guidelines:
  - Use async versions in async environments
  - Use sync versions in sync environments or simple scripts

### Error Handling

- Most functions return `AnyResult<T>`, requiring error handling
- Check functions (like `exists`, `is_file`, etc.) return `bool` directly

### Path Format

- Cross-platform path support (auto-handles Windows and Unix separators)
- `normalize_path` can unify path format

### Example Usage

```rust
use afs::*;

// Async file operations
async fn example() -> AnyResult<()> {
    // Read file
    let content = read_file("test.txt").await?;

    // Write file
    write_file("output.txt", &content).await?;

    // Check if file exists
    if file_exists("output.txt").await {
        println!("File created successfully");
    }

    // Get file size
    let size = get_file_size("output.txt").await?;
    println!("File size: {} bytes", size);

    Ok(())
}

// Sync operations
fn sync_example() -> AnyResult<()> {
    // Create directory
    mkdir_sync("new_dir")?;

    // Create file
    create_file_sync("new_dir/test.txt")?;

    // Write content
    write_file_sync("new_dir/test.txt", "Hello, World!")?;

    // Get path information
    let dir = dirname("new_dir/test.txt")?;
    let name = basename("new_dir/test.txt")?;

    println!("Directory: {}, Filename: {}", dir, name);

    Ok(())
}
```
