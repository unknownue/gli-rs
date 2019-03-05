
pub type Extent1d = u32;
pub type Extent2d = [u32; 2];
pub type Extent3d = [u32; 3];

pub use self::image::GliImage;

pub use self::target::Target;
pub use self::format::{Format, Swizzle, Swizzles};

pub use self::load::*;
pub use self::save::*;

pub mod texture;

mod target;
mod load;
mod save;
mod image;
mod format;

mod ffi;
mod error;
