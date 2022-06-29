use anyhow::Result;
use naga::{ImageClass, Module, ScalarKind};
use thiserror::Error;
use wgpu::{SamplerBindingType, TextureSampleType};

/// Needed to create BindGroupLayoutDescriptor
/// - BindGroupLayoutEntry
/// - Label
///
/// Needed to create BindGroupLayoutEntry
/// - binding index
/// - shader stage visibility
/// - type
/// - count (whether or not it is an array)
///
/// Needed to create RenderPipelineDescriptor
/// - vertex
///   - reference to shader
///   - entry point name as string
///   - buffers
/// - fragment
///   - reference to shader
///   - entry point name as string
///   - targets / ColorTargetState
///
/// for every global variable that is a binding
/// check each entry_point function for a global variable expression with a matching handle.
///
/// I think I will enforce that textures and samplers are filtering, at least for now
///

#[derive(Debug, Error)]
enum Error {
    #[error("Unknown type translation error")]
    Unknown,
    #[error("The translation for this type hasn't been implemented yet")]
    NotImplemented,
    #[error("Not a valid type for a binding resource.")]
    NotBinding,
    #[error("Not a texture array type supported by wgpu.")]
    NotValidTextureArray,
    #[error("None float scalarkind is not supported.")]
    NotFloat,
}

pub fn generate_bind_group_layout() -> wgpu::BindGroupLayout {
    todo!()
}
pub fn generate_pipeline_layout() -> wgpu::PipelineLayout {
    todo!()
}

pub fn generate_layout_entries(module: &Module) -> Result<Vec<wgpu::BindGroupLayoutEntry>> {
    Ok(module
        .global_variables
        .iter()
        .filter(|(_, b)| b.binding.is_some())
        .map(|(gh, gv)| {
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
            let ty =
                _map_naga_inner_type_to_wgpu_binding_type(module.types.get_handle(gv.ty).unwrap())
                    .unwrap();
            wgpu::BindGroupLayoutEntry {
                binding: gv.binding.as_ref().unwrap().binding,
                visibility: stages,
                ty,
                count: None,
            }
        })
        .collect())
}

fn _map_naga_inner_type_to_wgpu_binding_type(ty: &naga::Type) -> Result<wgpu::BindingType, Error> {
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
                _ => Err(Error::NotValidTextureArray),
            },
            ImageClass::Depth { .. } => Err(Error::NotImplemented),
            ImageClass::Storage { .. } => Err(Error::NotImplemented),
            _ => Err(Error::NotFloat),
        },
        naga::TypeInner::Sampler { comparison } if comparison => {
            Ok(wgpu::BindingType::Sampler(SamplerBindingType::Comparison))
        }
        naga::TypeInner::Sampler { comparison } if !comparison => {
            Ok(wgpu::BindingType::Sampler(SamplerBindingType::Filtering))
        }
        _ => Err(Error::NotBinding),
    }
}

fn _map_texture_dimension(d: &naga::ImageDimension) -> wgpu::TextureViewDimension {
    match d {
        naga::ImageDimension::D1 => wgpu::TextureViewDimension::D1,
        naga::ImageDimension::D2 => wgpu::TextureViewDimension::D2,
        naga::ImageDimension::D3 => wgpu::TextureViewDimension::D3,
        naga::ImageDimension::Cube => wgpu::TextureViewDimension::Cube,
    }
}
