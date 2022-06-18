extern crate filesystem;

use anyhow::Result;
use std::borrow::Cow;
use std::collections::HashMap;
use std::path::Path;

use filesystem::assets::loader::Loader;
use filesystem::assets::Asset;
use filesystem::builder::FileSystemBuilder;
use filesystem::system::FileSystem;

struct RawTextureLoader {}
#[derive(Debug)]
struct RawTexture {
    bytes: Vec<u8>,
}

impl Asset for RawTexture {
    const EXTENSION: &'static str = "art";

    type Loader = RawTextureLoader;
}

impl Loader<RawTexture> for RawTextureLoader {
    fn load(content: Cow<'_, [u8]>) -> Result<RawTexture> {
        let bytes: Vec<u8> = content.into();
        Ok(RawTexture { bytes })
    }
}

#[test]
fn builder_test() {
    let com = FileSystem {
        watcher: None,
        path: ".".into(),
        cache: HashMap::new(),
    };
    let filesystem = FileSystemBuilder::new(".").build();
    assert_eq!(com, filesystem);
}

#[test]
fn load_file() {
    let filesystem = FileSystemBuilder::new("./resources").build();
    assert!(filesystem
        .load::<RawTexture, _>("../resources/images/ifa_pic.jpeg")
        .is_ok());
}
