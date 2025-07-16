use std::{
    env,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

// Combined Result and anyhow imports
use anyhow::{Context, Result as AnyResult, anyhow};
use serde::Deserialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/// Re-exporting fs_err for compatibility with std::fs
pub use fs_err::*;
/// Re-exporting fs_extra for convenience
pub use fs_extra::*;

/// Asynchronously read file content to string
///
/// # Arguments
/// - `path`: Path to the file to read
///
/// # Returns
/// - On success, returns a string containing the file content
/// - On failure, returns an error
pub async fn read_file(path: &str) -> AnyResult<String> {
    // Asynchronously read file content to string
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read file: {path}"))?;
    Ok(content)
}

/// Synchronously read file content to string
///
/// # Arguments
/// - `path`: Path to the file to read
///
/// # Returns
/// - On success, returns a string containing the file content
/// - On failure, returns an error
pub fn read_file_sync(path: &str) -> AnyResult<String> {
    let content =
        std::fs::read_to_string(path).with_context(|| format!("Failed to read file: {path}"))?;
    Ok(content)
}

/// Synchronously write string content to file
///
/// # Arguments
/// - `path`: Path to the file to write
/// - `content`: String content to write
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn write_file_sync(path: &str, content: &str) -> AnyResult<()> {
    // Create a new file, overwriting if it already exists
    let mut file = File::create(path)?;
    // Write the content to the file
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Asynchronously write string content to file
///
/// # Arguments
/// - `path`: Path to the file to write
/// - `content`: String content to write
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub async fn write_file(path: &str, content: &str) -> AnyResult<()> {
    // Asynchronously create a new file, overwriting if it already exists
    let mut file = tokio::fs::File::create(path)
        .await
        .with_context(|| format!("Failed to create file: {path}"))?;
    // Asynchronously write content to the file
    file.write_all(content.as_bytes())
        .await
        .with_context(|| format!("Failed to write to file: {path}"))?;
    Ok(())
}

/// Synchronously append string content to file
///
/// # Arguments
/// - `path`: Path to the file to append to
/// - `content`: String content to append
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn append_file_sync(path: &str, content: &str) -> AnyResult<()> {
    // Open file in append mode
    let mut file = File::options().append(true).open(path)?;
    // Append content to the file
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Asynchronously append string content to file
///
/// # Arguments
/// - `path`: Path to the file to append to
/// - `content`: String content to append
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub async fn append_file(path: &str, content: &str) -> AnyResult<()> {
    // Asynchronously open file in append mode
    let mut file = tokio::fs::OpenOptions::new()
        .append(true)
        .create(true) // Create if it doesn't exist, common for append
        .open(path)
        .await
        .with_context(|| format!("Failed to open file for appending: {path}"))?;
    // Asynchronously append content to the file
    file.write_all(content.as_bytes())
        .await
        .with_context(|| format!("Failed to append to file: {path}"))?;
    Ok(())
}

/// Synchronously create directory
///
/// # Arguments
/// - `path`: Path to the directory to create
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn mkdir_sync(path: &str) -> AnyResult<()> {
    // Synchronously create directory, non-recursive
    dir::create(path, false)?;
    Ok(())
}

/// Asynchronously create directory
///
/// # Arguments
/// - `path`: Path to the directory to create
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub async fn mkdir(path: &str) -> AnyResult<()> {
    // Asynchronously create directory recursively
    tokio::fs::create_dir_all(path)
        .await
        .with_context(|| format!("Failed to create directory: {path}"))?;
    Ok(())
}

/// Synchronously remove directory
///
/// # Arguments
/// - `path`: Path to the directory to remove
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn rmdir_sync(path: &str) -> AnyResult<()> {
    // Synchronously remove directory
    dir::remove(path)?;
    Ok(())
}

/// Asynchronously remove directory
///
/// # Arguments
/// - `path`: Path to the directory to remove
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub async fn rmdir(path: &str) -> AnyResult<()> {
    // Asynchronously remove directory recursively
    tokio::fs::remove_dir_all(path)
        .await
        .with_context(|| format!("Failed to remove directory: {path}"))?;
    Ok(())
}

/// Read JSON file to struct, ignoring undefined fields
///
/// # Arguments
/// - `file_path`: Path to the JSON file to read
///
/// # Returns
/// - On success, returns the deserialized struct
/// - On failure, returns an error
pub async fn read_from_json<T: for<'a> Deserialize<'a>>(file_path: &str) -> AnyResult<T> {
    // Read file content
    let content = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Failed to read JSON file: {file_path}"))?;

    // Parse JSON string to struct
    let data: T = serde_json::from_str::<T>(&content)
        .with_context(|| format!("Failed to parse JSON from file: {file_path}"))?;

    Ok(data)
}

/// Read JSON file to serde_json::Value
///
/// # Arguments
/// - `file_path`: Path to the JSON file to read
///
/// # Returns
/// - On success, returns the JSON value
/// - On failure, returns an error
pub async fn read_json(file_path: &str) -> AnyResult<serde_json::Value> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Failed to read JSON file: {file_path}"))?;
    let data: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from file: {file_path}"))?;
    Ok(data)
}

/// Write struct to JSON file
///
/// # Arguments
/// - `file_path`: Path to the JSON file to write
/// - `data`: Data to serialize and write
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub async fn write_to_json<T: serde::Serialize>(file_path: &str, data: &T) -> AnyResult<()> {
    let mut file = tokio::fs::File::create(file_path)
        .await
        .with_context(|| format!("Failed to create JSON file: {file_path}"))?;
    let json = serde_json::to_string_pretty(data)
        .with_context(|| format!("Failed to serialize data to JSON for file: {file_path}"))?;
    file.write_all(json.as_bytes())
        .await
        .with_context(|| format!("Failed to write JSON to file: {file_path}"))?;
    Ok(())
}

/// Check if file exists
///
/// # Arguments
/// - `file_path`: Path to the file to check
///
/// # Returns
/// - `true` if the path exists and is a file
/// - `false` otherwise
pub async fn file_exists(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// Check if directory exists
///
/// # Arguments
/// - `dir_path`: Path to the directory to check
///
/// # Returns
/// - `true` if the path exists and is a directory
/// - `false` otherwise
pub async fn dir_exists(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// Check if path is a file
///
/// # Arguments
/// - `file_path`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a file
/// - `false` otherwise
pub async fn is_file(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// Check if path is a directory
///
/// # Arguments
/// - `dir_path`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a directory
/// - `false` otherwise
pub async fn is_dir(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// Check if path is a symbolic link
///
/// # Arguments
/// - `path`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a symbolic link
/// - `false` otherwise
pub async fn is_symlink(path: &str) -> bool {
    tokio::fs::symlink_metadata(path)
        .await
        .map(|metadata| metadata.is_symlink())
        .unwrap_or(false)
}

/// Get file size
///
/// # Arguments
/// - `file_path`: Path to the file to get size of
///
/// # Returns
/// - On success, returns the file size in bytes
/// - On failure, returns an error
pub async fn get_file_size(file_path: &str) -> AnyResult<u64> {
    let metadata = tokio::fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to get metadata for file size: {file_path}"))?;
    Ok(metadata.len())
}

/// Get real size of symlinked file (actual size of the target file)
///
/// # Arguments
/// - `file_path`: Path to the file to get size of
///
/// # Returns
/// - On success, returns file size in bytes
/// - On failure, returns an error
pub async fn get_file_real_size(file_path: &str) -> AnyResult<u64> {
    // tokio::fs::metadata follows symlinks by default.
    let metadata = tokio::fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to get metadata for file real size: {file_path}"))?;
    Ok(metadata.len())
}

/// Get directory size
///
/// # Arguments
/// - `dir_path`: Path to the directory to calculate size for
///
/// # Returns
/// - On success, returns total directory size in bytes
/// - On failure, returns an error
pub async fn get_dir_size(dir_path: &str) -> AnyResult<u64> {
    use std::path::PathBuf;

    let mut total_size = 0;
    let mut stack = vec![PathBuf::from(dir_path)];

    while let Some(path) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&path).await?;

        while let Some(entry) = entries.next_entry().await? {
            let metadata = entry.metadata().await?;

            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() {
                // Check if it's a symlink to avoid infinite loops
                if !metadata.is_symlink() {
                    stack.push(entry.path());
                }
            }
        }
    }

    Ok(total_size)
}

/// Get disk usage
///
/// # Returns
/// - On success, returns used disk space in bytes
/// - On failure, returns an error
pub async fn diskusage() -> AnyResult<f64> {
    use sys_info::disk_info; // Keep use statement localized if only used here
    let info = disk_info().map_err(|e| anyhow!("Failed to get disk info: {}", e))?;
    let used = info.total - info.free;
    Ok(used as f64)
}

/// Create temporary directory
///
/// # Returns
/// - On success, returns temporary directory path string
/// - On failure, returns an error
pub async fn mktempdir() -> AnyResult<String> {
    let dir = tempfile::TempDir::new()
        .context("Failed to create temp directory using tempfile::TempDir")?;
    // Keep the directory alive by converting TempDir to a PathBuf using into_path()
    let path_buf: PathBuf = dir.keep();
    path_buf.to_str().map(|s| s.to_string()).ok_or_else(|| {
        anyhow!(
            "Temporary directory path is not valid UTF-8: {:?}",
            path_buf
        )
    })
}

/// Generate timestamp-based random filename
///
/// # Arguments
/// - `ext`: File extension
///
/// # Returns
/// - Returns a random filename containing timestamp with specified extension
fn random_file_name(ext: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut result = String::new();
    let mut num = timestamp;
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let base = charset.len() as u64;

    while num > 0 {
        let remainder = num % base;
        result.push(charset.chars().nth(remainder as usize).unwrap());
        num /= base;
    }
    result.push_str(ext);
    result
}

/// Create temporary file
///
/// # Arguments
/// - `ext`: File extension
///
/// # Returns
/// - On success, returns temporary file path string
/// - On failure, returns an error
pub async fn mktempfile(ext: &str) -> AnyResult<String> {
    let dir_path_str = mktempdir()
        .await
        .context("Failed to create temp directory for temp file")?;
    let filename = random_file_name(ext);

    let p = PathBuf::from(dir_path_str);
    let file_path = p.join(filename);

    tokio::fs::File::create(&file_path)
        .await
        .with_context(|| format!("Failed to create temp file: {}", file_path.display()))?;
    file_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Temporary file path is not valid UTF-8: {:?}", file_path))
}

/// Modify file permissions
///
/// # Arguments
/// - `mode`: Octal permission mode string (e.g. "755")
/// - `file_path`: Path to the file to modify permissions
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn chmod_sync(mode: &str, file_path: &str) -> AnyResult<()> {
    match u32::from_str_radix(mode, 8) {
        Ok(mode) => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = std::fs::Permissions::from_mode(mode);
                if let Err(err) = std::fs::set_permissions(file_path, permissions) {
                    anyhow::bail!("Failed to set file permissions: {}", err)
                }
            }
            #[cfg(windows)]
            {
                let mut permissions = std::fs::metadata(file_path)?.permissions();
                permissions.set_readonly(mode & 0o444 == 0);
                if let Err(err) = std::fs::set_permissions(file_path, permissions) {
                    anyhow::bail!("Failed to set file permissions: {}", err)
                }
            }
            Ok(())
        }
        Err(_) => anyhow::bail!("Invalid mode: {}", mode),
    }
}

/// Create symlink
///
/// # Arguments
/// - `o`: Source file or directory path
/// - `l`: Link file path
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
pub fn soft_link(o: &str, l: &str) -> AnyResult<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(o, l)
            .with_context(|| format!("Failed to create symlink from {o} to {l}"))?;
    }
    #[cfg(windows)]
    {
        // For Windows, symlink_file is for files, symlink_dir for directories.
        // This function doesn't know if 'o' is a file or dir.
        // std::fs::symlink (stabilized in 1.78) might be better if available, or choose based on 'o's type.
        // Assuming 'o' is a file for now, as per original.
        std::os::windows::fs::symlink_file(o, l)
            .with_context(|| format!("Failed to create file symlink from {o} to {l}"))?;
    }
    Ok(())
}

