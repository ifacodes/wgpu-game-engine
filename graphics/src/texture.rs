/// TODO!

#[allow(dead_code)]
pub struct TextureMut<'a> {
    texture: &'a mut Texture,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
