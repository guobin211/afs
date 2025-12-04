use afs::*;

#[test]
fn test_unlink_sync() {
    let path = "test_unlink.txt";
    std::fs::write(path, "to be deleted").unwrap();
    assert!(std::path::Path::new(path).exists());

    unlink_sync(path).unwrap();
    assert!(!std::path::Path::new(path).exists());
}

#[test]
fn test_rmdir_sync() {
    let path = "test_rmdir_sync/nested";
    std::fs::create_dir_all(path).unwrap();
    std::fs::write(format!("{}/file.txt", path), "content").unwrap();

    rmdir_sync("test_rmdir_sync").unwrap();
    assert!(!std::path::Path::new("test_rmdir_sync").exists());
}

#[tokio::test]
async fn test_rmdir() {
    let path = "test_rmdir_async/nested";
    tokio::fs::create_dir_all(path).await.unwrap();
    tokio::fs::write(format!("{}/file.txt", path), "content")
        .await
        .unwrap();

    rmdir("test_rmdir_async").await.unwrap();
    assert!(!std::path::Path::new("test_rmdir_async").exists());
}

#[test]
fn test_unlink_nonexistent() {
    let result = unlink_sync("nonexistent_file_12345.txt");
    assert!(result.is_err());
}

#[test]
fn test_rmdir_nonexistent() {
    let result = rmdir_sync("nonexistent_dir_12345");
    assert!(result.is_err());
}