/// Simplified path handling like Node.js (join and resolve . and ..)
///
/// # Arguments
/// - `base_str`: Base path string
/// - `input_str`: Input path string to resolve
///
/// # Returns
/// - On success, returns resolved path string
/// - On failure, returns an error
pub fn resolve(base_str: &str, input_str: &str) -> Result<String, std::ffi::OsString> {
    let input_path = Path::new(input_str);
    let mut resolved_path: PathBuf;

    if input_path.is_absolute() {
        resolved_path = PathBuf::from(input_path);
    } else {
        resolved_path = PathBuf::from(base_str);
        for component in input_path.components() {
            match component {
                Component::ParentDir => {
                    resolved_path.pop();
                }
                Component::Normal(name) => {
                    resolved_path.push(name);
                }
                Component::CurDir => {
                    // Do nothing for current directory indicators
                }
                Component::RootDir => {
                    // Should not happen if !input_path.is_absolute()
                    resolved_path = PathBuf::from(component.as_os_str());
                }
                Component::Prefix(prefix) => {
                    // Should not happen if !input_path is_absolute()
                    resolved_path = PathBuf::from(prefix.as_os_str());
                }
            }
        }
    }

    // Normalize: remove redundant separators, but PathBuf usually handles this.
    // For a true "resolve" like Node.js, which also makes it absolute using CWD if needed,
    // more logic or `fs::canonicalize` (if path must exist) would be required.
    // This version focuses on joining and simplifying components.

    resolved_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| resolved_path.into_os_string())
}

