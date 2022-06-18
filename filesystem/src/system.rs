use anyhow::Result;
use notify::RecommendedWatcher;
use std::any::TypeId;
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};

use crate::assets::loader::Loader;
use crate::assets::Asset;

/// Filesystem
pub struct FileSystem {
    pub watcher: Option<RecommendedWatcher>,
    pub path: PathBuf,
    pub cache: HashMap<AssetKey, String>,
}

impl std::fmt::Debug for FileSystem {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.debug_struct("FileSystem")
            .field("watcher", &self.watcher.is_some())
            .field("path", &self.path)
            .finish()
    }
}

impl PartialEq for FileSystem {
    fn eq(&self, other: &Self) -> bool {
        self.watcher.is_some() == other.watcher.is_some() && self.path == other.path
    }
}

impl Eq for FileSystem {}

impl FileSystem {
    pub fn load<A, P>(&self, path: P) -> Result<()>
    where
        A: Asset + std::fmt::Debug,
        P: AsRef<Path>,
    {
        let bytes: Cow<[u8]> = std::fs::read(path).map(Into::into)?;
        let result = A::Loader::load(bytes)?;
        println!("{:?}", result);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AssetKey {
    version: usize,
    id: TypeId,
}
