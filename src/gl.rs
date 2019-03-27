
use crate::ffi::root::gli;
use crate::ffi::root::bindings::GL as bindings;

/// Translation class to convert GLI enums into OpenGL values.
pub struct GLConverter {
    inner: gli::gl,
}

impl GLConverter {

    pub fn new(profile: GLProfile) -> GLConverter {
        GLConverter { inner: unsafe { bindings::new_gl_converter(profile.0) } }
    }

    /// Convert GLI targets into OpenGL texture targets.
    pub fn translate(&self, target: crate::target::Target) -> GLTarget {
        let raw = unsafe { bindings::gl_translate(&self.inner, target.0) };
        GLTarget(raw)
    }

    /// Convert GLI formats into OpenGL texture formats.
    pub fn translate2(&self, format: crate::format::Format, swizzles: &[u32; 4]) -> GLFormat {
        let raw = unsafe { bindings::gl_translate1(&self.inner, format.0, swizzles) };
        GLFormat::from(raw)
    }

    /// Convert an OpenGL format into a GLI format.
    pub fn find(&mut self, internal: GLInternalFmt, external: GLExternalFmt, typ: GLTypeFmt) -> crate::format::Format {
        let raw = unsafe { bindings::gl_find(&mut self.inner, internal.0, external.0, typ.0) };
        crate::format::Format(raw)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLProfile(pub(crate) gli::gl_profile);

impl GLProfile {
    pub const ES20 : GLProfile = GLProfile(gli::gl_profile_PROFILE_ES20);
    pub const ES30 : GLProfile = GLProfile(gli::gl_profile_PROFILE_ES30);
    pub const GL32 : GLProfile = GLProfile(gli::gl_profile_PROFILE_GL32);
    pub const GL33 : GLProfile = GLProfile(gli::gl_profile_PROFILE_GL33);
    pub const KTX  : GLProfile = GLProfile(gli::gl_profile_PROFILE_KTX);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLTarget(pub(crate) gli::gl_target);

impl GLTarget {
    pub const TARGET_1D         : GLTarget = GLTarget(gli::gl_target_TARGET_1D);
    pub const TARGET_1D_ARRAY   : GLTarget = GLTarget(gli::gl_target_TARGET_1D_ARRAY);
    pub const TARGET_2D         : GLTarget = GLTarget(gli::gl_target_TARGET_2D);
    pub const TARGET_2D_ARRAY   : GLTarget = GLTarget(gli::gl_target_TARGET_2D_ARRAY);
    pub const TARGET_3D         : GLTarget = GLTarget(gli::gl_target_TARGET_3D);
    pub const TARGET_RECT       : GLTarget = GLTarget(gli::gl_target_TARGET_RECT);
    pub const TARGET_RECT_ARRAY : GLTarget = GLTarget(gli::gl_target_TARGET_RECT_ARRAY);
    pub const TARGET_CUBE       : GLTarget = GLTarget(gli::gl_target_TARGET_CUBE);
    pub const TARGET_CUBE_ARRAY : GLTarget = GLTarget(gli::gl_target_TARGET_CUBE_ARRAY);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLInternalFmt(pub(crate) gli::gl_internal_format);

impl GLInternalFmt {
    pub const RGB_UNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_UNORM);
    pub const BGR_UNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_BGR_UNORM);
    pub const RGBA_UNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_UNORM);
    pub const BGRA_UNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_BGRA_UNORM);
    pub const BGRA8_UNORM                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_BGRA8_UNORM);
    pub const R8_UNORM                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8_UNORM);
    pub const RG8_UNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8_UNORM);
    pub const RGB8_UNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8_UNORM);
    pub const RGBA8_UNORM                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8_UNORM);
    pub const R16_UNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16_UNORM);
    pub const RG16_UNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16_UNORM);
    pub const RGB16_UNORM                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16_UNORM);
    pub const RGBA16_UNORM                   : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16_UNORM);
    pub const RGB10A2_UNORM                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2_UNORM);
    pub const RGB10A2_SNORM_EXT              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2_SNORM_EXT);
    pub const R8_SNORM                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8_SNORM);
    pub const RG8_SNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8_SNORM);
    pub const RGB8_SNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8_SNORM);
    pub const RGBA8_SNORM                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8_SNORM);
    pub const R16_SNORM                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16_SNORM);
    pub const RG16_SNORM                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16_SNORM);
    pub const RGB16_SNORM                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16_SNORM);
    pub const RGBA16_SNORM                   : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16_SNORM);
    pub const R8U                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8U);
    pub const RG8U                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8U);
    pub const RGB8U                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8U);
    pub const RGBA8U                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8U);
    pub const R16U                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16U);
    pub const RG16U                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16U);
    pub const RGB16U                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16U);
    pub const RGBA16U                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16U);
    pub const R32U                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R32U);
    pub const RG32U                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG32U);
    pub const RGB32U                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB32U);
    pub const RGBA32U                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA32U);
    pub const RGB10A2U                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2U);
    pub const RGB10A2I_EXT                   : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2I_EXT);
    pub const R8I                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8I);
    pub const RG8I                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8I);
    pub const RGB8I                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8I);
    pub const RGBA8I                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8I);
    pub const R16I                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16I);
    pub const RG16I                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16I);
    pub const RGB16I                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16I);
    pub const RGBA16I                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16I);
    pub const R32I                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R32I);
    pub const RG32I                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG32I);
    pub const RGB32I                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB32I);
    pub const RGBA32I                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA32I);
    pub const R16F                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16F);
    pub const RG16F                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16F);
    pub const RGB16F                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16F);
    pub const RGBA16F                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16F);
    pub const R32F                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R32F);
    pub const RG32F                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG32F);
    pub const RGB32F                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB32F);
    pub const RGBA32F                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA32F);
    pub const R64F_EXT                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R64F_EXT);
    pub const RG64F_EXT                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG64F_EXT);
    pub const RGB64F_EXT                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB64F_EXT);
    pub const RGBA64F_EXT                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA64F_EXT);
    pub const SR8                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SR8);
    pub const SRG8                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRG8);
    pub const SRGB8                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8);
    pub const SRGB8_ALPHA8                   : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8);
    pub const RGB9E5                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB9E5);
    pub const RG11B10F                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG11B10F);
    pub const RG3B2                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG3B2);
    pub const R5G6B5                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R5G6B5);
    pub const RGB5A1                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB5A1);
    pub const RGBA4                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA4);
    pub const RG4_EXT                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG4_EXT);
    pub const LA4                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LA4);
    pub const L8                             : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_L8);
    pub const A8                             : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_A8);
    pub const LA8                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LA8);
    pub const L16                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_L16);
    pub const A16                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_A16);
    pub const LA16                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LA16);
    pub const D16                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D16);
    pub const D24                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D24);
    pub const D16S8_EXT                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D16S8_EXT);
    pub const D24S8                          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D24S8);
    pub const D32                            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D32);
    pub const D32F                           : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D32F);
    pub const D32FS8X24                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_D32FS8X24);
    pub const S8_EXT                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_S8_EXT);
    pub const RGB_DXT1                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_DXT1);
    pub const RGBA_DXT1                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_DXT1);
    pub const RGBA_DXT3                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_DXT3);
    pub const RGBA_DXT5                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_DXT5);
    pub const R_ATI1N_UNORM                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R_ATI1N_UNORM);
    pub const R_ATI1N_SNORM                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R_ATI1N_SNORM);
    pub const RG_ATI2N_UNORM                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG_ATI2N_UNORM);
    pub const RG_ATI2N_SNORM                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG_ATI2N_SNORM);
    pub const RGB_BP_UNSIGNED_FLOAT          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_BP_UNSIGNED_FLOAT);
    pub const RGB_BP_SIGNED_FLOAT            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_BP_SIGNED_FLOAT);
    pub const RGB_BP_UNORM                   : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_BP_UNORM);
    pub const RGB_PVRTC_4BPPV1               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_PVRTC_4BPPV1);
    pub const RGB_PVRTC_2BPPV1               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_PVRTC_2BPPV1);
    pub const RGBA_PVRTC_4BPPV1              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_PVRTC_4BPPV1);
    pub const RGBA_PVRTC_2BPPV1              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_PVRTC_2BPPV1);
    pub const RGBA_PVRTC_4BPPV2              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_PVRTC_4BPPV2);
    pub const RGBA_PVRTC_2BPPV2              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_PVRTC_2BPPV2);
    pub const ATC_RGB                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_ATC_RGB);
    pub const ATC_RGBA_EXPLICIT_ALPHA        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_ATC_RGBA_EXPLICIT_ALPHA);
    pub const ATC_RGBA_INTERPOLATED_ALPHA    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_ATC_RGBA_INTERPOLATED_ALPHA);
    pub const RGB_ETC                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_ETC);
    pub const RGB_ETC2                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB_ETC2);
    pub const RGBA_PUNCHTHROUGH_ETC2         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_PUNCHTHROUGH_ETC2);
    pub const RGBA_ETC2                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ETC2);
    pub const R11_EAC                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R11_EAC);
    pub const SIGNED_R11_EAC                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SIGNED_R11_EAC);
    pub const RG11_EAC                       : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG11_EAC);
    pub const SIGNED_RG11_EAC                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SIGNED_RG11_EAC);
    pub const RGBA_ASTC_4X4                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_4x4);
    pub const RGBA_ASTC_5X4                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_5x4);
    pub const RGBA_ASTC_5X5                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_5x5);
    pub const RGBA_ASTC_6X5                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_6x5);
    pub const RGBA_ASTC_6X6                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_6x6);
    pub const RGBA_ASTC_8X5                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_8x5);
    pub const RGBA_ASTC_8X6                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_8x6);
    pub const RGBA_ASTC_8X8                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_8x8);
    pub const RGBA_ASTC_10X5                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_10x5);
    pub const RGBA_ASTC_10X6                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_10x6);
    pub const RGBA_ASTC_10X8                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_10x8);
    pub const RGBA_ASTC_10X10                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_10x10);
    pub const RGBA_ASTC_12X10                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_12x10);
    pub const RGBA_ASTC_12X12                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA_ASTC_12x12);
    pub const SRGB_DXT1                      : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_DXT1);
    pub const SRGB_ALPHA_DXT1                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_DXT1);
    pub const SRGB_ALPHA_DXT3                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_DXT3);
    pub const SRGB_ALPHA_DXT5                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_DXT5);
    pub const SRGB_BP_UNORM                  : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_BP_UNORM);
    pub const SRGB_PVRTC_2BPPV1              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_PVRTC_2BPPV1);
    pub const SRGB_PVRTC_4BPPV1              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_PVRTC_4BPPV1);
    pub const SRGB_ALPHA_PVRTC_2BPPV1        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_PVRTC_2BPPV1);
    pub const SRGB_ALPHA_PVRTC_4BPPV1        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_PVRTC_4BPPV1);
    pub const SRGB_ALPHA_PVRTC_2BPPV2        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_PVRTC_2BPPV2);
    pub const SRGB_ALPHA_PVRTC_4BPPV2        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB_ALPHA_PVRTC_4BPPV2);
    pub const SRGB8_ETC2                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ETC2);
    pub const SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2);
    pub const SRGB8_ALPHA8_ETC2_EAC          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ETC2_EAC);
    pub const SRGB8_ALPHA8_ASTC_4X4          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_4x4);
    pub const SRGB8_ALPHA8_ASTC_5X4          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_5x4);
    pub const SRGB8_ALPHA8_ASTC_5X5          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_5x5);
    pub const SRGB8_ALPHA8_ASTC_6X5          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_6x5);
    pub const SRGB8_ALPHA8_ASTC_6X6          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_6x6);
    pub const SRGB8_ALPHA8_ASTC_8X5          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_8x5);
    pub const SRGB8_ALPHA8_ASTC_8X6          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_8x6);
    pub const SRGB8_ALPHA8_ASTC_8X8          : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_8x8);
    pub const SRGB8_ALPHA8_ASTC_10X5         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_10x5);
    pub const SRGB8_ALPHA8_ASTC_10X6         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_10x6);
    pub const SRGB8_ALPHA8_ASTC_10X8         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_10x8);
    pub const SRGB8_ALPHA8_ASTC_10X10        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_10x10);
    pub const SRGB8_ALPHA8_ASTC_12X10        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_12x10);
    pub const SRGB8_ALPHA8_ASTC_12X12        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_SRGB8_ALPHA8_ASTC_12x12);
    pub const ALPHA8                         : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_ALPHA8);
    pub const ALPHA16                        : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_ALPHA16);
    pub const LUMINANCE8                     : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LUMINANCE8);
    pub const LUMINANCE16                    : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LUMINANCE16);
    pub const LUMINANCE8_ALPHA8              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LUMINANCE8_ALPHA8);
    pub const LUMINANCE16_ALPHA16            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_LUMINANCE16_ALPHA16);
    pub const R8_USCALED_GTC                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8_USCALED_GTC);
    pub const R8_SSCALED_GTC                 : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R8_SSCALED_GTC);
    pub const RG8_USCALED_GTC                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8_USCALED_GTC);
    pub const RG8_SSCALED_GTC                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG8_SSCALED_GTC);
    pub const RGB8_USCALED_GTC               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8_USCALED_GTC);
    pub const RGB8_SSCALED_GTC               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB8_SSCALED_GTC);
    pub const RGBA8_USCALED_GTC              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8_USCALED_GTC);
    pub const RGBA8_SSCALED_GTC              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA8_SSCALED_GTC);
    pub const RGB10A2_USCALED_GTC            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2_USCALED_GTC);
    pub const RGB10A2_SSCALED_GTC            : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB10A2_SSCALED_GTC);
    pub const R16_USCALED_GTC                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16_USCALED_GTC);
    pub const R16_SSCALED_GTC                : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_R16_SSCALED_GTC);
    pub const RG16_USCALED_GTC               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16_USCALED_GTC);
    pub const RG16_SSCALED_GTC               : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RG16_SSCALED_GTC);
    pub const RGB16_USCALED_GTC              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16_USCALED_GTC);
    pub const RGB16_SSCALED_GTC              : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGB16_SSCALED_GTC);
    pub const RGBA16_USCALED_GTC             : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16_USCALED_GTC);
    pub const RGBA16_SSCALED_GTC             : GLInternalFmt = GLInternalFmt(gli::gl_internal_format_INTERNAL_RGBA16_SSCALED_GTC);
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLExternalFmt(pub(crate) gli::gl_external_format);

