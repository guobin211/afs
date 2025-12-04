# afs

[![Crates.io](https://img.shields.io/crates/v/afs.svg)](https://crates.io/crates/afs)
[![Documentation](https://docs.rs/afs/badge.svg)](https://docs.rs/afs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

一个简单强大的 Rust 文件系统库，灵感来自 Node.js fs 模块。

[English](./README.md)

## 特性

- 支持异步/同步 API
- 简洁的错误处理 `AfsError`
- JSON 文件操作
- 路径工具（basename, dirname, normalize）
- 文件哈希计算（SHA256）
- 临时文件/目录创建
- 跨平台支持

## 安装

```toml
[dependencies]
afs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## 快速开始

```rust
use afs::*;

#[tokio::main]
async fn main() -> AfsResult<()> {
    // 写入文件
    write_file("hello.txt", "Hello, World!").await?;
    
    // 读取文件
    let content = read_file("hello.txt").await?;
    println!("{}", content);
    
    // 检查文件是否存在
    if file_exists("hello.txt").await {
        println!("文件存在！");
    }
    
    Ok(())
}
```

## API 参考

### 文件操作

| 函数               | 描述                     |
| ------------------ | ------------------------ |
| `read_file`        | 异步读取文件内容到字符串 |
| `read_file_sync`   | 同步读取文件内容到字符串 |
| `write_file`       | 异步写入字符串到文件     |
| `write_file_sync`  | 同步写入字符串到文件     |
| `append_file`      | 异步追加字符串到文件     |
| `append_file_sync` | 同步追加字符串到文件     |
| `create_file_sync` | 创建文件并自动创建父目录 |
| `unlink_sync`      | 同步删除文件             |

### 目录操作

| 函数         | 描述         |
| ------------ | ------------ |
| `mkdir`      | 异步创建目录 |
| `mkdir_sync` | 同步创建目录 |
| `rmdir`      | 异步删除目录 |
| `rmdir_sync` | 同步删除目录 |

### JSON 操作

| 函数                | 描述                  |
| ------------------- | --------------------- |
| `read_from_json<T>` | 读取 JSON 文件到结构体|
| `read_json`         | 读取 JSON 文件到 Value|
| `write_to_json<T>`  | 写入结构体到 JSON 文件|

### 检查函数

| 函数              | 描述                     |
| ----------------- | ------------------------ |
| `file_exists`     | 异步检查文件是否存在     |
| `dir_exists`      | 异步检查目录是否存在     |
| `exists`          | 异步检查路径是否存在     |
| `exists_sync`     | 同步检查路径是否存在     |
| `is_file`         | 异步检查是否为文件       |
| `is_file_sync`    | 同步检查是否为文件       |
| `is_dir`          | 异步检查是否为目录       |
| `is_dir_sync`     | 同步检查是否为目录       |
| `is_symlink`      | 异步检查是否为符号链接   |
| `is_symlink_sync` | 同步检查是否为符号链接   |

### 元数据函数

| 函数                 | 描述                   |
| -------------------- | ---------------------- |
| `get_file_size`      | 获取文件大小（字节）   |
| `get_file_real_size` | 获取软链接文件实际大小 |
| `get_dir_size`       | 获取目录总大小         |
| `stat`               | 异步获取文件元数据     |
| `stat_sync`          | 同步获取文件元数据     |

### 系统函数

| 函数        | 描述                       |
| ----------- | -------------------------- |
| `diskusage` | 获取磁盘使用情况           |
| `which`     | 在 PATH 环境变量中查找命令 |

### 临时文件/目录

| 函数             | 描述         |
| ---------------- | ------------ |
| `create_tempdir` | 创建临时目录 |
| `create_tempfile`| 创建临时文件 |

### 权限和链接

| 函数         | 描述         |
| ------------ | ------------ |
| `chmod_sync` | 修改文件权限 |
| `soft_link`  | 创建软链接   |

### 路径工具

| 函数             | 描述                         |
| ---------------- | ---------------------------- |
| `resolve`        | 类似 Node.js 的路径解析      |
| `normalize_path` | 将反斜杠替换为正斜杠         |
| `get_filepath`   | 获取规范化的文件路径         |
| `basename`       | 获取文件名                   |
| `filename`       | 获取文件名（含扩展名）       |
| `dirname`        | 获取目录部分                 |

### 哈希函数

| 函数        | 描述                    |
| ----------- | ----------------------- |
| `hash`      | 异步计算 SHA256 哈希值  |
| `hash_sync` | 同步计算 SHA256 哈希值  |

## 示例

### 读写 JSON

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
    
    // 写入 JSON
    write_to_json("config.json", &config).await?;
    
    // 读取 JSON
    let loaded: Config = read_from_json("config.json").await?;
    println!("名称: {}, 版本: {}", loaded.name, loaded.version);
    
    Ok(())
}
```

### 创建临时文件

```rust
use afs::*;

#[tokio::main]
async fn main() -> AfsResult<()> {
    // 创建临时目录
    let temp_dir = create_tempdir().await?;
    println!("临时目录: {}", temp_dir);
    
    // 创建指定扩展名的临时文件
    let temp_file = create_tempfile(".txt").await?;
    println!("临时文件: {}", temp_file);
    
    Ok(())
}
```

### 路径操作

```rust
use afs::*;

fn main() -> AfsResult<()> {
    let path = "/home/user/documents/file.txt";
    
    println!("basename: {}", basename(path)?);  // file.txt
    println!("dirname: {}", dirname(path)?);    // /home/user/documents
    println!("filename: {}", filename(path)?);  // file.txt
    
    // 解析相对路径
    let resolved = resolve("/home/user", "../test.txt")?;
    println!("resolved: {}", resolved);  // /home/test.txt
    
    Ok(())
}
```

## 许可证

MIT License

