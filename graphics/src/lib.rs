mod texture;
use std::collections::HashMap;
use wgpu::Backends;
use winit::window::Window;

use self::texture::Texture;

/// The graphics system needs to simply:
/// - load the wgpu stuff
/// - take my texture types.
/// - draw them to the screen in a way I want.
///
/// The best way to do this might actually be to Have the graphics as it's own system inside a graphics section
/// the section contains all the texture info, and the system simply handles the rendering and organising of it.
/// i.e. the section just says "draw the items that rendering to each buffer.
/// and the system draws the necessary textures to each buffer, and then each buffer to the screen.

#[allow(dead_code)]
pub struct GraphicSystem {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    textures: HashMap<String, Texture>,
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
            textures: HashMap::new(),
        }
    }

    #[allow(dead_code)]
    pub fn resize_surface(&mut self) {
        todo!()
    }

    pub fn draw_to_view(
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        pipeline: &wgpu::RenderPipeline,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
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

        render_pass.set_pipeline(pipeline);
    }
}
