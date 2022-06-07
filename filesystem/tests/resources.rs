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
    let filesystem = FileSystem::new("/resources", "../resources");
    assert!(filesystem.is_ok());
}

// #[test]
// fn filesystem_watch() {
//     let filesystem = FileSystem::new("/resources", "../resources").unwrap();
// }
