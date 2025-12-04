use afs::*;

#[test]
fn test_chmod_sync() {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = "test_chmod.txt";
        write_file_sync(path, "test").unwrap();

        chmod_sync("755", path).unwrap();

        let metadata = std::fs::metadata(path).unwrap();
        let mode = metadata.permissions().mode();
        assert_eq!(mode & 0o777, 0o755);

        std::fs::remove_file(path).unwrap();
    }

    #[cfg(windows)]
    {
        let path = "test_chmod.txt";
        write_file_sync(path, "test").unwrap();

        chmod_sync("777", path).unwrap();

        let metadata = std::fs::metadata(path).unwrap();
        assert!(!metadata.permissions().readonly());

        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn test_chmod_readonly() {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let path = "test_chmod_readonly.txt";
        write_file_sync(path, "test").unwrap();

        chmod_sync("444", path).unwrap();

        let metadata = std::fs::metadata(path).unwrap();
        let mode = metadata.permissions().mode();
        assert_eq!(mode & 0o777, 0o444);

        chmod_sync("644", path).unwrap();
        std::fs::remove_file(path).unwrap();
    }
}

#[test]
fn test_resolve() {
    #[cfg(unix)]
    {
        let base = "/home/user";

        let result = resolve(base, "../test.txt").unwrap();
        assert_eq!(result, "/home/test.txt");

        let result = resolve(base, "./test.txt").unwrap();
        assert_eq!(result, "/home/user/test.txt");

        let result = resolve(base, "test.txt").unwrap();
        assert_eq!(result, "/home/user/test.txt");

        let result = resolve(base, "./../test.txt").unwrap();
        assert_eq!(result, "/home/test.txt");

        let result = resolve(base, "/absolute/path.txt").unwrap();
        assert_eq!(result, "/absolute/path.txt");
    }

    #[cfg(windows)]
    {
        let base = r"C:\Users\test";

        let result = resolve(base, "file.txt").unwrap();
        assert!(result.contains("file.txt"));
    }
}

#[test]
fn test_normalize_path() {
    let path = r"C:\Users\test\file.txt";
    let result = normalize_path(path);
    assert_eq!(result, "C:/Users/test/file.txt");

    let path = "already/normal/path";
    let result = normalize_path(path);
    assert_eq!(result, "already/normal/path");
}

#[test]
fn test_get_filepath() {
    let path = ".";
    let result = get_filepath(path);
    assert!(result.is_ok());

    let result = get_filepath("nonexistent_file_12345.txt");
    assert!(result.is_err());
}

#[test]
fn test_basename() {
    let result = basename("/home/user/test.txt").unwrap();
    assert_eq!(result, "test.txt");

    let result = basename("test.txt").unwrap();
    assert_eq!(result, "test.txt");

    let result = basename("/path/to/dir/").unwrap();
    assert_eq!(result, "dir");
}

#[test]
fn test_filename() {
    let result = filename("/home/user/test.txt").unwrap();
    assert_eq!(result, "test.txt");

    let result = filename("file.rs").unwrap();
    assert_eq!(result, "file.rs");
}

#[test]
fn test_dirname() {
    let result = dirname("/home/user/test.txt").unwrap();
    assert_eq!(result, "/home/user");

    let result = dirname("file.txt").unwrap();
    assert_eq!(result, ".");

    let result = dirname("/root").unwrap();
    assert_eq!(result, "/");
}

