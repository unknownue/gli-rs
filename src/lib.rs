
pub type Extent1d = u32;
pub type Extent2d = [u32; 2];
pub type Extent3d = [u32; 3];

pub use self::image::GliImage;
pub use self::texture::GliTexture;
pub use self::texture::Texture1D;

pub use self::target::Target;
pub use self::format::{Format, Swizzle, Swizzles};

pub mod ffi;
pub mod texture;

mod target;
mod image;
mod format;
