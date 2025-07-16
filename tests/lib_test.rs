#[cfg(test)]
mod tests {
    use afs::*;

    #[test]
    fn test_write_to_file() {
        let path = "test.txt";
        let content = "Hello, world!";

        // Write to file
        write_file_sync(path, content).unwrap();

        // Read the file and check the content
        let read_content = std::fs::read_to_string(path).unwrap();
        assert_eq!(read_content, content);

        // Clean up
        std::fs::remove_file(path).unwrap();
    }

    #[tokio::test]
    async fn test_read_json() {
        let file_path = "test.json";
        let content = r#"{"name": "test", "age": 30}"#;
        tokio::fs::write(file_path, content)
            .await
            .expect("Failed to write test file");
        let result: serde_json::Value = read_from_json(file_path)
            .await
            .expect("Failed to read json file");
        assert_eq!(result["name"], "test");
        tokio::fs::remove_file(file_path)
            .await
            .expect("failed to remove test file");
    }

    #[tokio::test]
    async fn test_get_dir_size() {
        let current_dir = std::env::current_dir().unwrap();
        let dir_path = current_dir.parent().unwrap();
        println!("dir path: {dir_path:?}");
        match get_dir_size(dir_path.to_str().unwrap()).await {
            Ok(x) => {
                println!("dir size: {x} bytes");
            }
            Err(err) => {
                eprintln!("Failed to get dir size : {err:?}");
            }
        };
    }

    #[tokio::test]
    async fn test_diskusage() {
        // 获取磁盘使用情况
        let usage = diskusage().await;
        // 验证函数执行成功
        assert!(usage.is_ok(), "磁盘使用情况获取失败");
        // 获取使用量
        let used_space = usage.unwrap();
        // 验证使用量是正数
        assert!(used_space > 0.0, "磁盘使用量应该大于0");
        // 打印磁盘使用情况
        let used_space_mb = used_space / (1024.0 * 1024.0);
        println!("磁盘已使用空间: {used_space_mb} mb");
    }

    #[test]
    fn test_chmod_sync() {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let path = "test_chmod.txt";
            let content = "测试文件权限";
            // 创建测试文件
            write_file_sync(path, content).unwrap();
            // 修改权限为 755 (rwxr-xr-x)
            chmod_sync("755", path).unwrap();
            // 验证权限是否正确设置
            let metadata = std::fs::metadata(path).unwrap();
            let permissions = metadata.permissions();
            let mode = permissions.mode();
            // 检查权限是否为 755 (0o755 = 493 十进制)
            assert_eq!(mode & 0o777, 0o755);
            // 清理
            std::fs::remove_file(path).unwrap();
        }

        #[cfg(windows)]
        {
            let path = "test_chmod.txt";
            let content = "测试文件权限";
            // 创建测试文件
            write_file_sync(path, content).unwrap();
            // 修改权限为只读
            chmod_sync("777", path).unwrap();
            // 验证权限是否正确设置
            let metadata = std::fs::metadata(path).unwrap();
            let permissions = metadata.permissions();
            assert!(!permissions.readonly());
            // 清理
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn test_resolve() {
        #[cfg(unix)]
        {
            let base = "/home/user";
            let input = "../test.txt";
            let result = resolve(base, input);
            assert_eq!(result.unwrap(), "/home/test.txt");
            let input = "./test.txt";
            let result = resolve(base, input);
            assert_eq!(result.unwrap(), "/home/user/test.txt");
            let input = "test.txt";
            let result = resolve(base, input);
            assert_eq!(result.unwrap(), "/home/user/test.txt");
            let input = "./../test.txt";
            let result = resolve(base, input);
            assert_eq!(result.unwrap(), "/home/test.txt");
        }
    }

    #[tokio::test]
    async fn test_mktempfile() {
        let dir = mktempdir().await.unwrap();
        println!("temp dir: {dir}");
        let file = mktempfile(".txt").await.unwrap();
        println!("temp file: {file}");
        std::fs::remove_file(file).unwrap();
    }
}
