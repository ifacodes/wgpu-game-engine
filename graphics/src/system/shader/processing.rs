use std::collections::HashMap;

use anyhow::Result;
use naga::{ImageClass, Module, ScalarKind};
use thiserror::Error;
use wgpu::{SamplerBindingType, TextureSampleType};

#[derive(Debug, Error)]
enum Error {
    // #[error("Unknown type translation error")]
    // Unknown,
    #[error("The translation for this type hasn't been implemented yet")]
    TypeNotImplemented,
    #[error("Not a valid type for a binding resource.")]
    NotBindingResource,
    #[error("Not a texture array type supported by wgpu.")]
    InvalidTextureArray,
    #[error("None float scalarkind is not supported.")]
    NotFloat,
}

pub fn generate_bind_group_layouts(
    device: &wgpu::Device,
    entries: HashMap<u32, Vec<wgpu::BindGroupLayoutEntry>>,
) -> Vec<wgpu::BindGroupLayout> {
    let bind_group_layouts: Vec<wgpu::BindGroupLayout> =
        entries
            .iter()
            .fold(Vec::new(), |mut acc, (group, entries)| {
                acc.push(
                    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        label: Some(format!("Group: {}", group).as_str()),
                        entries,
                    }),
                );
                acc
            });
    bind_group_layouts
}

pub fn generate_pipeline_layout(
    shader_name: &str,
    device: &wgpu::Device,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(shader_name),
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}

pub fn generate_layout_entries(
    module: &Module,
) -> Result<HashMap<u32, Vec<wgpu::BindGroupLayoutEntry>>> {
    let mut map = HashMap::new();
    for (gh, gv) in module
        .global_variables
        .iter()
        .filter(|(_, b)| b.binding.is_some())
    {
        let stages =
            module
                .entry_points
                .iter()
                .fold(
                    wgpu::ShaderStages::NONE,
                    |acc, entry_point| match entry_point.function.expressions.iter().find_map(
                        |(_, x)| match x {
                            naga::Expression::GlobalVariable(handle) => {
                                if *handle == gh {
                                    Some(entry_point.stage)
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        },
                    ) {
                        Some(s) => match s {
                            naga::ShaderStage::Vertex => acc | wgpu::ShaderStages::VERTEX,
                            naga::ShaderStage::Fragment => acc | wgpu::ShaderStages::FRAGMENT,
                            naga::ShaderStage::Compute => acc | wgpu::ShaderStages::COMPUTE,
                        },
                        None => acc,
                    },
                );
        let binding = gv.binding.as_ref().unwrap();
        let ty = map_naga_inner_type_to_wgpu_binding_type(module.types.get_handle(gv.ty)?)?;
        map.entry(binding.group)
            .or_insert(Vec::new())
            .push(wgpu::BindGroupLayoutEntry {
                binding: binding.binding,
                visibility: stages,
                ty,
                count: None,
            });
    }
    Ok(map)
}

fn map_naga_inner_type_to_wgpu_binding_type(ty: &naga::Type) -> Result<wgpu::BindingType, Error> {
    match ty.inner {
        naga::TypeInner::Image {
            dim,
            arrayed,
            class,
        } => match class {
            naga::ImageClass::Sampled {
                kind: ScalarKind::Float,
                multi,
            } => match dim {
                naga::ImageDimension::D1 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D1,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: multi,
                }),
                naga::ImageDimension::D3 if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D3,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if !arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::Cube,
                    multisampled: multi,
                }),
                naga::ImageDimension::D2 if arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2Array,
                    multisampled: multi,
                }),
                naga::ImageDimension::Cube if arrayed => Ok(wgpu::BindingType::Texture {
                    sample_type: TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::CubeArray,
                    multisampled: multi,
                }),
                _ => Err(Error::InvalidTextureArray),
            },
            ImageClass::Depth { .. } => Err(Error::TypeNotImplemented),
            ImageClass::Storage { .. } => Err(Error::TypeNotImplemented),
            _ => Err(Error::NotFloat),
        },
        naga::TypeInner::Sampler { comparison } if comparison => {
            Ok(wgpu::BindingType::Sampler(SamplerBindingType::Comparison))
        }
        naga::TypeInner::Sampler { comparison } if !comparison => {
            Ok(wgpu::BindingType::Sampler(SamplerBindingType::Filtering))
        }
        _ => Err(Error::NotBindingResource),
    }
}
