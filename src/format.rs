
use crate::ffi::root::gli;

pub struct Format(gli::format);

impl Format {

    /// Evaluate whether a format value is value or not.
    #[inline]
    pub fn is_valid(&self) -> bool {
        self.0 >= Format::FIRST.0 && self.0 <= Format::LAST.0
    }

    /// Evaluate whether a format is compressed with an S3TC algorithm.
    #[inline]
    pub fn is_compressed(&self) -> bool {
        unsafe { gli::is_compressed(self.0) }
    }

    /// Evaluate whether a format is compressed with an S3TC algorithm.
    #[inline]
    pub fn is_s3tc_compressed(&self) -> bool {
        unsafe { gli::is_s3tc_compressed(self.0) }
    }

    /// Evaluate whether a format stores sRGB color space values.
    #[inline]
    pub fn is_srgb(&self) -> bool {
        unsafe { gli::is_srgb(self.0) }
    }

    // TODO: Implement block_size, block_extent and component_count.
//    /// Return the size in bytes of a block for a format.
//    #[inline]
//    pub fn block_size(&self) -> usize {
//        unsafe { gli::block_size(self.0) }
//    }

//    /// Return the dimensions in texels of the block for a format
//    #[inline]
//    pub fn block_extent(&self) -> [u32; 3usize] {
//
//    }

//    /// Return the number of components of a format
//    #[inline]
//    size_t component_count(&self) -> bool {}

    /// Evaluate whether a format is unsigned.
    #[inline]
    pub fn is_unsigned(&self) -> bool {
        unsafe { gli::is_unsigned(self.0) }
    }

    /// Evaluate whether a format is signed.
    #[inline]
    pub fn is_signed(&self) -> bool {
        unsafe { gli::is_signed(self.0) }
    }

    /// Evaluate whether the format is an integer format.
    #[inline]
    pub fn is_integer(&self) -> bool {
        unsafe { gli::is_integer(self.0) }
    }

    /// Evaluate whether the format is a signed integer format.
    #[inline]
    pub fn is_signed_integer(&self) -> bool {
        unsafe { gli::is_signed_integer(self.0) }
    }

    /// Evaluate whether the format is an unsigned integer format.
    #[inline]
    pub fn is_unsigned_integer(&self) -> bool {
        unsafe { gli::is_unsigned_integer(self.0) }
    }

    /// Evaluate whether the format is an float format.
    #[inline]
    pub fn is_float(&self) -> bool {
        unsafe { gli::is_float(self.0) }
    }

    /// Evaluate whether the format is normalized.
    #[inline]
    pub fn is_normalized(&self) -> bool {
        unsafe { gli::is_normalized(self.0) }
    }

    /// Evaluate whether the format is an unsigned normalized format.
    #[inline]
    pub fn is_unorm(&self) -> bool {
        unsafe { gli::is_unorm(self.0) }
    }

    /// Evaluate whether the format is a signed normalized format.
    #[inline]
    pub fn is_snorm(&self) -> bool {
        unsafe { gli::is_snorm(self.0) }
    }

    /// Evaluate whether the format is packed.
    #[inline]
    pub fn is_packed(&self) -> bool {
        unsafe { gli::is_packed(self.0) }
    }

    /// Evaluate whether the format is a depth format.
    #[inline]
    pub fn is_depth(&self) -> bool {
        unsafe { gli::is_depth(self.0) }
    }

    /// Evaluate whether the format has a stencil component.
    #[inline]
    pub fn is_stencil(&self) -> bool {
        unsafe { gli::is_stencil(self.0) }
    }

    /// Evaluate whether the format has depth and stencil components.
    #[inline]
    pub fn is_depth_stencil(&self) -> bool {
        unsafe { gli::is_depth_stencil(self.0) }
    }
}

pub struct Swizzle(gli::swizzle);

impl Swizzle {