impl GLExternalFmt {
    pub const NONE            : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_NONE);
    pub const RED             : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RED);
    pub const RG              : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RG);
    pub const RGB             : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RGB);
    pub const BGR             : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_BGR);
    pub const RGBA            : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RGBA);
    pub const BGRA            : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_BGRA);
    pub const RED_INTEGER     : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RED_INTEGER);
    pub const RG_INTEGER      : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RG_INTEGER);
    pub const RGB_INTEGER     : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RGB_INTEGER);
    pub const BGR_INTEGER     : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_BGR_INTEGER);
    pub const RGBA_INTEGER    : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_RGBA_INTEGER);
    pub const BGRA_INTEGER    : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_BGRA_INTEGER);
    pub const DEPTH           : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_DEPTH);
    pub const DEPTH_STENCIL   : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_DEPTH_STENCIL);
    pub const STENCIL         : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_STENCIL);
    pub const LUMINANCE       : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_LUMINANCE);
    pub const ALPHA           : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_ALPHA);
    pub const LUMINANCE_ALPHA : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_LUMINANCE_ALPHA);
    pub const SRGB_EXT        : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_SRGB_EXT);
    pub const SRGB_ALPHA_EXT  : GLExternalFmt = GLExternalFmt(gli::gl_external_format_EXTERNAL_SRGB_ALPHA_EXT);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLTypeFmt(pub(crate) gli::gl_type_format);

