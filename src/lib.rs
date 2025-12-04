use std::{
    env,
    io::{Read, Write},
    path::{Component, Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use serde::Deserialize;
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub use fs_err::*;
pub use fs_extra::*;

#[derive(Error, Debug)]
pub enum AfsError {
    #[error("Failed to read file '{path}': {source}")]
    ReadFile { path: String, source: std::io::Error },

    #[error("Failed to write file '{path}': {source}")]
    WriteFile { path: String, source: std::io::Error },

    #[error("Failed to create file '{path}': {source}")]
    CreateFile { path: String, source: std::io::Error },

    #[error("Failed to remove file '{path}': {source}")]
    RemoveFile { path: String, source: std::io::Error },

    #[error("Failed to create directory '{path}': {source}")]
    CreateDir { path: String, source: std::io::Error },

    #[error("Failed to remove directory '{path}': {source}")]
    RemoveDir { path: String, source: std::io::Error },

    #[error("Failed to get metadata '{path}': {source}")]
    Metadata { path: String, source: std::io::Error },

    #[error("Failed to parse JSON '{path}': {source}")]
    JsonParse { path: String, source: serde_json::Error },

    #[error("Failed to serialize JSON: {0}")]
    JsonSerialize(#[from] serde_json::Error),

    #[error("Failed to canonicalize path '{path}': {source}")]
    Canonicalize { path: String, source: std::io::Error },

    #[error("Path contains invalid Unicode: {0}")]
    InvalidUnicode(String),

    #[error("Path not found: {0}")]
    PathNotFound(String),

    #[error("Path exists but is not a file: {0}")]
    NotAFile(String),

    #[error("Parent path is not a directory: {0}")]
    ParentNotDir(String),

    #[error("Path is empty")]
    EmptyPath,

    #[error("Invalid permission mode: {0}")]
    InvalidMode(String),

    #[error("Failed to get disk info: {0}")]
    DiskInfo(String),

    #[error("Failed to create temp directory: {0}")]
    TempDir(#[from] std::io::Error),

    #[error("Command not found: {0}")]
    CommandNotFound(String),

    #[error("Cannot get path component: {0}")]
    PathComponent(String),
}

pub type AfsResult<T> = Result<T, AfsError>;

pub type AnyResult<T> = AfsResult<T>;

pub async fn read_file(path: &str) -> AfsResult<String> {
    tokio::fs::read_to_string(path)
        .await
        .map_err(|e| AfsError::ReadFile { path: path.to_string(), source: e })
}

pub fn read_file_sync(path: &str) -> AfsResult<String> {
    std::fs::read_to_string(path)
        .map_err(|e| AfsError::ReadFile { path: path.to_string(), source: e })
}

pub fn write_file_sync(path: &str, content: &str) -> AfsResult<()> {
    let mut file = std::fs::File::create(path)
        .map_err(|e| AfsError::CreateFile { path: path.to_string(), source: e })?;
    file.write_all(content.as_bytes())
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })
}

pub async fn write_file(path: &str, content: &str) -> AfsResult<()> {
    let mut file = tokio::fs::File::create(path)
        .await
        .map_err(|e| AfsError::CreateFile { path: path.to_string(), source: e })?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })
}

pub fn append_file_sync(path: &str, content: &str) -> AfsResult<()> {
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })?;
    file.write_all(content.as_bytes())
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })
}

pub async fn append_file(path: &str, content: &str) -> AfsResult<()> {
    let mut file = tokio::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .await
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })?;
    file.write_all(content.as_bytes())
        .await
        .map_err(|e| AfsError::WriteFile { path: path.to_string(), source: e })
}

pub fn mkdir_sync(path: &str) -> AfsResult<()> {
    std::fs::create_dir_all(path)
        .map_err(|e| AfsError::CreateDir { path: path.to_string(), source: e })
}

pub async fn mkdir(path: &str) -> AfsResult<()> {
    tokio::fs::create_dir_all(path)
        .await
        .map_err(|e| AfsError::CreateDir { path: path.to_string(), source: e })
}

pub fn rmdir_sync(path: &str) -> AfsResult<()> {
    std::fs::remove_dir_all(path)
        .map_err(|e| AfsError::RemoveDir { path: path.to_string(), source: e })
}

pub async fn rmdir(path: &str) -> AfsResult<()> {
    tokio::fs::remove_dir_all(path)
        .await
        .map_err(|e| AfsError::RemoveDir { path: path.to_string(), source: e })
}

pub async fn read_from_json<T: for<'a> Deserialize<'a>>(file_path: &str) -> AfsResult<T> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|e| AfsError::ReadFile { path: file_path.to_string(), source: e })?;

    serde_json::from_str::<T>(&content)
        .map_err(|e| AfsError::JsonParse { path: file_path.to_string(), source: e })
}