    /// Determine whether the Swizzle value represent a channel.
    #[inline]
    pub fn is_channel(&self) -> bool {
        self.0 >= Swizzle::CHANNEL_FIRST.0 && self.0 <= Swizzle::LAST.0
    }
}

impl Format {
    pub const UNDEFINED: Format = Format(gli::format_FORMAT_UNDEFINED);

    pub const RG4_UNORM_PACK8     : Format = Format(gli::format_FORMAT_RG4_UNORM_PACK8);
    pub const FIRST               : Format = Format(gli::format_FORMAT_FIRST);
    pub const RGBA4_UNORM_PACK16  : Format = Format(gli::format_FORMAT_RGBA4_UNORM_PACK16);
    pub const BGRA4_UNORM_PACK16  : Format = Format(gli::format_FORMAT_BGRA4_UNORM_PACK16);
    pub const R5G6B5_UNORM_PACK16 : Format = Format(gli::format_FORMAT_R5G6B5_UNORM_PACK16);
    pub const B5G6R5_UNORM_PACK16 : Format = Format(gli::format_FORMAT_B5G6R5_UNORM_PACK16);
    pub const RGB5A1_UNORM_PACK16 : Format = Format(gli::format_FORMAT_RGB5A1_UNORM_PACK16);
    pub const BGR5A1_UNORM_PACK16 : Format = Format(gli::format_FORMAT_BGR5A1_UNORM_PACK16);
    pub const A1RGB5_UNORM_PACK16 : Format = Format(gli::format_FORMAT_A1RGB5_UNORM_PACK16);

