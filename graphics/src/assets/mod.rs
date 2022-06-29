use util::Lookup;

use crate::texture::Texture;

use self::shader::Shader;

pub mod shader;

pub struct GraphicAssets {
    shaders: Lookup<Shader>,
    textures: Lookup<Texture>,
}

impl Default for GraphicAssets {
    fn default() -> Self {
        Self::new()
    }
}

impl GraphicAssets {
    pub fn new() -> Self {
        Self {
            shaders: Lookup::new(),
            textures: Lookup::new(),
        }
    }
}
