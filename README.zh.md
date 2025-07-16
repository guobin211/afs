# afs

The first choice of fs library for Rust

[English](./README.md)

## afs 库函数快速索引表

## 文件操作函数

| 函数名                | 功能描述                 |
|--------------------|----------------------|
| `read_file_sync`   | 同步读取文件内容到字符串         |
| `read_file`        | 异步读取文件内容到字符串         |
| `write_file_sync`  | 同步写入字符串内容到文件         |
| `write_file`       | 异步写入字符串内容到文件         |
| `append_file_sync` | 同步追加字符串内容到文件         |
| `append_file`      | 异步追加字符串内容到文件         |
| `create_file_sync` | 创建文件（如果不存在），创建必要的父目录 |
| `unlink_sync`      | 同步删除文件               |

## 目录操作函数

| 函数名          | 功能描述         |
|--------------|--------------|
| `mkdir_sync` | 同步创建目录（不递归）  |
| `mkdir`      | 异步创建目录（递归创建） |
| `rmdir_sync` | 同步删除目录       |
| `rmdir`      | 异步删除目录（递归删除） |

## JSON 文件操作函数

| 函数名                 | 功能描述           |
|---------------------|----------------|
| `read_from_json<T>` | 读取JSON文件到结构体   |
| `read_json`         | 读取JSON文件到Value |
| `write_to_json<T>`  | 写入结构体到JSON文件   |

## 文件/目录检查函数

| 函数名               | 功能描述            |
|-------------------|-----------------|
| `file_exists`     | 判断文件是否存在        |
| `dir_exists`      | 判断目录是否存在        |
| `exists_sync`     | 同步检查文件或目录是否存在   |
| `exists`          | 异步检查文件或目录是否存在   |
| `is_file`         | 判断是否是文件         |
| `is_dir`          | 判断是否是目录         |
| `is_symlink`      | 判断是否是符号链接       |
| `is_file_sync`    | 同步检查指定路径是否为文件   |
| `is_dir_sync`     | 同步检查指定路径是否为目录   |
| `is_symlink_sync` | 同步检查指定路径是否为符号链接 |

## 文件大小和元数据函数

| 函数名                  | 功能描述         |
|----------------------|--------------|
| `get_file_size`      | 获取文件大小       |
| `get_file_real_size` | 获取软链接文件的实际大小 |
| `get_dir_size`       | 获取目录大小       |
| `stat_sync`          | 同步获取文件的元数据信息 |
| `stat`               | 异步获取文件的元数据信息 |

## 系统和磁盘函数

| 函数名         | 功能描述             |
|-------------|------------------|
| `diskusage` | 获取磁盘使用情况         |
| `which`     | 在PATH环境变量中查找指定命令 |

## 临时文件和目录函数

| 函数名          | 功能描述   |
|--------------|--------|
| `mktempdir`  | 创建临时目录 |
| `mktempfile` | 创建临时文件 |

## 权限和链接函数

| 函数名          | 功能描述   |
|--------------|--------|
| `chmod_sync` | 修改文件权限 |
| `soft_link`  | 创建软链接  |

## 路径处理函数

| 函数名              | 功能描述            |
|------------------|-----------------|
| `resolve`        | 按Node.js方式处理路径  |
| `normalize_path` | 将路径中的反斜杠替换为正斜杠  |
| `get_filepath`   | 获取文件的规范化路径      |
| `basename`       | 获取路径的基本文件名      |
| `filename`       | 获取路径的文件名（包含扩展名） |
| `dirname`        | 获取路径的目录部分       |

## 哈希函数

| 函数名         | 功能描述             |
|-------------|------------------|
| `hash_sync` | 同步计算文件的SHA256哈希值 |
| `hash`      | 异步计算文件的SHA256哈希值 |

## 按功能分类的快速查找

### 📁 文件读写

- 读取：`read_file`
- 写入：`write_file`, `write_file_sync`
- 追加：`append_file`, `append_file_sync`
- 创建：`create_file_sync`
- 删除：`unlink_sync`

### 📂 目录操作

- 创建：`mkdir`, `mkdir_sync`
- 删除：`rmdir`, `rmdir_sync`

### 🔍 存在性检查

- 文件：`file_exists`, `is_file`, `is_file_sync`
- 目录：`dir_exists`, `is_dir`, `is_dir_sync`
- 符号链接：`is_symlink`, `is_symlink_sync`
- 通用：`exists`, `exists_sync`

### 📊 大小和信息

- 文件大小：`get_file_size`, `get_file_real_size`
- 目录大小：`get_dir_size`
- 元数据：`stat`, `stat_sync`
- 磁盘使用：`diskusage`

### 🛣️ 路径处理

- 规范化：`normalize_path`, `get_filepath`
- 解析：`resolve`
- 提取：`basename`, `filename`, `dirname`

### 🔧 系统功能

- 临时文件：`mktempdir`, `mktempfile`
- 权限：`chmod_sync`
- 链接：`soft_link`
- 命令查找：`which`
- 哈希：`hash`, `hash_sync`

### 📋 JSON操作

- 读取：`read_json`, `read_from_json`
- 写入：`write_to_json`

## 使用说明

### 异步 vs 同步

- 带 `_sync` 后缀的函数是同步版本
- 不带后缀的函数通常是异步版本（需要 `.await`）
- 选择原则：
    - 在异步环境中优先使用异步版本
    - 在同步环境或简单脚本中使用同步版本

### 错误处理

- 大部分函数返回 `AnyResult<T>`，需要进行错误处理
- 检查类函数（如 `exists`, `is_file` 等）直接返回 `bool`

### 路径格式

- 支持跨平台路径（自动处理 Windows 和 Unix 路径分隔符）
- `normalize_path` 可以统一路径格式

### 示例用法

```rust
use afs::*;

// 异步文件操作
async fn example() -> AnyResult<()> {
    // 读取文件
    let content = read_file("test.txt").await?;

    // 写入文件
    write_file("output.txt", &content).await?;

    // 检查文件是否存在
    if file_exists("output.txt").await {
        println!("文件创建成功");
    }

    // 获取文件大小
    let size = get_file_size("output.txt").await?;
    println!("文件大小: {} 字节", size);

    Ok(())
}

// 同步操作
fn sync_example() -> AnyResult<()> {
    // 创建目录
    mkdir_sync("new_dir")?;

    // 创建文件
    create_file_sync("new_dir/test.txt")?;

    // 写入内容
    write_file_sync("new_dir/test.txt", "Hello, World!")?;

    // 获取路径信息
    let dir = dirname("new_dir/test.txt")?;
    let name = basename("new_dir/test.txt")?;

    println!("目录: {}, 文件名: {}", dir, name);

    Ok(())
}
```
