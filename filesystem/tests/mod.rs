use filesystem::{FileSystem, Load};
use image::DynamicImage;
use std::io::Cursor;
extern crate filesystem;
use anyhow::Result;
use std::string::String;

pub struct RawTexture(DynamicImage);

pub struct RawShader(String);

impl Load for RawTexture {
    fn load(content: std::borrow::Cow<'_, [u8]>) -> Result<RawTexture> {
        let reader = image::io::Reader::new(Cursor::new(content))
            .with_guessed_format()
            .expect("Cursor should never fail");
        Ok(RawTexture(reader.decode()?))
    }
}

impl Load for RawShader {
    fn load(content: std::borrow::Cow<[u8]>) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(RawShader(String::from_utf8(content.into())?))
    }
}

#[test]
fn new_filesystem() {
    let fs = FileSystem::default();
    let fs2 = FileSystem::new(".");
    assert_ne!(fs, fs2);
}

#[test]
fn load_texture() {
    let mut fs = FileSystem::new("../");
    assert!(fs
        .load::<RawTexture>("resources/images/ifa_pic.jpeg")
        .is_ok())
}

#[test]
fn fail_to_load_texture() {
    let mut fs = FileSystem::new("../");
    assert!(fs
        .load::<RawTexture>("resources/images/ifa_pic.png")
        .is_err())
}

#[test]
fn load_shader_into_string() {
    let mut fs = FileSystem::new("../");
    assert!(fs
        .load::<RawShader>("resources/shaders/shader.wgsl")
        .is_ok())
}

#[test]
fn fail_to_load_shader_into_string() {
    let mut fs = FileSystem::new("../");
    assert!(fs
        .load::<RawShader>("resources/shaders/idontexist.wgsl")
        .is_err())
}
