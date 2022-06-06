use std::collections::VecDeque;
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{error, fmt, thread};

use notify::{DebouncedEvent, RecommendedWatcher, Watcher};

#[derive(Debug)]
pub enum Error {
    FileSystem,
    /// Error from notify crate.
    Notify(notify::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error = match *self {
            Error::FileSystem => String::from("FileSystem Error"),
            Error::Notify(ref err) => err.to_string(),
        };
        write!(f, "{}", error)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::FileSystem => None,
            Error::Notify(ref e) => Some(e),
        }
    }
}

impl From<notify::Error> for Error {
    fn from(err: notify::Error) -> Error {
        Error::Notify(err)
    }
}

pub struct FileSystem {
    rx: Option<mpsc::Receiver<DebouncedEvent>>,
    watcher: RecommendedWatcher,
    event_queue: Arc<Mutex<VecDeque<DebouncedEvent>>>,
}

impl FileSystem {
    pub fn new() -> Result<Self, Error> {
        let (tx, rx) = mpsc::channel();
        let watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;
        let event_queue = Arc::new(Mutex::new(VecDeque::new()));
        Ok(Self {
            rx: Some(rx),
            watcher,
            event_queue,
        })
    }

    pub fn watch<P>(&mut self, path: P) -> Result<(), Error>
    where
        P: AsRef<Path>,
    {
        let rx = self.rx.take().unwrap();
        self.watcher.watch(path, notify::RecursiveMode::Recursive)?;
        let queue = self.event_queue.clone();
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
        Ok(())
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