pub async fn read_json(file_path: &str) -> AfsResult<serde_json::Value> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .map_err(|e| AfsError::ReadFile { path: file_path.to_string(), source: e })?;
    serde_json::from_str(&content)
        .map_err(|e| AfsError::JsonParse { path: file_path.to_string(), source: e })
}

pub async fn write_to_json<T: serde::Serialize>(file_path: &str, data: &T) -> AfsResult<()> {
    let mut file = tokio::fs::File::create(file_path)
        .await
        .map_err(|e| AfsError::CreateFile { path: file_path.to_string(), source: e })?;
    let json = serde_json::to_string_pretty(data)?;
    file.write_all(json.as_bytes())
        .await
        .map_err(|e| AfsError::WriteFile { path: file_path.to_string(), source: e })
}

pub async fn file_exists(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

pub async fn dir_exists(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub async fn is_file(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

pub async fn is_dir(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub async fn is_symlink(path: &str) -> bool {
    tokio::fs::symlink_metadata(path)
        .await
        .map(|metadata| metadata.is_symlink())
        .unwrap_or(false)
}

pub async fn get_file_size(file_path: &str) -> AfsResult<u64> {
    let metadata = tokio::fs::metadata(file_path)
        .await
        .map_err(|e| AfsError::Metadata { path: file_path.to_string(), source: e })?;
    Ok(metadata.len())
}

pub async fn get_file_real_size(file_path: &str) -> AfsResult<u64> {
    let metadata = tokio::fs::metadata(file_path)
        .await
        .map_err(|e| AfsError::Metadata { path: file_path.to_string(), source: e })?;
    Ok(metadata.len())
}

pub async fn get_dir_size(dir_path: &str) -> AfsResult<u64> {
    let mut total_size = 0;
    let mut stack = vec![PathBuf::from(dir_path)];

    while let Some(path) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&path)
            .await
            .map_err(|e| AfsError::Metadata { path: path.display().to_string(), source: e })?;

        while let Some(entry) = entries.next_entry().await.map_err(|e| AfsError::Metadata {
            path: path.display().to_string(),
            source: e
        })? {
            let metadata = entry.metadata().await.map_err(|e| AfsError::Metadata {
                path: entry.path().display().to_string(),
                source: e,
            })?;

            if metadata.is_file() {
                total_size += metadata.len();
            } else if metadata.is_dir() && !metadata.is_symlink() {
                stack.push(entry.path());
            }
        }
    }

    Ok(total_size)
}

pub async fn diskusage() -> AfsResult<f64> {
    let info = sys_info::disk_info().map_err(|e| AfsError::DiskInfo(e.to_string()))?;
    let used = info.total - info.free;
    Ok(used as f64)
}

pub async fn create_tempdir() -> AfsResult<String> {
    let dir = tempfile::TempDir::new()?;
    let path_buf: PathBuf = dir.keep();
    path_buf
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AfsError::InvalidUnicode(path_buf.display().to_string()))
}

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

pub async fn create_tempfile(ext: &str) -> AfsResult<String> {
    let dir_path_str = create_tempdir().await?;
    let filename = random_file_name(ext);
    let file_path = PathBuf::from(dir_path_str).join(filename);

    tokio::fs::File::create(&file_path)
        .await
        .map_err(|e| AfsError::CreateFile { path: file_path.display().to_string(), source: e })?;

    file_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| AfsError::InvalidUnicode(file_path.display().to_string()))
}

pub fn chmod_sync(mode: &str, file_path: &str) -> AfsResult<()> {
    let mode_val = u32::from_str_radix(mode, 8)
        .map_err(|_| AfsError::InvalidMode(mode.to_string()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = std::fs::Permissions::from_mode(mode_val);
        std::fs::set_permissions(file_path, permissions)
            .map_err(|e| AfsError::Metadata { path: file_path.to_string(), source: e })?;
    }
    #[cfg(windows)]
    {
        let mut permissions = std::fs::metadata(file_path)
            .map_err(|e| AfsError::Metadata { path: file_path.to_string(), source: e })?
            .permissions();
        permissions.set_readonly(mode_val & 0o444 == 0);
        std::fs::set_permissions(file_path, permissions)
            .map_err(|e| AfsError::Metadata { path: file_path.to_string(), source: e })?;
    }
    Ok(())
}

pub fn soft_link(o: &str, l: &str) -> AfsResult<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(o, l)
            .map_err(|e| AfsError::CreateFile { path: l.to_string(), source: e })?;
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_file(o, l)
            .map_err(|e| AfsError::CreateFile { path: l.to_string(), source: e })?;
    }
    Ok(())
}

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
                Component::CurDir => {}
                Component::RootDir => {
                    resolved_path = PathBuf::from(component.as_os_str());
                }
                Component::Prefix(prefix) => {
                    resolved_path = PathBuf::from(prefix.as_os_str());
                }
            }
        }
    }

    resolved_path
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| resolved_path.into_os_string())
}