/// Normalize path by replacing backslashes with forward slashes
///
/// # Arguments
/// - `path`: Path string to normalize
///
/// # Returns
/// - Returns a new string with all backslashes replaced by forward slashes
///
/// # Example
/// ```norun
/// let normalized = afs::normalize_path(r"C:\Users\Example");
/// assert_eq!(normalized, "C:/Users/Example");
/// ```
///
pub fn normalize_path(path: &str) -> String {
    path.replace("\\", "/")
}

/// Get canonicalized file path
///
/// This function first normalizes the path by replacing backslashes (`\`) with forward slashes (`/`),
/// then uses `std::fs::canonicalize` to get the absolute canonical form of the path.
///
/// # Arguments
/// - `path`: File path string to process
///
/// # Returns
/// - On success, returns the canonicalized path string
/// - On failure, returns an error if the path contains invalid Unicode characters or cannot be canonicalized
///
/// # Example
/// ```norun
/// let filepath = afs::get_filepath("C:\\Users\\Example\\file.txt")?;
/// assert_eq!(filepath, "C:/Users/Example/file.txt");
/// ```
///
pub fn get_filepath(path: &str) -> AnyResult<String> {
    // normalize_path first to handle mixed separators before canonicalize
    let normalized_input = normalize_path(path);
    let real_path = std::fs::canonicalize(&normalized_input)
        .with_context(|| format!("Failed to canonicalize path: {normalized_input}"))?;
    real_path
        .to_str()
        .ok_or_else(|| anyhow!("Path contains invalid Unicode characters: {:?}", real_path))
        .map(|s| {
            // 在Windows系统上修复路径前缀
            #[cfg(windows)]
            let s = s.trim_start_matches(r"\\?\");
            // 确保路径分隔符统一
            // normalize_path(s)
            s.to_string()
        })
}

/// Create file if it does not exist. Will create parent directories if they don't exist
///
/// # Arguments
/// - `filepath`: Path to the file to create
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
///
pub fn create_file_sync(filepath: &str) -> AnyResult<()> {
    let path_str_normalized = normalize_path(filepath);
    let path_obj = Path::new(&path_str_normalized);

    if path_obj.exists() {
        return if path_obj.is_file() {
            Ok(())
        } else {
            // 路径存在但不是文件（例如是目录）
            Err(anyhow!(
                "Path {} already exists and is not a file",
                path_obj.display()
            ))
        };
    }

    // 确保父目录存在
    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            create_dir_all(parent).with_context(|| {
                format!("Failed to create parent directory: {}", parent.display())
            })?;
        } else if !parent.is_dir() {
            return Err(anyhow!(
                "Parent path {} for {} is not a directory",
                parent.display(),
                path_obj.display()
            ));
        }
    }

    // 创建文件
    File::create(path_obj) // fs_err::File
        .map_err(|e| anyhow!("Unable to create file {}: {}", path_obj.display(), e))?;
    Ok(())
}

