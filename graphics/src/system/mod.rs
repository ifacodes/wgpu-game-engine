use cgmath::{vec2, vec3};
use wgpu::{util::DeviceExt, Backends};
use winit::{dpi::PhysicalSize, window::Window};

use self::{shader::Shader, vertex::Vertex};
mod pipeline;
mod shader;
mod vertex;

pub struct GraphicSystem {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl GraphicSystem {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("WGPU Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &surface_config);

        Self {
            device,
            queue,
            surface,
            surface_config,
            size,
        }
    }

    // #[allow(dead_code)]
    // pub fn resize_surface(&mut self) {
    //     todo!()
    // }

    // pub fn draw_to_view(
    //     view: &wgpu::TextureView,
    //     encoder: &mut wgpu::CommandEncoder,
    //     pipeline: &wgpu::RenderPipeline,
    // ) {
    //     let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
    //         label: Some("Render Pass"),
    //         color_attachments: &[wgpu::RenderPassColorAttachment {
    //             view,
    //             resolve_target: None,
    //             ops: wgpu::Operations {
    //                 load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
    //                 store: true,
    //             },
    //         }],
    //         depth_stencil_attachment: None,
    //     });

    //     render_pass.set_pipeline(pipeline);
    // }

    pub fn create_render_pass(&mut self, view: &wgpu::TextureView, shader: &Shader) {
        let vb: [Vertex; 3] = [
            Vertex {
                pos: vec3(0.0, 0.5, 0.0),
                frame: vec2(0.0, 0.0),
            },
            Vertex {
                pos: vec3(-0.5, -0.5, 0.0),
                frame: vec2(0.0, 0.0),
            },
            Vertex {
                pos: vec3(0.5, -0.5, 0.0),
                frame: vec2(0.0, 0.0),
            },
        ];

        let vertex_buffer = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("VBO"),
                contents: bytemuck::cast_slice(&vb),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        // get render pipeline based on parameters input into render pass
        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&shader.pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader.shader,
                    entry_point: "vs_main",
                    buffers: &[Vertex::desc()],
                },
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: Some(wgpu::IndexFormat::Uint16),
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader.shader,
                    entry_point: "fs_main",
                    targets: &[wgpu::ColorTargetState {
                        format: self.surface_config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                multiview: None,
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&render_pipeline);
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.draw(0..vb.len() as u32, 0..1);
        }
    }
}

struct RenderPass<'a> {
    view: &'a wgpu::TextureView,
    pipeline: &'a wgpu::RenderPipeline,
}

impl<'a> RenderPass<'a> {}
