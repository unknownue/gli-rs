
pub use self::format::{Format, Swizzle, Swizzles};
pub use self::extent::{Extent1d, Extent2d, Extent3d};

pub use self::image::GliImage;
pub use self::texture::*;

pub use self::load::*;
pub use self::save::*;

pub use self::error::{Result, Error};

pub mod target;
pub mod dx;
pub mod gl;
pub mod sampler;
pub mod extent;

mod load;
mod save;
mod image;
mod format;
mod error;

#[cfg(feature = "rc_debug")]
pub mod ffi;
#[cfg(feature = "rc_debug")]
pub mod texture;

#[cfg(not(feature = "rc_debug"))]
mod ffi;
#[cfg(not(feature = "rc_debug"))]
mod texture;
