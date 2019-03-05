
use crate::ffi::root::gli;


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Target(pub(crate) gli::target);

impl Target {

    /// Check whether a target is a 1D target.
    #[inline]
    pub fn is_target_1d(&self) -> bool {
        match *self {
            | Target::TARGET_1D
            | Target::TARGET_1D_ARRAY => true,
            | _ => false,
        }
    }

    /// Check whether a target is an array target.
    #[inline]
    pub fn is_target_array(&self) -> bool {
        match *self {
            | Target::TARGET_1D_ARRAY
            | Target::TARGET_2D_ARRAY
            | Target::TARGET_CUBE_ARRAY => true,
            | _ => false
        }
    }

    /// Check whether a target is a cube map target.
    #[inline]
    pub fn is_target_cube(&self) -> bool {
        match *self {
            | Target::TARGET_CUBE
            | Target::TARGET_CUBE_ARRAY => true,
            | _ => false,
        }
    }

    /// Check whether a target is a rectangle target.
    #[inline]
    pub fn is_target_rect(&self) -> bool {
        match *self {
            | Target::TARGET_RECT
            | Target::TARGET_RECT_ARRAY => true,
            | _ => false,
        }
    }
}

impl Target {
    pub const TARGET_FIRST      : Target = Target(gli::target_TARGET_FIRST);
    pub const TARGET_1D         : Target = Target(gli::target_TARGET_1D);
    pub const TARGET_1D_ARRAY   : Target = Target(gli::target_TARGET_1D_ARRAY);
    pub const TARGET_2D         : Target = Target(gli::target_TARGET_2D);
    pub const TARGET_2D_ARRAY   : Target = Target(gli::target_TARGET_2D_ARRAY);
    pub const TARGET_3D         : Target = Target(gli::target_TARGET_3D);
    pub const TARGET_RECT       : Target = Target(gli::target_TARGET_RECT);
    pub const TARGET_RECT_ARRAY : Target = Target(gli::target_TARGET_RECT_ARRAY);
    pub const TARGET_CUBE       : Target = Target(gli::target_TARGET_CUBE);
    pub const TARGET_CUBE_ARRAY : Target = Target(gli::target_TARGET_CUBE_ARRAY);
    pub const TARGET_LAST       : Target = Target(gli::target_TARGET_LAST);
}