    pub const R8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_R8_UNORM_PACK8);
    pub const R8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_R8_SNORM_PACK8);
    pub const R8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_R8_USCALED_PACK8);
    pub const R8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_R8_SSCALED_PACK8);
    pub const R8_UINT_PACK8    : Format = Format(gli::format_FORMAT_R8_UINT_PACK8);
    pub const R8_SINT_PACK8    : Format = Format(gli::format_FORMAT_R8_SINT_PACK8);
    pub const R8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_R8_SRGB_PACK8);

    pub const RG8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_RG8_UNORM_PACK8);
    pub const RG8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_RG8_SNORM_PACK8);
    pub const RG8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_RG8_USCALED_PACK8);
    pub const RG8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_RG8_SSCALED_PACK8);
    pub const RG8_UINT_PACK8    : Format = Format(gli::format_FORMAT_RG8_UINT_PACK8);
    pub const RG8_SINT_PACK8    : Format = Format(gli::format_FORMAT_RG8_SINT_PACK8);
    pub const RG8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_RG8_SRGB_PACK8);

    pub const RGB8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_RGB8_UNORM_PACK8);
    pub const RGB8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_RGB8_SNORM_PACK8);
    pub const RGB8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_RGB8_USCALED_PACK8);
    pub const RGB8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_RGB8_SSCALED_PACK8);
    pub const RGB8_UINT_PACK8    : Format = Format(gli::format_FORMAT_RGB8_UINT_PACK8);
    pub const RGB8_SINT_PACK8    : Format = Format(gli::format_FORMAT_RGB8_SINT_PACK8);
    pub const RGB8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_RGB8_SRGB_PACK8);

    pub const BGR8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_BGR8_UNORM_PACK8);
    pub const BGR8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_BGR8_SNORM_PACK8);
    pub const BGR8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_BGR8_USCALED_PACK8);
    pub const BGR8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_BGR8_SSCALED_PACK8);
    pub const BGR8_UINT_PACK8    : Format = Format(gli::format_FORMAT_BGR8_UINT_PACK8);
    pub const BGR8_SINT_PACK8    : Format = Format(gli::format_FORMAT_BGR8_SINT_PACK8);
    pub const BGR8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_BGR8_SRGB_PACK8);

    pub const RGBA8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_RGBA8_UNORM_PACK8);
    pub const RGBA8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_RGBA8_SNORM_PACK8);
    pub const RGBA8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_RGBA8_USCALED_PACK8);
    pub const RGBA8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_RGBA8_SSCALED_PACK8);
    pub const RGBA8_UINT_PACK8    : Format = Format(gli::format_FORMAT_RGBA8_UINT_PACK8);
    pub const RGBA8_SINT_PACK8    : Format = Format(gli::format_FORMAT_RGBA8_SINT_PACK8);
    pub const RGBA8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_RGBA8_SRGB_PACK8);

    pub const BGRA8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_BGRA8_UNORM_PACK8);
    pub const BGRA8_SNORM_PACK8   : Format = Format(gli::format_FORMAT_BGRA8_SNORM_PACK8);
    pub const BGRA8_USCALED_PACK8 : Format = Format(gli::format_FORMAT_BGRA8_USCALED_PACK8);
    pub const BGRA8_SSCALED_PACK8 : Format = Format(gli::format_FORMAT_BGRA8_SSCALED_PACK8);
    pub const BGRA8_UINT_PACK8    : Format = Format(gli::format_FORMAT_BGRA8_UINT_PACK8);
    pub const BGRA8_SINT_PACK8    : Format = Format(gli::format_FORMAT_BGRA8_SINT_PACK8);
    pub const BGRA8_SRGB_PACK8    : Format = Format(gli::format_FORMAT_BGRA8_SRGB_PACK8);

    pub const RGBA8_UNORM_PACK32   : Format = Format(gli::format_FORMAT_RGBA8_UNORM_PACK32);
    pub const RGBA8_SNORM_PACK32   : Format = Format(gli::format_FORMAT_RGBA8_SNORM_PACK32);
    pub const RGBA8_USCALED_PACK32 : Format = Format(gli::format_FORMAT_RGBA8_USCALED_PACK32);
    pub const RGBA8_SSCALED_PACK32 : Format = Format(gli::format_FORMAT_RGBA8_SSCALED_PACK32);
    pub const RGBA8_UINT_PACK32    : Format = Format(gli::format_FORMAT_RGBA8_UINT_PACK32);
    pub const RGBA8_SINT_PACK32    : Format = Format(gli::format_FORMAT_RGBA8_SINT_PACK32);
    pub const RGBA8_SRGB_PACK32    : Format = Format(gli::format_FORMAT_RGBA8_SRGB_PACK32);

    pub const RGB10A2_UNORM_PACK32   : Format = Format(gli::format_FORMAT_RGB10A2_UNORM_PACK32);
    pub const RGB10A2_SNORM_PACK32   : Format = Format(gli::format_FORMAT_RGB10A2_SNORM_PACK32);
    pub const RGB10A2_USCALED_PACK32 : Format = Format(gli::format_FORMAT_RGB10A2_USCALED_PACK32);
    pub const RGB10A2_SSCALED_PACK32 : Format = Format(gli::format_FORMAT_RGB10A2_SSCALED_PACK32);
    pub const RGB10A2_UINT_PACK32    : Format = Format(gli::format_FORMAT_RGB10A2_UINT_PACK32);
    pub const RGB10A2_SINT_PACK32    : Format = Format(gli::format_FORMAT_RGB10A2_SINT_PACK32);

    pub const BGR10A2_UNORM_PACK32   : Format = Format(gli::format_FORMAT_BGR10A2_UNORM_PACK32);
    pub const BGR10A2_SNORM_PACK32   : Format = Format(gli::format_FORMAT_BGR10A2_SNORM_PACK32);
    pub const BGR10A2_USCALED_PACK32 : Format = Format(gli::format_FORMAT_BGR10A2_USCALED_PACK32);
    pub const BGR10A2_SSCALED_PACK32 : Format = Format(gli::format_FORMAT_BGR10A2_SSCALED_PACK32);
    pub const BGR10A2_UINT_PACK32    : Format = Format(gli::format_FORMAT_BGR10A2_UINT_PACK32);
    pub const BGR10A2_SINT_PACK32    : Format = Format(gli::format_FORMAT_BGR10A2_SINT_PACK32);

    pub const R16_UNORM_PACK16   : Format = Format(gli::format_FORMAT_R16_UNORM_PACK16);
    pub const R16_SNORM_PACK16   : Format = Format(gli::format_FORMAT_R16_SNORM_PACK16);
    pub const R16_USCALED_PACK16 : Format = Format(gli::format_FORMAT_R16_USCALED_PACK16);
    pub const R16_SSCALED_PACK16 : Format = Format(gli::format_FORMAT_R16_SSCALED_PACK16);
    pub const R16_UINT_PACK16    : Format = Format(gli::format_FORMAT_R16_UINT_PACK16);
    pub const R16_SINT_PACK16    : Format = Format(gli::format_FORMAT_R16_SINT_PACK16);
    pub const R16_SFLOAT_PACK16  : Format = Format(gli::format_FORMAT_R16_SFLOAT_PACK16);

    pub const RG16_UNORM_PACK16   : Format = Format(gli::format_FORMAT_RG16_UNORM_PACK16);
    pub const RG16_SNORM_PACK16   : Format = Format(gli::format_FORMAT_RG16_SNORM_PACK16);
    pub const RG16_USCALED_PACK16 : Format = Format(gli::format_FORMAT_RG16_USCALED_PACK16);
    pub const RG16_SSCALED_PACK16 : Format = Format(gli::format_FORMAT_RG16_SSCALED_PACK16);
    pub const RG16_UINT_PACK16    : Format = Format(gli::format_FORMAT_RG16_UINT_PACK16);
    pub const RG16_SINT_PACK16    : Format = Format(gli::format_FORMAT_RG16_SINT_PACK16);
    pub const RG16_SFLOAT_PACK16  : Format = Format(gli::format_FORMAT_RG16_SFLOAT_PACK16);

    pub const RGB16_UNORM_PACK16   : Format = Format(gli::format_FORMAT_RGB16_UNORM_PACK16);
    pub const RGB16_SNORM_PACK16   : Format = Format(gli::format_FORMAT_RGB16_SNORM_PACK16);
    pub const RGB16_USCALED_PACK16 : Format = Format(gli::format_FORMAT_RGB16_USCALED_PACK16);
    pub const RGB16_SSCALED_PACK16 : Format = Format(gli::format_FORMAT_RGB16_SSCALED_PACK16);
    pub const RGB16_UINT_PACK16    : Format = Format(gli::format_FORMAT_RGB16_UINT_PACK16);
    pub const RGB16_SINT_PACK16    : Format = Format(gli::format_FORMAT_RGB16_SINT_PACK16);
    pub const RGB16_SFLOAT_PACK16  : Format = Format(gli::format_FORMAT_RGB16_SFLOAT_PACK16);

    pub const RGBA16_UNORM_PACK16   : Format = Format(gli::format_FORMAT_RGBA16_UNORM_PACK16);
    pub const RGBA16_SNORM_PACK16   : Format = Format(gli::format_FORMAT_RGBA16_SNORM_PACK16);
    pub const RGBA16_USCALED_PACK16 : Format = Format(gli::format_FORMAT_RGBA16_USCALED_PACK16);
    pub const RGBA16_SSCALED_PACK16 : Format = Format(gli::format_FORMAT_RGBA16_SSCALED_PACK16);
    pub const RGBA16_UINT_PACK16    : Format = Format(gli::format_FORMAT_RGBA16_UINT_PACK16);
    pub const RGBA16_SINT_PACK16    : Format = Format(gli::format_FORMAT_RGBA16_SINT_PACK16);
    pub const RGBA16_SFLOAT_PACK16  : Format = Format(gli::format_FORMAT_RGBA16_SFLOAT_PACK16);

    pub const R32_UINT_PACK32   : Format = Format(gli::format_FORMAT_R32_UINT_PACK32);
    pub const R32_SINT_PACK32   : Format = Format(gli::format_FORMAT_R32_SINT_PACK32);
    pub const R32_SFLOAT_PACK32 : Format = Format(gli::format_FORMAT_R32_SFLOAT_PACK32);

    pub const RG32_UINT_PACK32   : Format = Format(gli::format_FORMAT_RG32_UINT_PACK32);
    pub const RG32_SINT_PACK32   : Format = Format(gli::format_FORMAT_RG32_SINT_PACK32);
    pub const RG32_SFLOAT_PACK32 : Format = Format(gli::format_FORMAT_RG32_SFLOAT_PACK32);

    pub const RGB32_UINT_PACK32   : Format = Format(gli::format_FORMAT_RGB32_UINT_PACK32);
    pub const RGB32_SINT_PACK32   : Format = Format(gli::format_FORMAT_RGB32_SINT_PACK32);
    pub const RGB32_SFLOAT_PACK32 : Format = Format(gli::format_FORMAT_RGB32_SFLOAT_PACK32);

    pub const RGBA32_UINT_PACK32   : Format = Format(gli::format_FORMAT_RGBA32_UINT_PACK32);
    pub const RGBA32_SINT_PACK32   : Format = Format(gli::format_FORMAT_RGBA32_SINT_PACK32);
    pub const RGBA32_SFLOAT_PACK32 : Format = Format(gli::format_FORMAT_RGBA32_SFLOAT_PACK32);

    pub const R64_UINT_PACK64   : Format = Format(gli::format_FORMAT_R64_UINT_PACK64);
    pub const R64_SINT_PACK64   : Format = Format(gli::format_FORMAT_R64_SINT_PACK64);
    pub const R64_SFLOAT_PACK64 : Format = Format(gli::format_FORMAT_R64_SFLOAT_PACK64);

    pub const RG64_UINT_PACK64   : Format = Format(gli::format_FORMAT_RG64_UINT_PACK64);
    pub const RG64_SINT_PACK64   : Format = Format(gli::format_FORMAT_RG64_SINT_PACK64);
    pub const RG64_SFLOAT_PACK64 : Format = Format(gli::format_FORMAT_RG64_SFLOAT_PACK64);

    pub const RGB64_UINT_PACK64   : Format = Format(gli::format_FORMAT_RGB64_UINT_PACK64);
    pub const RGB64_SINT_PACK64   : Format = Format(gli::format_FORMAT_RGB64_SINT_PACK64);
    pub const RGB64_SFLOAT_PACK64 : Format = Format(gli::format_FORMAT_RGB64_SFLOAT_PACK64);

    pub const RGBA64_UINT_PACK64   : Format = Format(gli::format_FORMAT_RGBA64_UINT_PACK64);
    pub const RGBA64_SINT_PACK64   : Format = Format(gli::format_FORMAT_RGBA64_SINT_PACK64);
    pub const RGBA64_SFLOAT_PACK64 : Format = Format(gli::format_FORMAT_RGBA64_SFLOAT_PACK64);

    pub const RG11B10_UFLOAT_PACK32 : Format = Format(gli::format_FORMAT_RG11B10_UFLOAT_PACK32);
    pub const RGB9E5_UFLOAT_PACK32  : Format = Format(gli::format_FORMAT_RGB9E5_UFLOAT_PACK32);

    pub const D16_UNORM_PACK16          : Format = Format(gli::format_FORMAT_D16_UNORM_PACK16);
    pub const D24_UNORM_PACK32          : Format = Format(gli::format_FORMAT_D24_UNORM_PACK32);
    pub const D32_SFLOAT_PACK32         : Format = Format(gli::format_FORMAT_D32_SFLOAT_PACK32);
    pub const S8_UINT_PACK8             : Format = Format(gli::format_FORMAT_S8_UINT_PACK8);
    pub const D16_UNORM_S8_UINT_PACK32  : Format = Format(gli::format_FORMAT_D16_UNORM_S8_UINT_PACK32);
    pub const D24_UNORM_S8_UINT_PACK32  : Format = Format(gli::format_FORMAT_D24_UNORM_S8_UINT_PACK32);
    pub const D32_SFLOAT_S8_UINT_PACK64 : Format = Format(gli::format_FORMAT_D32_SFLOAT_S8_UINT_PACK64);

    pub const RGB_DXT1_UNORM_BLOCK8   : Format = Format(gli::format_FORMAT_RGB_DXT1_UNORM_BLOCK8);
    pub const RGB_DXT1_SRGB_BLOCK8    : Format = Format(gli::format_FORMAT_RGB_DXT1_SRGB_BLOCK8);
    pub const RGBA_DXT1_UNORM_BLOCK8  : Format = Format(gli::format_FORMAT_RGBA_DXT1_UNORM_BLOCK8);
    pub const RGBA_DXT1_SRGB_BLOCK8   : Format = Format(gli::format_FORMAT_RGBA_DXT1_SRGB_BLOCK8);
    pub const RGBA_DXT3_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_DXT3_UNORM_BLOCK16);
    pub const RGBA_DXT3_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_DXT3_SRGB_BLOCK16);
    pub const RGBA_DXT5_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_DXT5_UNORM_BLOCK16);
    pub const RGBA_DXT5_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_DXT5_SRGB_BLOCK16);
    pub const R_ATI1N_UNORM_BLOCK8    : Format = Format(gli::format_FORMAT_R_ATI1N_UNORM_BLOCK8);
    pub const R_ATI1N_SNORM_BLOCK8    : Format = Format(gli::format_FORMAT_R_ATI1N_SNORM_BLOCK8);
    pub const RG_ATI2N_UNORM_BLOCK16  : Format = Format(gli::format_FORMAT_RG_ATI2N_UNORM_BLOCK16);
    pub const RG_ATI2N_SNORM_BLOCK16  : Format = Format(gli::format_FORMAT_RG_ATI2N_SNORM_BLOCK16);
    pub const RGB_BP_UFLOAT_BLOCK16   : Format = Format(gli::format_FORMAT_RGB_BP_UFLOAT_BLOCK16);
    pub const RGB_BP_SFLOAT_BLOCK16   : Format = Format(gli::format_FORMAT_RGB_BP_SFLOAT_BLOCK16);
    pub const RGBA_BP_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_BP_UNORM_BLOCK16);
    pub const RGBA_BP_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_BP_SRGB_BLOCK16);

    pub const RGB_ETC2_UNORM_BLOCK8   : Format = Format(gli::format_FORMAT_RGB_ETC2_UNORM_BLOCK8);
    pub const RGB_ETC2_SRGB_BLOCK8    : Format = Format(gli::format_FORMAT_RGB_ETC2_SRGB_BLOCK8);
    pub const RGBA_ETC2_UNORM_BLOCK8  : Format = Format(gli::format_FORMAT_RGBA_ETC2_UNORM_BLOCK8);
    pub const RGBA_ETC2_SRGB_BLOCK8   : Format = Format(gli::format_FORMAT_RGBA_ETC2_SRGB_BLOCK8);
    pub const RGBA_ETC2_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ETC2_UNORM_BLOCK16);
    pub const RGBA_ETC2_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ETC2_SRGB_BLOCK16);
    pub const R_EAC_UNORM_BLOCK8      : Format = Format(gli::format_FORMAT_R_EAC_UNORM_BLOCK8);
    pub const R_EAC_SNORM_BLOCK8      : Format = Format(gli::format_FORMAT_R_EAC_SNORM_BLOCK8);
    pub const RG_EAC_UNORM_BLOCK16    : Format = Format(gli::format_FORMAT_RG_EAC_UNORM_BLOCK16);
    pub const RG_EAC_SNORM_BLOCK16    : Format = Format(gli::format_FORMAT_RG_EAC_SNORM_BLOCK16);

    pub const RGBA_ASTC_4X4_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_4X4_UNORM_BLOCK16);
    pub const RGBA_ASTC_4X4_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_4X4_SRGB_BLOCK16);
    pub const RGBA_ASTC_5X4_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_5X4_UNORM_BLOCK16);
    pub const RGBA_ASTC_5X4_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_5X4_SRGB_BLOCK16);
    pub const RGBA_ASTC_5X5_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_5X5_UNORM_BLOCK16);
    pub const RGBA_ASTC_5X5_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_5X5_SRGB_BLOCK16);
    pub const RGBA_ASTC_6X5_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_6X5_UNORM_BLOCK16);
    pub const RGBA_ASTC_6X5_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_6X5_SRGB_BLOCK16);
    pub const RGBA_ASTC_6X6_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_6X6_UNORM_BLOCK16);
    pub const RGBA_ASTC_6X6_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_6X6_SRGB_BLOCK16);
    pub const RGBA_ASTC_8X5_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X5_UNORM_BLOCK16);
    pub const RGBA_ASTC_8X5_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X5_SRGB_BLOCK16);
    pub const RGBA_ASTC_8X6_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X6_UNORM_BLOCK16);
    pub const RGBA_ASTC_8X6_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X6_SRGB_BLOCK16);
    pub const RGBA_ASTC_8X8_UNORM_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X8_UNORM_BLOCK16);
    pub const RGBA_ASTC_8X8_SRGB_BLOCK16    : Format = Format(gli::format_FORMAT_RGBA_ASTC_8X8_SRGB_BLOCK16);
    pub const RGBA_ASTC_10X5_UNORM_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X5_UNORM_BLOCK16);
    pub const RGBA_ASTC_10X5_SRGB_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X5_SRGB_BLOCK16);
    pub const RGBA_ASTC_10X6_UNORM_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X6_UNORM_BLOCK16);
    pub const RGBA_ASTC_10X6_SRGB_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X6_SRGB_BLOCK16);
    pub const RGBA_ASTC_10X8_UNORM_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X8_UNORM_BLOCK16);
    pub const RGBA_ASTC_10X8_SRGB_BLOCK16   : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X8_SRGB_BLOCK16);
    pub const RGBA_ASTC_10X10_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X10_UNORM_BLOCK16);
    pub const RGBA_ASTC_10X10_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_10X10_SRGB_BLOCK16);
    pub const RGBA_ASTC_12X10_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ASTC_12X10_UNORM_BLOCK16);
    pub const RGBA_ASTC_12X10_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_12X10_SRGB_BLOCK16);
    pub const RGBA_ASTC_12X12_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ASTC_12X12_UNORM_BLOCK16);
    pub const RGBA_ASTC_12X12_SRGB_BLOCK16  : Format = Format(gli::format_FORMAT_RGBA_ASTC_12X12_SRGB_BLOCK16);

    pub const RGB_PVRTC1_8X8_UNORM_BLOCK32   : Format = Format(gli::format_FORMAT_RGB_PVRTC1_8X8_UNORM_BLOCK32);
    pub const RGB_PVRTC1_8X8_SRGB_BLOCK32    : Format = Format(gli::format_FORMAT_RGB_PVRTC1_8X8_SRGB_BLOCK32);
    pub const RGB_PVRTC1_16X8_UNORM_BLOCK32  : Format = Format(gli::format_FORMAT_RGB_PVRTC1_16X8_UNORM_BLOCK32);
    pub const RGB_PVRTC1_16X8_SRGB_BLOCK32   : Format = Format(gli::format_FORMAT_RGB_PVRTC1_16X8_SRGB_BLOCK32);
    pub const RGBA_PVRTC1_8X8_UNORM_BLOCK32  : Format = Format(gli::format_FORMAT_RGBA_PVRTC1_8X8_UNORM_BLOCK32);
    pub const RGBA_PVRTC1_8X8_SRGB_BLOCK32   : Format = Format(gli::format_FORMAT_RGBA_PVRTC1_8X8_SRGB_BLOCK32);
    pub const RGBA_PVRTC1_16X8_UNORM_BLOCK32 : Format = Format(gli::format_FORMAT_RGBA_PVRTC1_16X8_UNORM_BLOCK32);
    pub const RGBA_PVRTC1_16X8_SRGB_BLOCK32  : Format = Format(gli::format_FORMAT_RGBA_PVRTC1_16X8_SRGB_BLOCK32);
    pub const RGBA_PVRTC2_4X4_UNORM_BLOCK8   : Format = Format(gli::format_FORMAT_RGBA_PVRTC2_4X4_UNORM_BLOCK8);
    pub const RGBA_PVRTC2_4X4_SRGB_BLOCK8    : Format = Format(gli::format_FORMAT_RGBA_PVRTC2_4X4_SRGB_BLOCK8);
    pub const RGBA_PVRTC2_8X4_UNORM_BLOCK8   : Format = Format(gli::format_FORMAT_RGBA_PVRTC2_8X4_UNORM_BLOCK8);
    pub const RGBA_PVRTC2_8X4_SRGB_BLOCK8    : Format = Format(gli::format_FORMAT_RGBA_PVRTC2_8X4_SRGB_BLOCK8);

    pub const RGB_ETC_UNORM_BLOCK8    : Format = Format(gli::format_FORMAT_RGB_ETC_UNORM_BLOCK8);
    pub const RGB_ATC_UNORM_BLOCK8    : Format = Format(gli::format_FORMAT_RGB_ATC_UNORM_BLOCK8);
    pub const RGBA_ATCA_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ATCA_UNORM_BLOCK16);
    pub const RGBA_ATCI_UNORM_BLOCK16 : Format = Format(gli::format_FORMAT_RGBA_ATCI_UNORM_BLOCK16);

    pub const L8_UNORM_PACK8    : Format = Format(gli::format_FORMAT_L8_UNORM_PACK8);
    pub const A8_UNORM_PACK8    : Format = Format(gli::format_FORMAT_A8_UNORM_PACK8);
    pub const LA8_UNORM_PACK8   : Format = Format(gli::format_FORMAT_LA8_UNORM_PACK8);
    pub const L16_UNORM_PACK16  : Format = Format(gli::format_FORMAT_L16_UNORM_PACK16);
    pub const A16_UNORM_PACK16  : Format = Format(gli::format_FORMAT_A16_UNORM_PACK16);
    pub const LA16_UNORM_PACK16 : Format = Format(gli::format_FORMAT_LA16_UNORM_PACK16);

    pub const BGR8_UNORM_PACK32 : Format = Format(gli::format_FORMAT_BGR8_UNORM_PACK32);
    pub const BGR8_SRGB_PACK32  : Format = Format(gli::format_FORMAT_BGR8_SRGB_PACK32);

    pub const RG3B2_UNORM_PACK8 : Format = Format(gli::format_FORMAT_RG3B2_UNORM_PACK8);
    pub const LAST              : Format = Format(gli::format_FORMAT_LAST);
}

impl Swizzle {
    pub const RED           : Swizzle = Swizzle(gli::swizzle_SWIZZLE_RED);
    pub const FIRST         : Swizzle = Swizzle(gli::swizzle_SWIZZLE_FIRST);
    pub const CHANNEL_FIRST : Swizzle = Swizzle(gli::swizzle_SWIZZLE_CHANNEL_FIRST);
    pub const GREEN         : Swizzle = Swizzle(gli::swizzle_SWIZZLE_GREEN);
    pub const BLUE          : Swizzle = Swizzle(gli::swizzle_SWIZZLE_BLUE);
    pub const ALPHA         : Swizzle = Swizzle(gli::swizzle_SWIZZLE_ALPHA);
    pub const CHANNEL_LAST  : Swizzle = Swizzle(gli::swizzle_SWIZZLE_CHANNEL_LAST);
    pub const ZERO          : Swizzle = Swizzle(gli::swizzle_SWIZZLE_ZERO);
    pub const LAST          : Swizzle = Swizzle(gli::swizzle_SWIZZLE_LAST);
}
