use cgmath::Vector2;

/// Texture Definitions
pub struct TextureMut<'a> {
    pub texture: &'a mut Texture,
    pub device: &'a wgpu::Device,
    pub queue: &'a wgpu::Queue,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub format: wgpu::TextureFormat,
    pub size: Vector2<usize>,
    pub render_target: bool,
}
