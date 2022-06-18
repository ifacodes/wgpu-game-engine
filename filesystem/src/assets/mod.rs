mod handle;
pub mod loader;
use anyhow::Result;
use loader::Loader;
pub trait Asset: Sized {
    type Loader: Loader<Self>;

    const EXTENSION: &'static str = "";

    fn default_value(id: &str) -> Result<Self> {
        todo!()
    }
}