pub fn normalize_path(path: &str) -> String {
    path.replace('\\', "/")
}

pub fn get_filepath(path: &str) -> AfsResult<String> {
    let normalized_input = normalize_path(path);
    let real_path = std::fs::canonicalize(&normalized_input)
        .map_err(|e| AfsError::Canonicalize { path: normalized_input.clone(), source: e })?;
    real_path
        .to_str()
        .map(|s| {
            #[cfg(windows)]
            let s = s.trim_start_matches(r"\\?\");
            s.to_string()
        })
        .ok_or_else(|| AfsError::InvalidUnicode(real_path.display().to_string()))
}

pub fn create_file_sync(filepath: &str) -> AfsResult<()> {
    let path_str_normalized = normalize_path(filepath);
    let path_obj = Path::new(&path_str_normalized);

    if path_obj.exists() {
        return if path_obj.is_file() {
            Ok(())
        } else {
            Err(AfsError::NotAFile(path_obj.display().to_string()))
        };
    }

    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AfsError::CreateDir { path: parent.display().to_string(), source: e })?;
        } else if !parent.is_dir() {
            return Err(AfsError::ParentNotDir(parent.display().to_string()));
        }
    }

    std::fs::File::create(path_obj)
        .map_err(|e| AfsError::CreateFile { path: path_obj.display().to_string(), source: e })?;
    Ok(())
}

pub fn stat_sync(filepath: &str) -> AfsResult<std::fs::Metadata> {
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    std::fs::metadata(&path)
        .map_err(|e| AfsError::Metadata { path, source: e })
}

pub async fn stat(filepath: &str) -> AfsResult<std::fs::Metadata> {
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    tokio::fs::metadata(&path)
        .await
        .map_err(|e| AfsError::Metadata { path, source: e })
}

pub fn exists_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized).is_ok()
}

pub async fn exists(filepath: &str) -> bool {
    tokio::fs::metadata(filepath).await.is_ok()
}

pub fn is_file_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

pub fn is_dir_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

pub fn is_symlink_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::symlink_metadata(path_str_normalized)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
}

pub fn hash_sync(filepath: &str) -> AfsResult<String> {
    use sha2::{Digest, Sha256};
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    let mut file = std::fs::File::open(&path)
        .map_err(|e| AfsError::ReadFile { path: path.clone(), source: e })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|e| AfsError::ReadFile { path: path.clone(), source: e })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub async fn hash(filepath: &str) -> AfsResult<String> {
    use sha2::{Digest, Sha256};
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    let mut file = tokio::fs::File::open(&path)
        .await
        .map_err(|e| AfsError::ReadFile { path: path.clone(), source: e })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 8192];
    loop {
        let bytes_read = file
            .read(&mut buffer)
            .await
            .map_err(|e| AfsError::ReadFile { path: path.clone(), source: e })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn which(command: &str) -> AfsResult<String> {
    let paths_var = env::var("PATH").unwrap_or_default();
    for path_dir_osstr in env::split_paths(&paths_var) {
        let mut full_path = path_dir_osstr;
        full_path.push(command);
        if full_path.is_file() {
            return full_path
                .to_str()
                .map(|s| s.to_string())
                .ok_or_else(|| AfsError::InvalidUnicode(full_path.display().to_string()));
        }
    }
    Err(AfsError::CommandNotFound(command.to_string()))
}

pub fn unlink_sync(filepath: &str) -> AfsResult<()> {
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    std::fs::remove_file(&path)
        .map_err(|e| AfsError::RemoveFile { path, source: e })
}

pub fn basename(path_str: &str) -> AfsResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    Path::new(&normalized_path)
        .file_name()
        .ok_or_else(|| AfsError::PathComponent(normalized_path.clone()))?
        .to_str()
        .map(|s| s.to_string())
        .ok_or(AfsError::InvalidUnicode(normalized_path))
}

pub fn filename(path_str: &str) -> AfsResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    Path::new(&normalized_path)
        .file_name()
        .ok_or_else(|| AfsError::PathComponent(normalized_path.clone()))?
        .to_str()
        .map(|s| s.to_string())
        .ok_or(AfsError::InvalidUnicode(normalized_path))
}

pub fn dirname(path_str: &str) -> AfsResult<String> {
    let normalized_path = normalize_path(path_str);
    if normalized_path.is_empty() {
        return Err(AfsError::EmptyPath);
    }
    Path::new(&normalized_path)
        .parent()
        .ok_or_else(|| AfsError::PathComponent(normalized_path.clone()))?
        .to_str()
        .map(|s| {
            if s.is_empty() {
                ".".to_string()
            } else {
                s.to_string()
            }
        })
        .ok_or(AfsError::InvalidUnicode(normalized_path))
}

