mod processing;
use filesystem::Load;
use naga::{front::wgsl::parse_str, Module};
use wgpu::ShaderModule;

use self::processing::generate_layout_entries;

struct Shader<S> {
    state: S,
}

struct UnprocessedModule {
    module: Module,
}

struct ProcessedModule {
    shader: ShaderModule,
}

impl From<Shader<UnprocessedModule>> for Shader<ProcessedModule> {
    fn from(u: Shader<UnprocessedModule>) -> Self {
        let _ = generate_layout_entries(&u.state.module).unwrap();
        todo!()
    }
}

impl Load for Shader<UnprocessedModule> {
    fn load(content: std::borrow::Cow<[u8]>) -> anyhow::Result<Self>
    where
        Self: std::marker::Sized,
    {
        Ok(parse_str(std::str::from_utf8(&content)?)?.into())
    }
}

impl From<Module> for Shader<UnprocessedModule> {
    fn from(module: Module) -> Self {
        Shader {
            state: UnprocessedModule { module },
        }
    }
}

#[cfg(test)]
mod test {
    use filesystem::FileSystem;

    use super::{processing, Shader, UnprocessedModule};

    #[test]
    fn list_function_expressions() {
        let mut fs = FileSystem::new("../");
        let shader = fs
            .load::<Shader<UnprocessedModule>>("resources/shaders/shader.wgsl")
            .unwrap();
        dbg!(processing::generate_layout_entries(&shader.state.module).unwrap());
    }
}
