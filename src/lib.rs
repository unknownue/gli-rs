
pub type Extent3d = [u32; 3];

pub use self::target::Target;

pub mod ffi;
pub mod format;
pub mod texture;

mod target;
mod image;
