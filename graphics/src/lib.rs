mod assets;
mod pipeline;
mod sampler;
mod shader;
mod texture;
use assets::shader::Shader;
pub use assets::GraphicAssets;
use util::Lookup;
use wgpu::Backends;
use winit::{dpi::PhysicalSize, window::Window};

/// The graphics system needs to simply:
/// - load the wgpu stuff
/// - take my texture types.
/// - draw them to the screen in a way I want.
///
/// The best way to do this might actually be to Have the graphics as it's own system inside a graphics section
/// the section contains all the texture info, and the system simply handles the rendering and organising of it.
/// i.e. the section just says "draw the items that rendering to each buffer.
/// and the system draws the necessary textures to each buffer, and then each buffer to the screen.
///

// fn load_texture(&mut self, key: Key, bytes: &[u8], size: Vector2<usize>) {
//     let texture = new_texture(
//         &self.system.device,
//         size,
//         wgpu::TextureFormat::Rgba8UnormSrgb,
//     );
//     self.system.queue.write_texture(
//         texture.as_image_copy(),
//         bytes,
//         wgpu::ImageDataLayout {
//             offset: 0,
//             bytes_per_row: std::num::NonZeroU32::new(4 * size.x as u32),
//             rows_per_image: std::num::NonZeroU32::new(size.y as u32),
//         },
//         wgpu::Extent3d {
//             width: size.x as u32,
//             height: size.y as u32,
//             depth_or_array_layers: 1,
//         },
//     );
//     self.textures.insert(
//         key,
//         Texture {
//             texture,
//             size,
//             format: wgpu::TextureFormat::Rgba8UnormSrgb,
//             render_target: false,
//         },
//     );
// }

// pub fn get_texture(&self, key: &Key) -> Option<&Texture> {
//     self.textures.get(key)
// }

// pub fn get_texture_mut(&mut self, key: &Key) -> Option<TextureMut> {
//     let texture = self.textures.get_mut(key)?;
//     Some(TextureMut {
//         texture,
//         device: &self.system.device,
//         queue: &self.system.queue,
//     })
// }

#[allow(dead_code)]
pub struct GraphicSystem {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
    shaders: Lookup<Shader>,
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
            shaders: Lookup::new(),
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
}
