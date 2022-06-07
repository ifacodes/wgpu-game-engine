mod filecache;

use anyhow::Result;
//use filecache::FileCache;
use mini_fs::{LocalFs, MiniFs};
use notify::{DebouncedEvent, RecommendedWatcher, Watcher};
use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

#[allow(dead_code)]
pub struct FileSystem {
    watcher: RecommendedWatcher,
    event_queue: Arc<Mutex<VecDeque<DebouncedEvent>>>,
    internal: MiniFs,
}

impl FileSystem {
    pub fn new<P>(mount_point: P, path: P) -> Result<Self>
    where
        P: Into<PathBuf> + std::convert::AsRef<std::path::Path>,
    {
        let (tx, rx) = mpsc::channel();
        let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

        let event_queue = Arc::new(Mutex::new(VecDeque::new()));
        let queue = event_queue.clone();
        watcher.watch(path.as_ref(), notify::RecursiveMode::Recursive)?;
        thread::spawn(move || loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::NoticeRemove(_) => {}
                    DebouncedEvent::NoticeWrite(_) => {}
                    event => queue.lock().unwrap().push_back(event),
                },
                Err(_) => {}
            }
        });
        let internal = MiniFs::new().mount(mount_point, LocalFs::new(path));
        Ok(Self {
            watcher,
            event_queue,
            internal,
        })
    }

    pub fn handle_events(&self) {
        for event in self
            .event_queue
            .clone()
            .lock()
            .unwrap()
            .drain(..)
            .into_iter()
        {
            println!("event\n{:#?}\n was processed.", event);
        }
    }
}
