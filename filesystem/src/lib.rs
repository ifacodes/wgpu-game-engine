pub mod assets;
pub mod builder;
pub mod subscribe;
pub mod system;

// trait FromArchive {
//     fn new_from_archive<P>(mount_point: P, path: P) -> Result<Self>
//     where
//         P: Into<PathBuf> + std::convert::AsRef<Path>,
//         Self: Sized;
// }

// pub struct FileSystem {
//     watcher: RecommendedWatcher,
//     event_queue: Arc<Mutex<VecDeque<DebouncedEvent>>>,
//     internal: MiniFs,
// }

// impl FileSystem {
//     pub fn new<P>(mount_point: P, path: P) -> Result<Self>
//     where
//         P: Into<PathBuf> + std::convert::AsRef<Path>,
//     {
//         let (tx, rx) = mpsc::channel();
//         let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

//         let event_queue = Arc::new(Mutex::new(VecDeque::new()));
//         let queue = event_queue.clone();
//         watcher.watch(path.as_ref(), notify::RecursiveMode::Recursive)?;
//         thread::spawn(move || loop {
//             match rx.recv() {
//                 Ok(event) => match event {
//                     DebouncedEvent::NoticeRemove(_) => {}
//                     DebouncedEvent::NoticeWrite(_) => {}
//                     event => queue.lock().unwrap().push_back(event),
//                 },
//                 Err(_) => {}
//             }
//         });
//         let internal = MiniFs::new().mount(mount_point, LocalFs::new(path));
//         Ok(Self {
//             watcher,
//             event_queue,
//             internal,
//         })
//     }

//     pub fn handle_events(&self) {
//         for event in self
//             .event_queue
//             .clone()
//             .lock()
//             .unwrap()
//             .drain(..)
//             .into_iter()
//         {
//             println!("event\n{:#?}\n was processed.", event);
//         }
//     }

//     pub fn open<P>(&self, path: P) -> io::Result<mini_fs::File>
//     where
//         P: AsRef<Path>,
//     {
//         self.internal.open(path)
//     }

//     pub fn unmount<P>(&mut self, path: P) -> Result<()>
//     where
//         P: AsRef<Path>,
//     {
//         self.watcher.unwatch(path.as_ref())?;
//         self.internal.umount(path);
//         Ok(())
//     }
// }

// impl FromArchive for FileSystem {
//     fn new_from_archive<P>(mount_point: P, path: P) -> Result<Self>
//     where
//         P: Into<PathBuf> + std::convert::AsRef<Path>,
//         Self: Sized,
//     {
//         let (tx, rx) = mpsc::channel();
//         let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2))?;

//         let event_queue = Arc::new(Mutex::new(VecDeque::new()));
//         let queue = event_queue.clone();
//         watcher.watch(path.as_ref(), notify::RecursiveMode::Recursive)?;
//         thread::spawn(move || loop {
//             match rx.recv() {
//                 Ok(event) => match event {
//                     DebouncedEvent::NoticeRemove(_) => {}
//                     DebouncedEvent::NoticeWrite(_) => {}
//                     event => queue.lock().unwrap().push_back(event),
//                 },
//                 Err(_) => {}
//             }
//         });
//         let archive = TarFs::open(path)?;
//         let internal = MiniFs::new().mount(mount_point, archive);
//         Ok(Self {
//             watcher,
//             event_queue,
//             internal,
//         })
//     }
// }
