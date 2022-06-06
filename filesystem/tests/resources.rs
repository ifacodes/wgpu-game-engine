extern crate filesystem;

use filesystem::*;
use notify::*;

#[test]
fn new_watcher() {
    use std::sync::mpsc;

    use notify::RecommendedWatcher;

    let (tx, _) = mpsc::channel();
    let w: Result<RecommendedWatcher> = Watcher::new_raw(tx);
    assert!(w.is_ok());
}

#[test]
fn new_filesystem() {
    let filesystem = FileSystem::new();
    assert!(filesystem.is_ok());
}

#[test]
fn filesystem_watch() {
    let mut filesystem = FileSystem::new().unwrap();
    assert!(filesystem.watch("../resources").is_ok());
}
