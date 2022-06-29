use anyhow::{Context, Result};
use filesystem::Load;
use std::borrow::Cow;
use wgpu::ShaderSource;

pub(crate) struct Shader {
    module: wgpu::ShaderModule,
}

impl Load for Shader {
    fn load(content: std::borrow::Cow<[u8]>) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        todo!()
    }
}

#[allow(dead_code)]
pub(crate) struct RawShader<'a>(ShaderSource<'a>);

impl<'a> Load for RawShader<'a> {
    fn load(content: std::borrow::Cow<[u8]>) -> Result<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(ShaderSource::Wgsl(Cow::from(
            String::from_utf8(content.into()).context("Unable to load shader as Wgsl")?,
        ))
        .into())
    }
}

impl<'a> From<ShaderSource<'a>> for RawShader<'a> {
    fn from(source: ShaderSource<'a>) -> Self {
        RawShader(source)
    }
}

impl<'a> RawShader<'a> {
    pub fn create_shader_module(self, device: &wgpu::Device) -> wgpu::ShaderModule {
        device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: self.0,
        })
    }
}

#[cfg(test)]
mod test {
    use filesystem::FileSystem;

    use super::RawShader;

    #[test]
    fn load_shader_from_source() {
        let mut filesystem = FileSystem::new("../../");
        let _ = filesystem.load::<RawShader>("resources/shaders/shader.wgsl");
    }
}