/// Synchronously get file metadata
///
/// # Arguments
/// - `filepath`: Path to the file to get metadata for
///
/// # Returns
/// - On success, returns the file metadata
/// - On failure, returns an error
///
pub fn stat_sync(filepath: &str) -> AnyResult<std::fs::Metadata> {
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(anyhow::anyhow!("file path cannot be empty for stat"));
    }
    std::fs::metadata(&path)
        .map_err(|e: std::io::Error| anyhow::anyhow!("can not read {} metadata: {}", path, e))
}

/// Asynchronously get file metadata
///
/// # Arguments
/// - `filepath`: Path to the file to get metadata for
///
/// # Returns
/// - On success, returns the file metadata
/// - On failure, returns an error
///
pub async fn stat(filepath: &str) -> AnyResult<std::fs::Metadata> {
    // For async stat, consider if get_filepath (which is sync and uses canonicalize) is appropriate.
    // tokio::fs::metadata itself doesn't canonicalize but handles paths directly.
    // If canonicalization is desired, an async equivalent or careful handling is needed.
    // For now, assuming get_filepath's behavior (path must exist) is acceptable for `stat`.
    let path = get_filepath(filepath)
        .with_context(|| format!("Failed to get filepath for stat: {filepath}"))?;
    tokio::fs::metadata(&path)
        .await
        .with_context(|| format!("Unable to get metadata for file {path}:"))
}

