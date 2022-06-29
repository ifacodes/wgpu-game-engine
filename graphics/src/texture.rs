use std::ops::Deref;

use anyhow::{Context, Result};
use cgmath::Vector2;
use filesystem::Load;
use image::RgbaImage;

/// Texture Definitions
#[allow(dead_code)]
pub struct TextureMut<'a> {
    pub texture: &'a mut Texture,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

#[allow(dead_code)]
impl<'a> TextureMut<'a> {
    pub fn write_texture(&mut self) {}
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub format: wgpu::TextureFormat,
    pub size: Vector2<usize>,
    pub render_target: bool,
}

struct RawTexture(RgbaImage);

impl Deref for RawTexture {
    type Target = RgbaImage;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Load for RawTexture {
    fn load(content: std::borrow::Cow<[u8]>) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(RawTexture(
            image::load_from_memory(&content)
                .context("image::load_from_memory failed to load DynamicImage")?
                .to_rgba8(),
        ))
    }
}

#[allow(dead_code)]
fn new_texture(
    device: &wgpu::Device,
    size: Vector2<usize>,
    format: wgpu::TextureFormat,
) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        size: wgpu::Extent3d {
            width: size.x as u32,
            height: size.y as u32,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
        label: None,
    })
}

#[allow(dead_code)]
fn write_texture(_queue: &wgpu::Queue, _bytes: RawTexture, _size: Vector2<f32>) {}

#[cfg(test)]
mod test {
    use super::RawTexture;

    #[test]
    fn image_to_bytes() {
        let image = include_bytes!("../../resources/images/ifa_pic.jpeg");
        let diffuse = image::load_from_memory(image).unwrap();
        let rgba = diffuse.to_rgba8();
        let raw_texture = RawTexture(rgba);
        let _bytes: &[u8] = &raw_texture;
    }
}
