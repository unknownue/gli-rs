
pub use self::format::{Format, Swizzle, Swizzles};
pub use self::extent::*;

pub use self::image::GliImage;
pub use self::texture::*;

pub use self::load::*;
pub use self::save::*;

pub use self::error::{Result, Error};

pub mod target;
pub mod dx;
pub mod gl;
pub mod sampler;

mod load;
mod save;
mod texture;
mod image;
mod format;
mod extent;
mod ffi;
mod error;


pub enum TexFormatType {
    DDS,
    KTX,
    KMG,
}
