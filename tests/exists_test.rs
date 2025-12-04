use afs::*;

#[test]
fn test_exists_sync() {
    let path = "test_exists.txt";
    std::fs::write(path, "test").unwrap();

    assert!(exists_sync(path));
    assert!(!exists_sync("nonexistent.txt"));

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_exists() {
    let path = "test_exists_async.txt";
    tokio::fs::write(path, "test").await.unwrap();

    assert!(exists(path).await);
    assert!(!exists("nonexistent.txt").await);

    tokio::fs::remove_file(path).await.unwrap();
}

#[test]
fn test_is_file_sync() {
    let file_path = "test_is_file.txt";
    let dir_path = "test_is_file_dir";

    std::fs::write(file_path, "test").unwrap();
    std::fs::create_dir(dir_path).unwrap();

    assert!(is_file_sync(file_path));
    assert!(!is_file_sync(dir_path));
    assert!(!is_file_sync("nonexistent.txt"));

    std::fs::remove_file(file_path).unwrap();
    std::fs::remove_dir(dir_path).unwrap();
}

#[tokio::test]
async fn test_is_file() {
    let path = "test_is_file_async.txt";
    tokio::fs::write(path, "test").await.unwrap();

    assert!(is_file(path).await);
    assert!(!is_file("nonexistent.txt").await);

    tokio::fs::remove_file(path).await.unwrap();
}

#[test]
fn test_is_dir_sync() {
    let file_path = "test_is_dir.txt";
    let dir_path = "test_is_dir_dir";

    std::fs::write(file_path, "test").unwrap();
    std::fs::create_dir(dir_path).unwrap();

    assert!(!is_dir_sync(file_path));
    assert!(is_dir_sync(dir_path));
    assert!(!is_dir_sync("nonexistent"));

    std::fs::remove_file(file_path).unwrap();
    std::fs::remove_dir(dir_path).unwrap();
}

#[tokio::test]
async fn test_is_dir() {
    let path = "test_is_dir_async";
    tokio::fs::create_dir(path).await.unwrap();

    assert!(is_dir(path).await);
    assert!(!is_dir("nonexistent").await);

    tokio::fs::remove_dir(path).await.unwrap();
}

#[tokio::test]
async fn test_file_exists() {
    let path = "test_file_exists.txt";
    tokio::fs::write(path, "test").await.unwrap();

    assert!(file_exists(path).await);
    assert!(!file_exists("nonexistent.txt").await);

    tokio::fs::remove_file(path).await.unwrap();
}

#[tokio::test]
async fn test_dir_exists() {
    let path = "test_dir_exists";
    tokio::fs::create_dir(path).await.unwrap();

    assert!(dir_exists(path).await);
    assert!(!dir_exists("nonexistent").await);

    tokio::fs::remove_dir(path).await.unwrap();
}

#[test]
fn test_is_symlink_sync() {
    #[cfg(unix)]
    {
        let target = "test_symlink_target.txt";
        let link = "test_symlink.txt";
        std::fs::write(target, "content").unwrap();
        std::os::unix::fs::symlink(target, link).unwrap();

        assert!(is_symlink_sync(link));
        assert!(!is_symlink_sync(target));

        std::fs::remove_file(link).unwrap();
        std::fs::remove_file(target).unwrap();
    }
}

#[tokio::test]
async fn test_is_symlink() {
    #[cfg(unix)]
    {
        let target = "test_symlink_async_target.txt";
        let link = "test_symlink_async.txt";
        std::fs::write(target, "content").unwrap();
        std::os::unix::fs::symlink(target, link).unwrap();

        assert!(is_symlink(link).await);
        assert!(!is_symlink(target).await);

        std::fs::remove_file(link).unwrap();
        std::fs::remove_file(target).unwrap();
    }
}

#[test]
fn test_which() {
    #[cfg(unix)]
    {
        let result = which("ls");
        assert!(result.is_ok());

        let result = which("nonexistent_command_12345");
        assert!(result.is_err());
    }

    #[cfg(windows)]
    {
        let result = which("cmd.exe");
        assert!(result.is_ok());
    }
}

