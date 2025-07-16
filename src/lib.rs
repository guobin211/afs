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

/// 异步读取文件内容到字符串
///
/// # 参数
/// - `path`: 要读取的文件的路径
///
/// # 返回
/// - 如果成功，返回包含文件内容的字符串
/// - 如果失败，返回一个错误
pub async fn read_file(path: &str) -> AnyResult<String> {
    // 异步读取文件内容到字符串
    let content = tokio::fs::read_to_string(path)
        .await
        .with_context(|| format!("Failed to read file: {path}"))?;
    Ok(content)
}

/// 同步读取文件内容到字符串
pub fn read_file_sync(path: &str) -> AnyResult<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {path}"))?;
    Ok(content)
}

/// 同步写入字符串内容到文件
///
/// # 参数
/// - `path`: 要写入的文件的路径
/// - `content`: 要写入的字符串内容
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub fn write_file_sync(path: &str, content: &str) -> AnyResult<()> {
    // 创建一个新文件，如果文件已存在则覆盖
    let mut file = File::create(path)?;
    // 将内容写入文件
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 异步写入字符串内容到文件
///
/// # 参数
/// - `path`: 要写入的文件的路径
/// - `content`: 要写入的字符串内容
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub async fn write_file(path: &str, content: &str) -> AnyResult<()> {
    // 异步创建一个新文件，如果文件已存在则覆盖
    let mut file = tokio::fs::File::create(path)
        .await
        .with_context(|| format!("Failed to create file: {path}"))?;
    // 异步将内容写入文件
    file.write_all(content.as_bytes())
        .await
        .with_context(|| format!("Failed to write to file: {path}"))?;
    Ok(())
}

/// 同步追加字符串内容到文件
///
/// # 参数
/// - `path`: 要追加内容的文件的路径
/// - `content`: 要追加的字符串内容
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub fn append_file_sync(path: &str, content: &str) -> AnyResult<()> {
    // 以追加模式打开文件
    let mut file = File::options().append(true).open(path)?;
    // 将内容追加到文件
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// 异步追加字符串内容到文件
///
/// # 参数
/// - `path`: 要追加内容的文件的路径
/// - `content`: 要追加的字符串内容
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub async fn append_file(path: &str, content: &str) -> AnyResult<()> {
    // 异步以追加模式打开文件
    let mut file = tokio::fs::OpenOptions::new()
        .append(true)
        .create(true) // Create if it doesn't exist, common for append
        .open(path)
        .await
        .with_context(|| format!("Failed to open file for appending: {path}"))?;
    // 异步将内容追加到文件
    file.write_all(content.as_bytes())
        .await
        .with_context(|| format!("Failed to append to file: {path}"))?;
    Ok(())
}

/// 同步创建目录
///
/// # 参数
/// - `path`: 要创建的目录的路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub fn mkdir_sync(path: &str) -> AnyResult<()> {
    // 同步创建目录，不递归创建
    dir::create(path, false)?;
    Ok(())
}

/// 异步创建目录
///
/// # 参数
/// - `path`: 要创建的目录的路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub async fn mkdir(path: &str) -> AnyResult<()> {
    // 异步递归创建目录
    tokio::fs::create_dir_all(path)
        .await
        .with_context(|| format!("Failed to create directory: {path}"))?;
    Ok(())
}

/// 同步删除目录
///
/// # 参数
/// - `path`: 要删除的目录的路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub fn rmdir_sync(path: &str) -> AnyResult<()> {
    // 同步删除目录
    dir::remove(path)?;
    Ok(())
}

/// 异步删除目录
///
/// # 参数
/// - `path`: 要删除的目录的路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub async fn rmdir(path: &str) -> AnyResult<()> {
    // 异步递归删除目录
    tokio::fs::remove_dir_all(path)
        .await
        .with_context(|| format!("Failed to remove directory: {path}"))?;
    Ok(())
}

/// 读取json文件到结构体，忽略未定义的字段
pub async fn read_from_json<T: for<'a> Deserialize<'a>>(file_path: &str) -> AnyResult<T> {
    // 读取文件内容
    let content = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Failed to read JSON file: {file_path}"))?;

    // 解析json字符串到结构体
    let data: T = serde_json::from_str::<T>(&content)
        .with_context(|| format!("Failed to parse JSON from file: {file_path}"))?;

    Ok(data)
}

/// 读取json文件到serde_json::Value
pub async fn read_json(file_path: &str) -> AnyResult<serde_json::Value> {
    let content = tokio::fs::read_to_string(file_path)
        .await
        .with_context(|| format!("Failed to read JSON file: {file_path}"))?;
    let data: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON from file: {file_path}"))?;
    Ok(data)
}

/// 写入结构体到json文件
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

