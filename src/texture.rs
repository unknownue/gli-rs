
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

use crate::ffi::root::bindings::Texture as bindings;
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

    /// Return the base face of the texture instance, effectively a memory offset in the actual texture storage_type
    /// to identify where to start reading the faces.
    fn base_face(&self) -> usize {
        unsafe { bindings::texture_base_face(self.raw_texture()) }
    }

    /// Return the base layer of the texture instance, effectively a memory offset in the actual texture storage_type
    /// to identify where to start reading the layers.
    fn base_layer(&self) -> usize {
        unsafe { bindings::texture_base_layer(self.raw_texture()) }
    }

    /// Return the base level of the texture instance, effectively a memory offset in the actual texture storage_type
    /// to identify where to start reading the levels.
    fn base_level(&self) -> usize {
        unsafe { bindings::texture_base_level(self.raw_texture()) }
    }

    /// Clear the entire texture storage_linear with zeros.
    fn clear(&mut self) {
        unsafe { bindings::texture_clear(self.raw_texture_mut()) }
    }

    // TODO: Other 3 clear methods is missing.

    /// Copy a specific image of a texture.
    fn copy(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, dst_layer: usize, dst_face: usize, dst_level: usize) {
        unsafe {
            bindings::texture_copy(
                self.raw_texture_mut(), src_texture.raw_texture(), src_layer, src_face, src_level, dst_layer, dst_face, dst_level)
        }
    }

    /// Copy a subset of a specific image of a texture.
    fn copy_subset(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, src_offset: Extent3d, dst_layer: usize, dst_face: usize, dst_level: usize, dst_offset: Extent3d, extent: Extent3d) {

        unsafe {
            bindings::texture_copy_subset(
                self.raw_texture_mut(), src_texture.raw_texture(), src_layer, src_face, src_level, &src_offset.into(), dst_layer, dst_face, dst_level, &dst_offset.into(), &extent.into())
        }
    }

    /// Return a pointer to the beginning of the texture instance data.
    fn data(&self) -> *const c_void {
        unsafe { bindings::texture_data(self.raw_texture()) }
    }

    fn data_mut(&mut self) -> *mut c_void {
        unsafe { bindings::texture_data_mut(self.raw_texture_mut()) }
    }

    // TODO: Other 6 data methods is missing.

    /// Return whether the texture instance is empty, no storage_type or description have been assigned to the instance.
    fn empty(&self) -> bool {
        unsafe { bindings::texture_empty(self.raw_texture()) }
    }

    /// Return max_face() - base_face() + 1.
    fn faces(&self) -> usize {
        unsafe { bindings::texture_faces(self.raw_texture()) }
    }

    /// Return the texture instance format.
    fn format(&self) -> Format {
        let format = unsafe { bindings::texture_format(self.raw_texture()) };
        Format(format)
    }

    /// Return max_layer() - base_layer() + 1.
    fn layers(&self) -> usize {
        unsafe { bindings::texture_layers(self.raw_texture()) }
    }

    /// Return max_level() - base_level() + 1.
    fn levels(&self) -> usize {
        unsafe { bindings::texture_levels(self.raw_texture()) }
    }

    // TODO: load(..) method is missing, due to template specialization.
    // fn load();

    /// Return the max face of the texture instance, effectively a memory offset to the beginning of the last face
    /// in the actual texture storage_type that the texture instance can access.
    fn max_face(&self) -> usize {
        unsafe { bindings::texture_max_face(self.raw_texture()) }
    }

    /// Return the max layer of the texture instance, effectively a memory offset to the beginning of the last layer
    /// in the actual texture storage_type that the texture instance can access.
    fn max_layer(&self) -> usize {
        unsafe { bindings::texture_max_layer(self.raw_texture()) }
    }

    /// Return the max level of the texture instance, effectively a memory offset to the beginning of the last level
    /// in the actual texture storage_type that the texture instance can access.
    fn max_level(&self) -> usize {
        unsafe { bindings::texture_max_level(self.raw_texture()) }
    }

    /// Return the memory size of a texture instance storage_type in bytes.
    fn size(&self) -> usize {
        unsafe { bindings::texture_size(self.raw_texture()) }
    }

    // TODO: another size(&self) method is missing, due to template specialization.

    /// Return the memory size of a specific level identified by Level.
    fn size_at_level(&self, level: usize) -> usize {
        unsafe { bindings::texture_size_level(self.raw_texture(), level) }
    }

    // TODO: another size_at_level(&self, level: usize) method is missing, due to template specialization.

    // TODO: store(..) methods is missing, due to template specialization.

    // TODO: swizzle(&self) methods is missing, due to template specialization.

    /// Return the target of a texture instance.
    fn target(&self) -> Target {
        Self::TARGET_TYPE
    }
}

impl Drop for crate::ffi::root::gli::texture {

    fn drop(&mut self) {

        // In original gli::texture class(C++), it contains member wrapped with std::shared_ptr.
        // Here manually call destructor(`~texture()`) on texture object to decrease shared_ptr counter for its inner member.
        // Dangerous operation. This operation is not fully tested.
        // It does make sense since Rust can't dual with the class member with shared_ptr in texture class.
        // If you find better method to dual with this problem, welcome to create an issue on github.
        unsafe {
            bindings::destroy_texture(self)
        }
    }
}