/// Synchronously check if file or directory exists
///
/// # Arguments
/// - `filepath`: Path to check
///
/// # Returns
/// - `true` if the path exists
/// - `false` otherwise
///
pub fn exists_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized).is_ok()
}

/// Asynchronously check if file or directory exists
///
/// # Arguments
/// - `filepath`: Path to check
///
/// # Returns
/// - `true` if the path exists
/// - `false` otherwise
///
pub async fn exists(filepath: &str) -> bool {
    // normalize_path is not strictly necessary for tokio::fs::metadata, but can be good for consistency.
    // let path_str_normalized = normalize_path(filepath);
    tokio::fs::metadata(filepath).await.is_ok()
}

/// Synchronously check if path is a file
///
/// # Arguments
/// - `filepath`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a file
/// - `false` otherwise
///
pub fn is_file_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// Synchronously check if path is a directory
///
/// # Arguments
/// - `filepath`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a directory
/// - `false` otherwise
///
pub fn is_dir_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// Synchronously check if path is a symbolic link
///
/// # Arguments
/// - `filepath`: Path to check
///
/// # Returns
/// - `true` if the path exists and is a symbolic link
/// - `false` otherwise
///
pub fn is_symlink_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::symlink_metadata(path_str_normalized) // Use std::fs::symlink_metadata
        .map(|metadata| metadata.file_type().is_symlink()) // metadata.is_symlink() is on FileType
        .unwrap_or(false)
}

