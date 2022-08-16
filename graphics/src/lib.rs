pub mod system;

/*
The graphics system needs to simply:
- load the wgpu stuff
- take my texture types.
- draw them to the screen in a way I want.

The best way to do this might actually be to Have the graphics as it's own system inside a graphics section
the section contains all the texture info, and the system simply handles the rendering and organising of it.
i.e. the section just says "draw the items that rendering to each buffer.
and the system draws the necessary textures to each buffer, and then each buffer to the screen.


*/

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