/// 判断文件是否存在
pub async fn file_exists(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// 判断目录是否存在
pub async fn dir_exists(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// 判断是否是文件
pub async fn is_file(file_path: &str) -> bool {
    tokio::fs::metadata(file_path)
        .await
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// 判断是否是目录
pub async fn is_dir(dir_path: &str) -> bool {
    tokio::fs::metadata(dir_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// 判断是否是符号链接
pub async fn is_symlink(path: &str) -> bool {
    tokio::fs::symlink_metadata(path)
        .await
        .map(|metadata| metadata.is_symlink())
        .unwrap_or(false)
}

/// 获取文件大小
pub async fn get_file_size(file_path: &str) -> AnyResult<u64> {
    let metadata = tokio::fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to get metadata for file size: {file_path}"))?;
    Ok(metadata.len())
}

/// 获取软连接文件大小 (实际指向的文件的大小)
///
/// # 参数
/// - `file_path`: 要获取大小的文件路径
///
/// # 返回
/// - 如果成功，返回文件大小（字节数）
/// - 如果失败，返回一个错误
pub async fn get_file_real_size(file_path: &str) -> AnyResult<u64> {
    // tokio::fs::metadata follows symlinks by default.
    let metadata = tokio::fs::metadata(file_path)
        .await
        .with_context(|| format!("Failed to get metadata for file real size: {file_path}"))?;
    Ok(metadata.len())
}

/// 获取目录大小
///
/// # 参数
/// - `dir_path`: 要计算大小的目录路径
///
/// # 返回
/// - 如果成功，返回目录的总大小（字节数）
/// - 如果失败，返回一个错误
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
                // 检查是否是符号链接，避免无限循环
                if !metadata.is_symlink() {
                    stack.push(entry.path());
                }
            }
        }
    }

    Ok(total_size)
}

/// 获取磁盘使用情况
///
/// # 返回
/// - 如果成功，返回已使用的磁盘空间（字节数）
/// - 如果失败，返回一个错误
pub async fn diskusage() -> AnyResult<f64> {
    use sys_info::disk_info; // Keep use statement localized if only used here
    let info = disk_info().map_err(|e| anyhow!("Failed to get disk info: {}", e))?;
    let used = info.total - info.free;
    Ok(used as f64)
}

/// 创建临时目录
///
/// # 返回
/// - 如果成功，返回临时目录的路径字符串
/// - 如果失败，返回一个错误
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

/// 生成基于时间戳的随机文件名
///
/// # 参数
/// - `ext`: 文件扩展名
///
/// # 返回
/// - 返回一个包含时间戳和指定扩展名的随机文件名
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

/// 创建临时文件
///
/// # 参数
/// - `ext`: 文件扩展名
///
/// # 返回
/// - 如果成功，返回临时文件的路径字符串
/// - 如果失败，返回一个错误
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

/// 修改文件权限
///
/// # 参数
/// - `mode`: 八进制权限模式字符串（如 "755"）
/// - `file_path`: 要修改权限的文件路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
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

/// 创建软链接
///
/// # 参数
/// - `o`: 源文件或目录的路径
/// - `l`: 链接文件的路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
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

/// 按Node.js的方式处理路径（简化版：连接路径并处理 . 和 ..）
///
/// # 参数
/// - `base_str`: 基础路径字符串
/// - `input_str`: 要解析的输入路径字符串
///
/// # 返回
/// - 如果成功，返回解析后的路径字符串
/// - 如果路径包含无效的Unicode字符，返回错误
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

/// 将路径中的反斜杠 (`\`) 替换为正斜杠 (`/`)。
///
/// # 参数
/// - `path`: 要规范化的路径字符串。
///
/// # 返回
/// - 返回一个新的字符串，其中所有的反斜杠 (`\`) 都被替换为正斜杠 (`/`)。
///
/// # 示例
/// ```
/// let normalized = afs::normalize_path(r"C:\Users\Example");
/// assert_eq!(normalized, "C:/Users/Example");
/// ```
pub fn normalize_path(path: &str) -> String {
    path.replace("\\", "/")
}

/// 获取文件的规范化路径。
///
/// 此函数首先对路径进行规范化处理，将路径中的反斜杠 (`\`) 替换为正斜杠 (`/`)，
/// 然后使用 `std::fs::canonicalize` 获取路径的绝对规范化形式。
///
/// # 参数
/// - `path`: 要处理的文件路径字符串。
///
/// # 返回
/// - 如果成功，返回规范化后的路径字符串。
/// - 如果路径包含无效的 Unicode 字符或无法规范化路径，则返回错误。
///
/// # 示例
/// ```no-run
/// let filepath = afs::get_filepath("C:\\Users\\Example\\file.txt")?;
/// assert_eq!(filepath, "C:/Users/Example/file.txt");
/// ```
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

/// 创建文件（如果尚不存在）。如果路径中的父目录不存在，则会尝试创建它们。
///
/// # 参数
/// - `filepath`: 要创建的文件路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
pub fn create_file_sync(filepath: &str) -> AnyResult<()> {
    let path_str_normalized = normalize_path(filepath);
    let path_obj = Path::new(&path_str_normalized);

    if path_obj.exists() {
        return if path_obj.is_file() {
            Ok(()) // 文件已存在
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

/// 同步获取文件的元数据信息
///
/// # 参数
/// - `filepath`: 要获取元数据的文件路径
///
/// # 返回
/// - 如果成功，返回文件的元数据
/// - 如果失败，返回一个错误
pub fn stat_sync(filepath: &str) -> AnyResult<std::fs::Metadata> {
    let path = get_filepath(filepath)?;
    if path.is_empty() {
        return Err(anyhow::anyhow!("文件路径不能为空"));
    }
    std::fs::metadata(&path)
        .map_err(|e: std::io::Error| anyhow::anyhow!("无法获取文件 {} 的元数据: {}", path, e))
}

/// 异步获取文件的元数据信息
///
/// # 参数
/// - `filepath`: 要获取元数据的文件路径
///
/// # 返回
/// - 如果成功，返回文件的元数据
/// - 如果失败，返回一个错误
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

/// 同步检查文件或目录是否存在
///
/// # 参数
/// - `filepath`: 要检查的文件或目录路径
///
/// # 返回
/// - 如果存在，返回 `true`
/// - 如果不存在，返回 `false`
pub fn exists_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized).is_ok()
}

/// 异步检查文件或目录是否存在
///
/// # 参数
/// - `filepath`: 要检查的文件或目录路径
///
/// # 返回
/// - 如果存在，返回 `true`
/// - 如果不存在，返回 `false`
pub async fn exists(filepath: &str) -> bool {
    // normalize_path is not strictly necessary for tokio::fs::metadata, but can be good for consistency.
    // let path_str_normalized = normalize_path(filepath);
    tokio::fs::metadata(filepath).await.is_ok()
}

/// 同步检查指定路径是否为文件
///
/// # 参数
/// - `filepath`: 要检查的路径
///
/// # 返回
/// - 如果是文件，返回 `true`
/// - 如果不是文件或路径不存在，返回 `false`
pub fn is_file_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

/// 同步检查指定路径是否为目录
///
/// # 参数
/// - `filepath`: 要检查的路径
///
/// # 返回
/// - 如果是目录，返回 `true`
/// - 如果不是目录或路径不存在，返回 `false`
pub fn is_dir_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::metadata(path_str_normalized)
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
}

/// 同步检查指定路径是否为符号链接
///
/// # 参数
/// - `filepath`: 要检查的路径
///
/// # 返回
/// - 如果是符号链接，返回 `true`
/// - 如果不是符号链接或路径不存在，返回 `false`
pub fn is_symlink_sync(filepath: &str) -> bool {
    let path_str_normalized = normalize_path(filepath);
    std::fs::symlink_metadata(path_str_normalized) // Use std::fs::symlink_metadata
        .map(|metadata| metadata.file_type().is_symlink()) // metadata.is_symlink() is on FileType
        .unwrap_or(false)
}

/// 同步计算文件的SHA256哈希值
///
/// # 参数
/// - `filepath`: 要计算哈希值的文件路径
///
/// # 返回
/// - 如果成功，返回文件的十六进制哈希值字符串
/// - 如果失败，返回一个错误
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

/// 异步计算文件的SHA256哈希值
///
/// # 参数
/// - `filepath`: 要计算哈希值的文件路径
///
/// # 返回
/// - 如果成功，返回文件的十六进制哈希值字符串
/// - 如果失败，返回一个错误
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

/// 在PATH环境变量中查找指定命令的完整路径
///
/// # 参数
/// - `command`: 要查找的命令名称
///
/// # 返回
/// - 如果找到命令，返回命令的完整路径
/// - 如果未找到命令，返回一个错误
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

/// 同步删除文件
///
/// # 参数
/// - `filepath`: 要删除的文件路径
///
/// # 返回
/// - 如果成功，返回 `Ok(())`
/// - 如果失败，返回一个错误
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

/// 获取路径的基本文件名（包含扩展名）
///
/// # 参数
/// - `path_str`: 文件路径字符串
///
/// # 返回
/// - 如果成功，返回文件的基本名称
/// - 如果失败，返回一个错误
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

/// 获取路径的文件名（包含扩展名）
///
/// # 参数
/// - `path_str`: 文件路径字符串
///
/// # 返回
/// - 如果成功，返回文件名
/// - 如果失败，返回一个错误
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

/// 获取路径的目录部分
///
/// # 参数
/// - `path_str`: 文件路径字符串
///
/// # 返回
/// - 如果成功，返回目录路径
/// - 如果失败，返回一个错误
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
