use anyhow::Result;

pub trait Load {
    fn load(content: std::borrow::Cow<[u8]>) -> Result<Self>
    where
        Self: std::marker::Sized;
}
