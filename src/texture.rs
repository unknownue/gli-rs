
pub use self::t1d::Texture1D;
pub use self::t2d::Texture2D;
pub use self::t3d::Texture3D;
pub use self::t1d_array::Texture1DArray;
pub use self::t2d_array::Texture2DArray;
pub use self::tcube::TextureCube;
pub use self::tcube_array::TextureCubeArray;

mod t1d;
mod t2d;
mod t3d;
mod t1d_array;
mod t2d_array;
mod tcube;
mod tcube_array;

use std::os::raw::c_void;

use crate::format::{Format, Swizzles};
use crate::target::Target;
use crate::Extent3d;

pub(crate) mod inner {

    use crate::ffi::root::gli::texture as RawTexture;

    pub trait TextureAccessible: From<RawTexture> {
        fn raw_texture(&self) -> &RawTexture;
        fn raw_texture_mut(&mut self) -> &mut RawTexture;
    }
}

pub trait GliTexture: inner::TextureAccessible + Sized {
    const TARGET_TYPE: Target;
    type ExtentType;

    /// Return the size of a texture instance: width, height and depth.
    fn extent(&self, level: usize) -> Self::ExtentType;

    fn set_swizzles(&mut self, swizzles: Swizzles) {
        self.raw_texture_mut().Swizzles = swizzles;
    }

    /// Return the base face of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the faces.
    fn base_face(&self) -> usize {
        unsafe { self.raw_texture().base_face() }
    }

    /// Return the base layer of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the layers.
    fn base_layer(&self) -> usize {
        unsafe { self.raw_texture().base_layer() }
    }

    /// Return the base level of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the levels.
    fn base_level(&self) -> usize {
        unsafe { self.raw_texture().base_level() }
    }

    /// Clear the entire texture storage_linear with zeros.
    fn clear(&mut self) {
        unsafe { self.raw_texture_mut().clear() }
    }

    // TODO: Other 3 clear methods is missing.

    /// Copy a specific image of a texture.
    fn copy(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, dst_layer: usize, dst_face: usize, dst_level: usize) {
        unsafe {
            self.raw_texture_mut()
                .copy(src_texture.raw_texture(), src_layer, src_face, src_level, dst_layer, dst_face, dst_level)
        }
    }

    /// Copy a subset of a specific image of a texture.
    fn copy_subset(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, src_offset: &Extent3d, dst_layer: usize, dst_face: usize, dst_level: usize, dst_offset: &Extent3d, extent: &Extent3d) {
        unsafe {
            self.raw_texture_mut()
                .copy1(src_texture.raw_texture(), src_layer, src_face, src_level, src_offset, dst_layer, dst_face, dst_level, dst_offset, extent)
        }
    }

    /// Return a pointer to the beginning of the texture instance data.
    fn data(&self) -> *const c_void {
        unsafe { self.raw_texture().data1() }
    }

    fn data_mut(&mut self) -> *mut c_void {
        unsafe { self.raw_texture_mut().data() }
    }

    // TODO: Other 6 data methods is missing.

    /// Return whether the texture instance is empty, no storage_type or description have been assigned to the instance.
    fn empty(&self) -> bool {
        unsafe { self.raw_texture().empty() }
    }

    /// Return max_face() - base_face() + 1.
    fn faces(&self) -> usize {
        unsafe { self.raw_texture().faces() }
    }

    /// Return the texture instance format.
    fn format(&self) -> Format {
        let format = unsafe { self.raw_texture().format() };
        Format(format)
    }

    /// Return max_layer() - base_layer() + 1.
    fn layers(&self) -> usize {
        unsafe { self.raw_texture().layers() }
    }

    /// Return max_level() - base_level() + 1.
    fn levels(&self) -> usize {
        unsafe { self.raw_texture().levels() }
    }

    // TODO: load(..) method is missing, due to template specialization.
    // fn load();

    /// Return the max face of the texture instance, effectively a memory offset to the beginning of the last face in the actual texture storage_type that the texture instance can access.
    fn max_face(&self) -> usize {
        unsafe { self.raw_texture().max_face() }
    }

    /// Return the max layer of the texture instance, effectively a memory offset to the beginning of the last layer in the actual texture storage_type that the texture instance can access.
    fn max_layer(&self) -> usize {
        unsafe { self.raw_texture().max_layer() }
    }

    /// Return the max level of the texture instance, effectively a memory offset to the beginning of the last level in the actual texture storage_type that the texture instance can access.
    fn max_level(&self) -> usize {
        unsafe { self.raw_texture().max_level() }
    }

    /// Return the memory size of a texture instance storage_type in bytes.
    fn size(&self) -> usize {
        unsafe { self.raw_texture().size() }
    }

    // TODO: another size(&self) method is missing, due to template specialization.

    /// Return the memory size of a specific level identified by Level.
    fn size_at_level(&self, level: usize) -> usize {
        unsafe { self.raw_texture().size1(level) }
    }

    // TODO: another size_at_level(&self, level: usize) method is missing, due to template specialization.

    // TODO: store(..) methods is missing, due to template specialization.

    // TODO: swizzle(&self) methods is missing, due to template specialization.

    /// Return the target of a texture instance.
    fn target(&self) -> Target {
        Self::TARGET_TYPE
    }
}
