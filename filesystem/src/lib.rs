//! File loading and watching library.
//!
//! Provides an abstraction over the filesystem. When a loader is assigned
//! to an extension or a directory it allows easy loading of the type.
//!
//!

mod assets;
mod system;

pub use assets::Load;
pub use system::FileSystem;
