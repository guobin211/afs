use afs::*;

#[test]
fn test_write_file_sync() {
    let path = "test_write_sync.txt";

    write_file_sync(path, "Hello, world!").unwrap();

    let content = std::fs::read_to_string(path).unwrap();
    assert_eq!(content, "Hello, world!");

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_write_file() {
    let path = "test_write_async.txt";

    write_file(path, "Hello, async!").await.unwrap();

    let content = tokio::fs::read_to_string(path).await.unwrap();
    assert_eq!(content, "Hello, async!");

    tokio::fs::remove_file(path).await.unwrap();
}

#[test]
fn test_append_file_sync() {
    let path = "test_append_sync.txt";
    write_file_sync(path, "Hello").unwrap();
    append_file_sync(path, ", World!").unwrap();

    let content = std::fs::read_to_string(path).unwrap();
    assert_eq!(content, "Hello, World!");

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_append_file() {
    let path = "test_append_async.txt";
    write_file(path, "Hello").await.unwrap();
    append_file(path, ", Async!").await.unwrap();

    let content = tokio::fs::read_to_string(path).await.unwrap();
    assert_eq!(content, "Hello, Async!");

    tokio::fs::remove_file(path).await.unwrap();
}

#[tokio::test]
async fn test_write_json() {
    use serde::Serialize;

    #[derive(Serialize)]
    struct Person {
        name: String,
        age: u32,
    }

    let path = "test_write.json";
    let data = Person {
        name: "test".to_string(),
        age: 25,
    };

    write_to_json(path, &data).await.unwrap();

    let content = tokio::fs::read_to_string(path).await.unwrap();
    assert!(content.contains("\"name\""));
    assert!(content.contains("\"test\""));

    tokio::fs::remove_file(path).await.unwrap();
}

#[test]
fn test_create_file_sync() {
    let path = "test_create/nested/file.txt";

    create_file_sync(path).unwrap();
    assert!(std::path::Path::new(path).exists());

    std::fs::remove_dir_all("test_create").unwrap();
}

#[test]
fn test_mkdir_sync() {
    let path = "test_mkdir_sync/nested/dir";

    mkdir_sync(path).unwrap();
    assert!(std::path::Path::new(path).is_dir());

    std::fs::remove_dir_all("test_mkdir_sync").unwrap();
}

#[tokio::test]
async fn test_mkdir() {
    let path = "test_mkdir_async/nested/dir";

    mkdir(path).await.unwrap();
    assert!(std::path::Path::new(path).is_dir());

    tokio::fs::remove_dir_all("test_mkdir_async").await.unwrap();
}

#[tokio::test]
async fn test_create_tempdir() {
    let dir = create_tempdir().await.unwrap();
    assert!(std::path::Path::new(&dir).is_dir());

    std::fs::remove_dir_all(&dir).unwrap();
}

#[tokio::test]
async fn test_create_tempfile() {
    let file = create_tempfile(".txt").await.unwrap();
    assert!(std::path::Path::new(&file).is_file());
    assert!(file.ends_with(".txt"));

    let parent = std::path::Path::new(&file).parent().unwrap();
    std::fs::remove_dir_all(parent).unwrap();
}

#[test]
fn test_soft_link() {
    #[cfg(unix)]
    {
        let target = "test_link_target.txt";
        let link = "test_link.txt";
        std::fs::write(target, "content").unwrap();

        soft_link(target, link).unwrap();
        assert!(std::path::Path::new(link).is_symlink());

        std::fs::remove_file(link).unwrap();
        std::fs::remove_file(target).unwrap();
    }
}