impl GLTypeFmt {
    pub const NONE                : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_NONE);
    pub const I8                  : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_I8);
    pub const U8                  : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_U8);
    pub const I16                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_I16);
    pub const U16                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_U16);
    pub const I32                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_I32);
    pub const U32                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_U32);
    pub const I64                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_I64);
    pub const U64                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_U64);
    pub const F16                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_F16);
    pub const F16_OES             : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_F16_OES);
    pub const F32                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_F32);
    pub const F64                 : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_F64);
    pub const UINT32_RGB9_E5_REV  : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RGB9_E5_REV);
    pub const UINT32_RG11B10F_REV : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RG11B10F_REV);
    pub const UINT8_RG3B2         : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT8_RG3B2);
    pub const UINT8_RG3B2_REV     : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT8_RG3B2_REV);
    pub const UINT16_RGB5A1       : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_RGB5A1);
    pub const UINT16_RGB5A1_REV   : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_RGB5A1_REV);
    pub const UINT16_R5G6B5       : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_R5G6B5);
    pub const UINT16_R5G6B5_REV   : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_R5G6B5_REV);
    pub const UINT16_RGBA4        : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_RGBA4);
    pub const UINT16_RGBA4_REV    : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_RGBA4_REV);
    pub const UINT32_RGBA8        : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RGBA8);
    pub const UINT32_RGBA8_REV    : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RGBA8_REV);
    pub const UINT32_RGB10A2      : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RGB10A2);
    pub const UINT32_RGB10A2_REV  : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT32_RGB10A2_REV);
    pub const UINT8_RG4_REV_GTC   : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT8_RG4_REV_GTC);
    pub const UINT16_A1RGB5_GTC   : GLTypeFmt = GLTypeFmt(gli::gl_type_format_TYPE_UINT16_A1RGB5_GTC);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GLSwizzle(pub(crate) gli::gl_swizzle);

impl GLSwizzle {
    pub const RED   : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_RED);
    pub const GREEN : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_GREEN);
    pub const BLUE  : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_BLUE);
    pub const ALPHA : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_ALPHA);
    pub const ZERO  : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_ZERO);
    pub const ONE   : GLSwizzle = GLSwizzle(gli::gl_swizzle_SWIZZLE_ONE);
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GLFormat {
    pub internal: GLInternalFmt,
    pub external: GLExternalFmt,
    pub typ     : GLTypeFmt,
    pub swizzles: [u32; 4],
}

impl From<gli::gl_format> for GLFormat {

    fn from(v: gli::gl_format) -> GLFormat {
        GLFormat {
            internal: GLInternalFmt(v.Internal),
            external: GLExternalFmt(v.External),
            typ     : GLTypeFmt(v.Type),
            swizzles: v.Swizzles,
        }
    }
}

impl From<GLFormat> for gli::gl_format {

    fn from(v: GLFormat) -> gli::gl_format {
        gli::gl_format {
            Internal: v.internal.0,
            External: v.external.0,
            Type    : v.typ.0,
            Swizzles: v.swizzles,
        }
    }
}
