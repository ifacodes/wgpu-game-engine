mod processing;
use anyhow::Result;
use filesystem::Load;
use naga::{
    back::wgsl::{write_string, WriterFlags},
    front::wgsl::parse_str,
    valid::{Capabilities, ValidationFlags, Validator},
    Module,
};
use wgpu::ShaderModule;

use self::processing::{
    generate_bind_group_layouts, generate_layout_entries, generate_pipeline_layout,
};

struct UnprocessedShader(Module);

pub struct Shader {
    pub shader: ShaderModule,
    pub bind_group_layouts: Vec<wgpu::BindGroupLayout>,
    pub pipeline_layout: wgpu::PipelineLayout,
}

// TODO: Throw error if entry point names do not match required.
// TODO: Figure out a better way to create a debug label for the shader.

impl Shader {
    fn new(u: UnprocessedShader, device: &wgpu::Device) -> Result<Self> {
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::all());
        let info = validator.validate(&u.0)?;
        let entries = generate_layout_entries(&u.0).unwrap();
        let bind_group_layouts = generate_bind_group_layouts(device, entries);
        let pipeline_layout =
            generate_pipeline_layout("", device, &bind_group_layouts.iter().collect::<Vec<_>>());
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(write_string(&u.0, &info, WriterFlags::all())?.into()),
        });
        Ok(Self {
            shader,
            bind_group_layouts,
            pipeline_layout,
        })
    }
}

impl Load for UnprocessedShader {
    fn load(content: std::borrow::Cow<[u8]>) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(Self(parse_str(std::str::from_utf8(&content)?)?))
    }
}