/// Synchronously calculate SHA256 hash of a file
///
/// # Arguments
/// - `filepath`: Path to the file to hash
///
/// # Returns
/// - On success, returns the hexadecimal hash string
/// - On failure, returns an error
///
pub fn hash_sync(filepath: &str) -> AnyResult<String> {
    use sha2::{Digest, Sha256};
    // get_filepath requires path to exist, which is suitable for hashing.
    let path = get_filepath(filepath)
        .with_context(|| format!("Failed to get filepath for hashing: {filepath}"))?;
    if path.is_empty() {
        // This check is somewhat redundant if get_filepath succeeds
        return Err(anyhow!("File path cannot be empty for hashing"));
    }
    let mut file = std::fs::File::open(&path)
        .map_err(|e: std::io::Error| anyhow!("Unable to open file {}: {}", path, e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; // 8KB buffer
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .with_context(|| format!("Failed to read from file for hashing: {path}"))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Asynchronously calculate SHA256 hash of a file
///
/// # Arguments
/// - `filepath`: Path to the file to hash
///
/// # Returns
/// - On success, returns the hexadecimal hash string
/// - On failure, returns an error
///
pub async fn hash(filepath: &str) -> AnyResult<String> {
    use sha2::{Digest, Sha256};
    // get_filepath requires path to exist.
    let path = get_filepath(filepath)
        .with_context(|| format!("Failed to get filepath for async hashing: {filepath}"))?;
    if path.is_empty() {
        return Err(anyhow!("File path cannot be empty for async hashing"));
    }
    let mut file = tokio::fs::File::open(&path)
        .await
        .map_err(|e: std::io::Error| anyhow!("Unable to open file {}: {}", path, e))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192]; // 8KB buffer
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .await
            .with_context(|| format!("Failed to read from file for async hashing: {path}"))?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Find the full path of a command in the PATH environment variable
///
/// # Arguments
/// - `command`: Name of the command to find
///
/// # Returns
/// - On success, returns the full path to the command
/// - On failure, returns an error if command not found
///
pub fn which(command: &str) -> AnyResult<String> {
    let paths_var = env::var("PATH").unwrap_or_default();
    for path_dir_osstr in env::split_paths(&paths_var) {
        let mut full_path = path_dir_osstr;
        full_path.push(command);
        if full_path.is_file() {
            // Consider also checking for executable permissions on Unix.
            return full_path.to_str().map(|s| s.to_string()).ok_or_else(|| {
                anyhow!(
                    "Command path found but contains invalid Unicode: {:?}",
                    full_path
                )
            });
        }
    }
    Err(anyhow!("Command '{}' not found in PATH", command))
}

/// Synchronously delete a file
///
/// # Arguments
/// - `filepath`: Path to the file to delete
///
/// # Returns
/// - On success, returns `Ok(())`
/// - On failure, returns an error
///
pub fn unlink_sync(filepath: &str) -> AnyResult<()> {
    // get_filepath requires path to exist, suitable for unlink.
    let path = get_filepath(filepath)
        .with_context(|| format!("Failed to get filepath for unlink: {filepath}"))?;
    if path.is_empty() {
        // Redundant if get_filepath succeeds
        return Err(anyhow!("File path cannot be empty for unlink"));
    }
    std::fs::remove_file(path).with_context(|| format!("Failed to remove file: {filepath}"))?; // Use original filepath in context
    Ok(())
}

/// Get the basename of a path (filename with extension)
///
/// # Arguments
/// - `path_str`: Path string
///
/// # Returns
/// - On success, returns the basename
/// - On failure, returns an error
///
pub fn basename(path_str: &str) -> AnyResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(anyhow!("Path cannot be empty for basename"));
    }
    Path::new(&normalized_path)
        .file_name()
        .ok_or_else(|| anyhow!("Unable to get basename for path: {}", normalized_path))?
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Path basename contains invalid Unicode characters"))
}

/// Get the filename of a path (with extension)
///
/// # Arguments
/// - `path_str`: Path string
///
/// # Returns
/// - On success, returns the filename
/// - On failure, returns an error
///
pub fn filename(path_str: &str) -> AnyResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(anyhow!("Path cannot be empty for filename"));
    }
    Path::new(&normalized_path)
        .file_name() // This includes the extension.
        .ok_or_else(|| anyhow!("Unable to get filename for path: {}", normalized_path))?
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Path filename contains invalid Unicode characters"))
}

/// Get the directory portion of a path
///
/// # Arguments
/// - `path_str`: Path string
///
/// # Returns
/// - On success, returns the directory path
/// - On failure, returns an error
///
pub fn dirname(path_str: &str) -> AnyResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(anyhow!("Path cannot be empty for dirname"));
    }
    Path::new(&normalized_path)
        .parent()
        .ok_or_else(|| anyhow!("Unable to get dirname for path: {}", normalized_path))?
        .to_str()
        .map(|s| {
            if s.is_empty() {
                ".".to_string()
            } else {
                s.to_string()
            }
        })
        .ok_or_else(|| anyhow!("Path dirname contains invalid Unicode characters"))
}
