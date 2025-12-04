use afs::*;

#[tokio::test]
async fn test_read_file() {
    let path = "test_read.txt";
    std::fs::write(path, "Hello, world!").unwrap();

    let result = read_file(path).await.unwrap();
    assert_eq!(result, "Hello, world!");

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_read_file_sync() {
    let path = "test_read_sync.txt";
    std::fs::write(path, "Hello, sync!").unwrap();

    let result = read_file_sync(path).unwrap();
    assert_eq!(result, "Hello, sync!");

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_read_json() {
    let file_path = "test_read.json";
    let content = r#"{"name": "test", "age": 30}"#;
    tokio::fs::write(file_path, content).await.unwrap();

    let result: serde_json::Value = read_from_json(file_path).await.unwrap();
    assert_eq!(result["name"], "test");
    assert_eq!(result["age"], 30);

    tokio::fs::remove_file(file_path).await.unwrap();
}

#[tokio::test]
async fn test_read_json_value() {
    let file_path = "test_read_value.json";
    let content = r#"{"items": [1, 2, 3]}"#;
    tokio::fs::write(file_path, content).await.unwrap();

    let result = read_json(file_path).await.unwrap();
    assert_eq!(result["items"][0], 1);

    tokio::fs::remove_file(file_path).await.unwrap();
}

#[tokio::test]
async fn test_get_file_size() {
    let path = "test_size.txt";
    std::fs::write(path, "12345").unwrap();

    let size = get_file_size(path).await.unwrap();
    assert_eq!(size, 5);

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_get_dir_size() {
    let dir = "test_dir_size";
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(format!("{}/a.txt", dir), "123").unwrap();
    std::fs::write(format!("{}/b.txt", dir), "4567").unwrap();

    let size = get_dir_size(dir).await.unwrap();
    assert_eq!(size, 7);

    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn test_stat_sync() {
    let path = "test_stat.txt";
    std::fs::write(path, "test").unwrap();

    let metadata = stat_sync(path).unwrap();
    assert!(metadata.is_file());

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_stat() {
    let path = "test_stat_async.txt";
    std::fs::write(path, "test").unwrap();

    let metadata = stat(path).await.unwrap();
    assert!(metadata.is_file());

    std::fs::remove_file(path).unwrap();
}

#[test]
fn test_hash_sync() {
    let path = "test_hash.txt";
    std::fs::write(path, "hello").unwrap();

    let hash_val = hash_sync(path).unwrap();
    assert!(!hash_val.is_empty());
    assert_eq!(hash_val.len(), 64);

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_hash() {
    let path = "test_hash_async.txt";
    std::fs::write(path, "hello").unwrap();

    let hash_result = hash(path).await.unwrap();
    assert!(!hash_result.is_empty());
    assert_eq!(hash_result.len(), 64);

    std::fs::remove_file(path).unwrap();
}

#[tokio::test]
async fn test_diskusage() {
    let usage = diskusage().await;
    assert!(usage.is_ok());
    let used_space = usage.unwrap();
    assert!(used_space > 0.0);
}

