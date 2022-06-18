use anyhow::Result;

use notify::RecommendedWatcher;
use std::collections::HashSet;
use std::fmt::Formatter;

use std::path::{Path, PathBuf};

use crate::assets::Load;

/// Filesystem

#[derive(Default, Debug, PartialEq)]
pub struct FileSystem {
    watcher: Watcher,
    path: PathBuf,
    loaded: HashSet<PathBuf>,
}

impl FileSystem {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }
    pub fn new_with_watcher<P>(_path: P) -> Self
    where
        P: Into<PathBuf> + AsRef<Path>,
    {
        todo!()
    }

    pub fn load<A>(&mut self, path: &str) -> Result<A>
    where
        A: Load,
    {
        if let Ok(asset) = A::load(std::fs::read(path)?.into()) {
            self.loaded.insert(path.into());
            Ok(asset)
        } else {
            Err(anyhow::anyhow!("TODO: Write error message lmao!"))
        }
    }
}

#[derive(Default)]
struct Watcher(Option<RecommendedWatcher>);

impl std::fmt::Debug for Watcher {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0.is_some()))
    }
}

impl PartialEq for Watcher {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_some() == other.0.is_some()
    }
}
