
use std::os::raw::c_void;

use crate::ffi::root::gli;
use crate::format::Format;
use crate::target::Target;
use crate::Extent3d;

trait GliTexture {
    type ExtentType;

    /// Return the base face of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the faces.
    fn base_face(&self) -> usize;

    /// Return the base layer of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the layers.
    fn base_layer(&self) -> usize;

    /// Return the base level of the texture instance, effectively a memory offset in the actual texture storage_type to identify where to start reading the levels.
    fn base_level(&self) -> usize;

    /// Clear the entire texture storage_linear with zeros.
    fn clear(&mut self);

    // TODO: Other 3 clear methods is missing.

    /// Copy a specific image of a texture.
    fn copy(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, dst_layer: usize, dst_face: usize, dst_level: usize);

    /// Copy a subset of a specific image of a texture.
    fn copy_subset(&mut self, src_texture: &Self, src_layer: usize, src_face: usize, src_level: usize, src_offset: &Self::ExtentType, dst_layer: usize, dst_face: usize, dst_level: usize, dst_offset: &Self::ExtentType, extent: &Self::ExtentType);

    /// Return a pointer to the beginning of the texture instance data.
    fn data(&self) -> *mut c_void;

    // TODO: Other 7 data methods is missing.

    /// Return whether the texture instance is empty, no storage_type or description have been assigned to the instance.
    fn empty(&self) -> bool;

    /// Return the size of a texture instance: width, height and depth.
    fn extent(&self, level: usize) -> Self::ExtentType;

    /// Return max_face() - base_face() + 1.
    fn faces(&self) -> usize;

    /// Return the texture instance format.
    fn format(&self) -> Format;

    /// Return max_layer() - base_layer() + 1.
    fn layers(&self) -> usize;

    /// Return max_level() - base_level() + 1.
    fn levels(&self) -> usize;

    // TODO: load(..) method is missing, due to template specialization.
    // fn load();

    /// Return the max face of the texture instance, effectively a memory offset to the beginning of the last face in the actual texture storage_type that the texture instance can access.
    fn max_face(&self) -> usize;

    /// Return the max layer of the texture instance, effectively a memory offset to the beginning of the last layer in the actual texture storage_type that the texture instance can access.
    fn max_layer(&self) -> usize;

    /// Return the max level of the texture instance, effectively a memory offset to the beginning of the last level in the actual texture storage_type that the texture instance can access.
    fn max_level(&self) -> usize;

    /// Return the memory size of a texture instance storage_type in bytes.
    fn size(&self) -> usize;

    // TODO: another size(&self) method is missing, due to template specialization.

    /// Return the memory size of a specific level identified by Level.
    fn size_at_level(&self, level: usize) -> usize;

    // TODO: another size_at_level(&self, level: usize) method is missing, due to template specialization.

    // TODO: store(..) methods is missing, due to template specialization.

    // TODO: swizzle(&self) methods is missing, due to template specialization.

    /// Return the target of a texture instance.
    fn target(&self) -> Target;
}

pub struct Texture1D {
    ffi: gli::texture1d,
}

impl Texture1D {

}
