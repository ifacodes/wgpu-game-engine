use anyhow::Result;
use std::borrow::Cow;

pub trait Loader<T> {
    fn load(content: Cow<'_, [u8]>) -> Result<T>;
}
