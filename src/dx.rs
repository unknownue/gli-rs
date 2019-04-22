
use crate::ffi::root::gli;
use crate::ffi::root::bindings::DX as bindings;

/// Translation class to convert GLI enums into DirectX enums.
#[repr(transparent)]
pub struct DxConverter {
    inner: gli::dx,
}

impl DxConverter {

    pub fn new() -> DxConverter {
        DxConverter { inner: unsafe { bindings::new_dx_converter() } }
    }

    /// Convert GLI formats into Direct3D formats.
    pub fn translate(&self, format: crate::format::Format) -> DxFormat {

        let raw = unsafe { bindings::dx_translate(&self.inner, format.0) };
        DxFormat::from(raw)
    }

    /// Convert a Direct3D 9 format into a GLI format.
    pub fn find_dx9(&self, d3d_fmt: D3DFormat) -> crate::format::Format {
        let raw = unsafe { bindings::dx_find(&self.inner, d3d_fmt.0) };
        crate::format::Format(raw)
    }

    /// Convert a Direct3D 10 format into a GLI format.
    pub fn find_dx10(&self, d3d_fmt: D3DFormat, dxgi_fmt: DXGIFormat) -> crate::format::Format {
        let raw = unsafe { bindings::dx_find2(&self.inner, d3d_fmt.0, dxgi_fmt.into()) };
        crate::format::Format(raw)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct D3DFormat(pub(crate) gli::dx_d3dfmt);

impl D3DFormat {
    pub const UNKNOWN       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_UNKNOWN);
    pub const R8G8B8        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R8G8B8);
    pub const A8R8G8B8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8R8G8B8);
    pub const X8R8G8B8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_X8R8G8B8);
    pub const R5G6B5        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R5G6B5);
    pub const X1R5G5B5      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_X1R5G5B5);
    pub const A1R5G5B5      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A1R5G5B5);
    pub const A4R4G4B4      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A4R4G4B4);
    pub const R3G3B2        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R3G3B2);
    pub const A8            : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8);
    pub const A8R3G3B2      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8R3G3B2);
    pub const X4R4G4B4      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_X4R4G4B4);
    pub const A2B10G10R10   : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A2B10G10R10);
    pub const A8B8G8R8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8B8G8R8);
    pub const X8B8G8R8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_X8B8G8R8);
    pub const G16R16        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_G16R16);
    pub const A2R10G10B10   : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A2R10G10B10);
    pub const A16B16G16R16  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A16B16G16R16);
    pub const A8P8          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8P8);
    pub const P8            : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_P8);
    pub const L8            : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_L8);
    pub const A8L8          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A8L8);
    pub const A4L4          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A4L4);
    pub const V8U8          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_V8U8);
    pub const L6V5U5        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_L6V5U5);
    pub const X8L8V8U8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_X8L8V8U8);
    pub const Q8W8V8U8      : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_Q8W8V8U8);
    pub const V16U16        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_V16U16);
    pub const A2W10V10U10   : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A2W10V10U10);
    pub const UYVY          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_UYVY);
    pub const R8G8_B8G8     : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R8G8_B8G8);
    pub const YUY2          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_YUY2);
    pub const G8R8_G8B8     : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_G8R8_G8B8);
    pub const DXT1          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DXT1);
    pub const DXT2          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DXT2);
    pub const DXT3          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DXT3);
    pub const DXT4          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DXT4);
    pub const DXT5          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DXT5);
    pub const ATI1          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ATI1);
    pub const AT1N          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_AT1N);
    pub const ATI2          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ATI2);
    pub const AT2N          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_AT2N);
    pub const BC4U          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_BC4U);
    pub const BC4S          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_BC4S);
    pub const BC5U          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_BC5U);
    pub const BC5S          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_BC5S);
    pub const ETC           : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ETC);
    pub const ETC1          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ETC1);
    pub const ATC           : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ATC);
    pub const ATCA          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ATCA);
    pub const ATCI          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_ATCI);
    pub const POWERVR_2BPP  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_POWERVR_2BPP);
    pub const POWERVR_4BPP  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_POWERVR_4BPP);
    pub const D16_LOCKABLE  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D16_LOCKABLE);
    pub const D32           : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D32);
    pub const D15S1         : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D15S1);
    pub const D24S8         : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D24S8);
    pub const D24X8         : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D24X8);
    pub const D24X4S4       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D24X4S4);
    pub const D16           : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D16);
    pub const D32F_LOCKABLE : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D32F_LOCKABLE);
    pub const D24FS8        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_D24S8);
    pub const L16           : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_L16);
    pub const VERTEXDATA    : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_VERTEXDATA);
    pub const INDEX16       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_INDEX16);
    pub const INDEX32       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_INDEX32);
    pub const Q16W16V16U16  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_Q16W16V16U16);
    pub const MULTI2_ARGB8  : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_MULTI2_ARGB8);
    pub const R16F          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R16F);
    pub const G16R16F       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_G16R16F);
    pub const A16B16G16R16F : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A16B16G16R16F);
    pub const R32F          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_R32F);
    pub const G32R32F       : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_G32R32F);
    pub const A32B32G32R32F : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_A32B32G32R32F);
    pub const CXV8U8        : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_CxV8U8);
    pub const DX10          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_DX10);
    pub const GLI1          : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_GLI1);
    pub const FORCE_DWORD   : D3DFormat = D3DFormat(gli::dx_d3dfmt_D3DFMT_FORCE_DWORD);
}

#[derive(Clone, Copy)]
pub union DXGIFormat {
    dds: gli::dx_dxgi_format_dds,
    gli: gli::dx_dxgi_format_gli,
}

impl DXGIFormat {

    pub const DDS_UNKNOWN                    : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_UNKNOWN };
    pub const DDS_R32G32B32A32_TYPELESS      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32A32_TYPELESS };
    pub const DDS_R32G32B32A32_FLOAT         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32A32_FLOAT };
    pub const DDS_R32G32B32A32_UINT          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32A32_UINT };
    pub const DDS_R32G32B32A32_SINT          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32A32_SINT };
    pub const DDS_R32G32B32_TYPELESS         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32_TYPELESS };
    pub const DDS_R32G32B32_FLOAT            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32_FLOAT };
    pub const DDS_R32G32B32_UINT             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32_UINT };
    pub const DDS_R32G32B32_SINT             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32B32_SINT };
    pub const DDS_R16G16B16A16_TYPELESS      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_TYPELESS };
    pub const DDS_R16G16B16A16_FLOAT         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_FLOAT };
    pub const DDS_R16G16B16A16_UNORM         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_UNORM };
    pub const DDS_R16G16B16A16_UINT          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_UINT };
    pub const DDS_R16G16B16A16_SNORM         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_SNORM };
    pub const DDS_R16G16B16A16_SINT          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16B16A16_SINT };
    pub const DDS_R32G32_TYPELESS            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32_TYPELESS };
    pub const DDS_R32G32_FLOAT               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32_FLOAT };
    pub const DDS_R32G32_UINT                : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32_UINT };
    pub const DDS_R32G32_SINT                : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G32_SINT };
    pub const DDS_R32G8X24_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32G8X24_TYPELESS };
    pub const DDS_D32_FLOAT_S8X24_UINT       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_D32_FLOAT_S8X24_UINT };
    pub const DDS_R32_FLOAT_X8X24_TYPELESS   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS };
    pub const DDS_X32_TYPELESS_G8X24_UINT    : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_X32_TYPELESS_G8X24_UINT };
    pub const DDS_R10G10B10A2_TYPELESS       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R10G10B10A2_TYPELESS };
    pub const DDS_R10G10B10A2_UNORM          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R10G10B10A2_UNORM };
    pub const DDS_R10G10B10A2_UINT           : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R10G10B10A2_UINT };
    pub const DDS_R8G8B8A8_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_TYPELESS };
    pub const DDS_R8G8B8A8_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_UNORM };
    pub const DDS_R8G8B8A8_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_UNORM_SRGB };
    pub const DDS_R8G8B8A8_UINT              : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_UINT };
    pub const DDS_R8G8B8A8_SNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_SNORM };
    pub const DDS_R8G8B8A8_SINT              : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8B8A8_SINT };
    pub const DDS_R16G16_TYPELESS            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_TYPELESS };
    pub const DDS_R16G16_FLOAT               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_FLOAT };
    pub const DDS_R16G16_UNORM               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_UNORM };
    pub const DDS_R16G16_UINT                : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_UINT };
    pub const DDS_R16G16_SNORM               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_SNORM };
    pub const DDS_R16G16_SINT                : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16G16_SINT };
    pub const DDS_R32_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32_TYPELESS };
    pub const DDS_D32_FLOAT                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_D32_FLOAT };
    pub const DDS_R32_FLOAT                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32_FLOAT };
    pub const DDS_R32_UINT                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32_UINT };
    pub const DDS_R32_SINT                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R32_SINT };
    pub const DDS_R24G8_TYPELESS             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R24G8_TYPELESS };
    pub const DDS_D24_UNORM_S8_UINT          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_D24_UNORM_S8_UINT };
    pub const DDS_R24_UNORM_X8_TYPELESS      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R24_UNORM_X8_TYPELESS };
    pub const DDS_X24_TYPELESS_G8_UINT       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_X24_TYPELESS_G8_UINT };
    pub const DDS_R8G8_TYPELESS              : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_TYPELESS };
    pub const DDS_R8G8_UNORM                 : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_UNORM };
    pub const DDS_R8G8_UINT                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_UINT };
    pub const DDS_R8G8_SNORM                 : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_SNORM };
    pub const DDS_R8G8_SINT                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_SINT };
    pub const DDS_R16_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_TYPELESS };
    pub const DDS_R16_FLOAT                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_FLOAT };
    pub const DDS_D16_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_D16_UNORM };
    pub const DDS_R16_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_UNORM };
    pub const DDS_R16_UINT                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_UINT };
    pub const DDS_R16_SNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_SNORM };
    pub const DDS_R16_SINT                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R16_SINT };
    pub const DDS_R8_TYPELESS                : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8_TYPELESS };
    pub const DDS_R8_UNORM                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8_UNORM };
    pub const DDS_R8_UINT                    : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8_UINT };
    pub const DDS_R8_SNORM                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8_SNORM };
    pub const DDS_R8_SINT                    : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8_SINT };
    pub const DDS_A8_UNORM                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_A8_UNORM };
    pub const DDS_R1_UNORM                   : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R1_UNORM };
    pub const DDS_R9G9B9E5_SHAREDEXP         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R9G9B9E5_SHAREDEXP };
    pub const DDS_R8G8_B8G8_UNORM            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R8G8_B8G8_UNORM };
    pub const DDS_G8R8_G8B8_UNORM            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_G8R8_G8B8_UNORM };
    pub const DDS_BC1_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC1_TYPELESS };
    pub const DDS_BC1_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC1_UNORM };
    pub const DDS_BC1_UNORM_SRGB             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC1_UNORM_SRGB };
    pub const DDS_BC2_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC2_TYPELESS };
    pub const DDS_BC2_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC2_UNORM };
    pub const DDS_BC2_UNORM_SRGB             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC2_UNORM_SRGB };
    pub const DDS_BC3_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC3_TYPELESS };
    pub const DDS_BC3_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC3_UNORM };
    pub const DDS_BC3_UNORM_SRGB             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC3_UNORM_SRGB };
    pub const DDS_BC4_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC4_TYPELESS };
    pub const DDS_BC4_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC4_UNORM };
    pub const DDS_BC4_SNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC4_SNORM };
    pub const DDS_BC5_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC5_TYPELESS };
    pub const DDS_BC5_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC5_UNORM };
    pub const DDS_BC5_SNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC5_SNORM };
    pub const DDS_B5G6R5_UNORM               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B5G6R5_UNORM };
    pub const DDS_B5G5R5A1_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B5G5R5A1_UNORM };
    pub const DDS_B8G8R8A8_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8A8_UNORM };
    pub const DDS_B8G8R8X8_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8X8_UNORM };
    pub const DDS_R10G10B10_XR_BIAS_A2_UNORM : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM };
    pub const DDS_B8G8R8A8_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8A8_TYPELESS };
    pub const DDS_B8G8R8A8_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8A8_UNORM_SRGB };
    pub const DDS_B8G8R8X8_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8X8_TYPELESS };
    pub const DDS_B8G8R8X8_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B8G8R8X8_UNORM_SRGB };
    pub const DDS_BC6H_TYPELESS              : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC6H_TYPELESS };
    pub const DDS_BC6H_UF16                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC6H_UF16 };
    pub const DDS_BC6H_SF16                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC6H_SF16 };
    pub const DDS_BC7_TYPELESS               : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC7_TYPELESS };
    pub const DDS_BC7_UNORM                  : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC7_UNORM };
    pub const DDS_BC7_UNORM_SRGB             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_BC7_UNORM_SRGB };
    pub const DDS_AYUV                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_AYUV };
    pub const DDS_Y410                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_Y410 };
    pub const DDS_Y416                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_Y416 };
    pub const DDS_NV12                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_NV12 };
    pub const DDS_P010                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_P010 };
    pub const DDS_P016                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_P016 };
    pub const DDS_420_OPAQUE                 : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_420_OPAQUE };
    pub const DDS_YUY2                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_YUY2 };
    pub const DDS_Y210                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_Y210 };
    pub const DDS_Y216                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_Y216 };
    pub const DDS_NV11                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_NV11 };
    pub const DDS_AI44                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_AI44 };
    pub const DDS_IA44                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_IA44 };
    pub const DDS_P8                         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_P8 };
    pub const DDS_A8P8                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_A8P8 };
    pub const DDS_B4G4R4A4_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_B4G4R4A4_UNORM };
    pub const DDS_P208                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_P208 };
    pub const DDS_V208                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_V208 };
    pub const DDS_V408                       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_V408 };
    pub const DDS_ASTC_4X4_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_4X4_TYPELESS };
    pub const DDS_ASTC_4X4_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_4X4_UNORM };
    pub const DDS_ASTC_4X4_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_4X4_UNORM_SRGB };
    pub const DDS_ASTC_5X4_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X4_TYPELESS };
    pub const DDS_ASTC_5X4_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X4_UNORM };
    pub const DDS_ASTC_5X4_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X4_UNORM_SRGB };
    pub const DDS_ASTC_5X5_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X5_TYPELESS };
    pub const DDS_ASTC_5X5_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X5_UNORM };
    pub const DDS_ASTC_5X5_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_5X5_UNORM_SRGB };
    pub const DDS_ASTC_6X5_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X5_TYPELESS };
    pub const DDS_ASTC_6X5_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X5_UNORM };
    pub const DDS_ASTC_6X5_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X5_UNORM_SRGB };
    pub const DDS_ASTC_6X6_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X6_TYPELESS };
    pub const DDS_ASTC_6X6_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X6_UNORM };
    pub const DDS_ASTC_6X6_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_6X6_UNORM_SRGB };
    pub const DDS_ASTC_8X5_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X5_TYPELESS };
    pub const DDS_ASTC_8X5_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X5_UNORM };
    pub const DDS_ASTC_8X5_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X5_UNORM_SRGB };
    pub const DDS_ASTC_8X6_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X6_TYPELESS };
    pub const DDS_ASTC_8X6_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X6_UNORM };
    pub const DDS_ASTC_8X6_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X6_UNORM_SRGB };
    pub const DDS_ASTC_8X8_TYPELESS          : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X8_TYPELESS };
    pub const DDS_ASTC_8X8_UNORM             : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X8_UNORM };
    pub const DDS_ASTC_8X8_UNORM_SRGB        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_8X8_UNORM_SRGB };
    pub const DDS_ASTC_10X5_TYPELESS         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X5_TYPELESS };
    pub const DDS_ASTC_10X5_UNORM            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X5_UNORM };
    pub const DDS_ASTC_10X5_UNORM_SRGB       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X5_UNORM_SRGB };
    pub const DDS_ASTC_10X6_TYPELESS         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X6_TYPELESS };
    pub const DDS_ASTC_10X6_UNORM            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X6_UNORM };
    pub const DDS_ASTC_10X6_UNORM_SRGB       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X6_UNORM_SRGB };
    pub const DDS_ASTC_10X8_TYPELESS         : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X8_TYPELESS };
    pub const DDS_ASTC_10X8_UNORM            : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X8_UNORM };
    pub const DDS_ASTC_10X8_UNORM_SRGB       : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X8_UNORM_SRGB };
    pub const DDS_ASTC_10X10_TYPELESS        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X10_TYPELESS };
    pub const DDS_ASTC_10X10_UNORM_SRGB      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_10X10_UNORM_SRGB };
    pub const DDS_ASTC_12X10_TYPELESS        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X10_TYPELESS };
    pub const DDS_ASTC_12X10_UNORM           : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X10_UNORM };
    pub const DDS_ASTC_12X10_UNORM_SRGB      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X10_UNORM_SRGB };
    pub const DDS_ASTC_12X12_TYPELESS        : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X12_TYPELESS };
    pub const DDS_ASTC_12X12_UNORM           : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X12_UNORM };
    pub const DDS_ASTC_12X12_UNORM_SRGB      : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_ASTC_12X12_UNORM_SRGB };
    pub const DDS_FORCE_UINT                 : DXGIFormat = DXGIFormat { dds: gli::dx_dxgi_format_dds_DXGI_FORMAT_FORCE_UINT };


    pub const GLI_R64_UINT               : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64_UINT_GLI };
    pub const GLI_R64_SINT               : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64_SINT_GLI };
    pub const GLI_R64_FLOAT              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64_FLOAT_GLI };
    pub const GLI_R64G64_UINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64_UINT_GLI };
    pub const GLI_R64G64_SINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64_SINT_GLI };
    pub const GLI_R64G64_FLOAT           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64_FLOAT_GLI };
    pub const GLI_R64G64B64_UINT         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64_UINT_GLI };
    pub const GLI_R64G64B64_SINT         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64_SINT_GLI };
    pub const GLI_R64G64B64_FLOAT        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64_FLOAT_GLI };
    pub const GLI_R64G64B64A64_UINT      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64A64_UINT_GLI };
    pub const GLI_R64G64B64A64_SINT      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64A64_SINT_GLI };
    pub const GLI_R64G64B64A64_FLOAT     : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R64G64B64A64_FLOAT_GLI };
    pub const GLI_RG4_UNORM              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RG4_UNORM_GLI };
    pub const GLI_RGBA4_UNORM            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA4_UNORM_GLI };
    pub const GLI_R5G6B5_UNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R5G6B5_UNORM_GLI };
    pub const GLI_R5G5B5A1_UNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R5G5B5A1_UNORM_GLI };
    pub const GLI_A1B5G5R5_UNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_A1B5G5R5_UNORM_GLI };
    pub const GLI_R8_SRGB                : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8_SRGB_GLI };
    pub const GLI_R8_USCALED             : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8_USCALED_GLI };
    pub const GLI_R8_SSCALED             : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8_SSCALED_GLI };
    pub const GLI_R8G8_SRGB              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8_SRGB_GLI };
    pub const GLI_R8G8_USCALED           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8_USCALED_GLI };
    pub const GLI_R8G8_SSCALED           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8_SSCALED_GLI };
    pub const GLI_R8G8B8_UNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_UNORM_GLI };
    pub const GLI_R8G8B8_SNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_SNORM_GLI };
    pub const GLI_R8G8B8_USCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_USCALED_GLI };
    pub const GLI_R8G8B8_SSCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_SSCALED_GLI };
    pub const GLI_R8G8B8_UINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_UINT_GLI };
    pub const GLI_R8G8B8_SINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_SINT_GLI };
    pub const GLI_R8G8B8_SRGB            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8_SRGB_GLI };
    pub const GLI_B8G8R8_UNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_UNORM_GLI };
    pub const GLI_B8G8R8_SNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_SNORM_GLI };
    pub const GLI_B8G8R8_USCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_USCALED_GLI };
    pub const GLI_B8G8R8_SSCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_SSCALED_GLI };
    pub const GLI_B8G8R8_UINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_UINT_GLI };
    pub const GLI_B8G8R8_SINT            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_SINT_GLI };
    pub const GLI_B8G8R8_SRGB            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8_SRGB_GLI };
    pub const GLI_R8G8B8A8_USCALED       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_USCALED_GLI };
    pub const GLI_R8G8B8A8_SSCALED       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_SSCALED_GLI };
    pub const GLI_B8G8R8A8_SNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8A8_SNORM_GLI };
    pub const GLI_B8G8R8A8_USCALED       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8A8_USCALED_GLI };
    pub const GLI_B8G8R8A8_SSCALED       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8A8_SSCALED_GLI };
    pub const GLI_B8G8R8A8_UINT          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8A8_UINT_GLI };
    pub const GLI_B8G8R8A8_SINT          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B8G8R8A8_SINT_GLI };
    pub const GLI_R8G8B8A8_PACK_UNORM    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_UNORM_GLI };
    pub const GLI_R8G8B8A8_PACK_SNORM    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_SNORM_GLI };
    pub const GLI_R8G8B8A8_PACK_USCALED  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_USCALED_GLI };
    pub const GLI_R8G8B8A8_PACK_SSCALED  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_SSCALED_GLI };
    pub const GLI_R8G8B8A8_PACK_UINT     : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_UINT_GLI };
    pub const GLI_R8G8B8A8_PACK_SINT     : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_SINT_GLI };
    pub const GLI_R8G8B8A8_PACK_SRGB     : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R8G8B8A8_PACK_SRGB_GLI };
    pub const GLI_R10G10B10A2_SNORM      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R10G10B10A2_SNORM_GLI };
    pub const GLI_R10G10B10A2_USCALED    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R10G10B10A2_USCALED_GLI };
    pub const GLI_R10G10B10A2_SSCALED    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R10G10B10A2_SSCALED_GLI };
    pub const GLI_R10G10B10A2_SINT       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R10G10B10A2_SINT_GLI };
    pub const GLI_B10G10R10A2_UNORM      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_UNORM_GLI };
    pub const GLI_B10G10R10A2_SNORM      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_SNORM_GLI };
    pub const GLI_B10G10R10A2_USCALED    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_USCALED_GLI };
    pub const GLI_B10G10R10A2_SSCALED    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_SSCALED_GLI };
    pub const GLI_B10G10R10A2_UINT       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_UINT_GLI };
    pub const GLI_B10G10R10A2_SINT       : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_B10G10R10A2_SINT_GLI };
    pub const GLI_R16_USCALED            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16_USCALED_GLI };
    pub const GLI_R16_SSCALED            : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16_SSCALED_GLI };
    pub const GLI_R16G16_USCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16_USCALED_GLI };
    pub const GLI_R16G16_SSCALED         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16_SSCALED_GLI };
    pub const GLI_R16G16B16_UNORM        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_UNORM_GLI };
    pub const GLI_R16G16B16_SNORM        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_SNORM_GLI };
    pub const GLI_R16G16B16_USCALED      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_USCALED_GLI };
    pub const GLI_R16G16B16_SSCALED      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_SSCALED_GLI };
    pub const GLI_R16G16B16_UINT         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_UINT_GLI };
    pub const GLI_R16G16B16_SINT         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_SINT_GLI };
    pub const GLI_R16G16B16_FLOAT        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16_FLOAT_GLI };
    pub const GLI_R16G16B16A16_USCALED   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16A16_USCALED_GLI };
    pub const GLI_R16G16B16A16_SSCALED   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R16G16B16A16_SSCALED_GLI };
    pub const GLI_S8_UINT                : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_S8_UINT_GLI };
    pub const GLI_D16_UNORM_S8_UINT      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_D16_UNORM_S8_UINT_GLI };
    pub const GLI_D24_UNORM              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_D24_UNORM_GLI };
    pub const GLI_L8_UNORM               : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_L8_UNORM_GLI };
    pub const GLI_A8_UNORM               : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_A8_UNORM_GLI };
    pub const GLI_LA8_UNORM              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_LA8_UNORM_GLI };
    pub const GLI_L16_UNORM              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_L16_UNORM_GLI };
    pub const GLI_A16_UNORM              : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_A16_UNORM_GLI };
    pub const GLI_LA16_UNORM             : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_LA16_UNORM_GLI };
    pub const GLI_R3G3B2_UNORM           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R3G3B2_UNORM_GLI };
    pub const GLI_BC1_RGB_UNORM          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_BC1_RGB_UNORM_GLI };
    pub const GLI_BC1_RGB_SRGB           : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_BC1_RGB_SRGB_GLI };
    pub const GLI_RGB_ETC2_UNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_ETC2_UNORM_GLI };
    pub const GLI_RGB_ETC2_SRGB          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_ETC2_SRGB_GLI };
    pub const GLI_RGBA_ETC2_A1_UNORM     : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ETC2_A1_UNORM_GLI };
    pub const GLI_RGBA_ETC2_A1_SRGB      : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ETC2_A1_SRGB_GLI };
    pub const GLI_RGBA_ETC2_UNORM        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ETC2_UNORM_GLI };
    pub const GLI_RGBA_ETC2_SRGB         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ETC2_SRGB_GLI };
    pub const GLI_R11_EAC_UNORM          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R11_EAC_UNORM_GLI };
    pub const GLI_R11_EAC_SNORM          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_R11_EAC_SNORM_GLI };
    pub const GLI_RG11_EAC_UNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RG11_EAC_UNORM_GLI };
    pub const GLI_RG11_EAC_SNORM         : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RG11_EAC_SNORM_GLI };
    pub const GLI_RGB_PVRTC1_8X8_UNORM   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_PVRTC1_8X8_UNORM_GLI };
    pub const GLI_RGB_PVRTC1_8X8_SRGB    : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_PVRTC1_8X8_SRGB_GLI };
    pub const GLI_RGB_PVRTC1_16X8_UNORM  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_PVRTC1_16X8_UNORM_GLI };
    pub const GLI_RGB_PVRTC1_16X8_SRGB   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_PVRTC1_16X8_SRGB_GLI };
    pub const GLI_RGBA_PVRTC1_8X8_UNORM  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC1_8X8_UNORM_GLI };
    pub const GLI_RGBA_PVRTC1_8X8_SRGB   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC1_8X8_SRGB_GLI };
    pub const GLI_RGBA_PVRTC1_16X8_UNORM : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC1_16X8_UNORM_GLI };
    pub const GLI_RGBA_PVRTC1_16X8_SRGB  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC1_16X8_SRGB_GLI };
    pub const GLI_RGBA_PVRTC2_8X8_UNORM  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC2_8X8_UNORM_GLI };
    pub const GLI_RGBA_PVRTC2_8X8_SRGB   : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC2_8X8_SRGB_GLI };
    pub const GLI_RGBA_PVRTC2_16X8_UNORM : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC2_16X8_UNORM_GLI };
    pub const GLI_RGBA_PVRTC2_16X8_SRGB  : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_PVRTC2_16X8_SRGB_GLI };
    pub const GLI_RGB_ETC_UNORM          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_ETC_UNORM_GLI };
    pub const GLI_RGB_ATC_UNORM          : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGB_ATC_UNORM_GLI };
    pub const GLI_RGBA_ATCA_UNORM        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ATCA_UNORM_GLI };
    pub const GLI_RGBA_ATCI_UNORM        : DXGIFormat = DXGIFormat { gli: gli::dx_dxgi_format_gli_DXGI_FORMAT_RGBA_ATCI_UNORM_GLI };
}

impl From<gli::dx_dxgiFormat> for DXGIFormat {

    fn from(v: gli::dx_dxgiFormat) -> DXGIFormat {

        let raw_dxgi = unsafe { v.DDS.as_ref().clone() };
        DXGIFormat { dds: raw_dxgi }
    }
}

impl From<DXGIFormat> for gli::dx_dxgiFormat {

    fn from(v: DXGIFormat) -> gli::dx_dxgiFormat {

        let mut dxgi = gli::dx_dxgiFormat::default();

        unsafe {
            match v {
                | DXGIFormat { dds } => *dxgi.DDS.as_mut() = dds,
            };
        }

        dxgi
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)]
pub struct DDPF(pub(crate) gli::dx_ddpf);

impl DDPF {
    pub const ALPHAPIXELS           : DDPF = DDPF(gli::dx_ddpf_DDPF_ALPHAPIXELS);
    pub const ALPHA                 : DDPF = DDPF(gli::dx_ddpf_DDPF_ALPHA);
    pub const FOURCC                : DDPF = DDPF(gli::dx_ddpf_DDPF_FOURCC);
    pub const RGB                   : DDPF = DDPF(gli::dx_ddpf_DDPF_RGB);
    pub const YUV                   : DDPF = DDPF(gli::dx_ddpf_DDPF_YUV);
    pub const LUMINANCE             : DDPF = DDPF(gli::dx_ddpf_DDPF_LUMINANCE);
    pub const LUMINANCE_ALPHA       : DDPF = DDPF(gli::dx_ddpf_DDPF_LUMINANCE_ALPHA);
    pub const RGBAPIXELS            : DDPF = DDPF(gli::dx_ddpf_DDPF_RGBAPIXELS);
    pub const RGBA                  : DDPF = DDPF(gli::dx_ddpf_DDPF_RGBA);
    pub const LUMINANCE_ALPHAPIXELS : DDPF = DDPF(gli::dx_ddpf_DDPF_LUMINANCE_ALPHAPIXELS);
}




#[derive(Clone)]
pub struct DxFormat {
    dd_pixel_format: DDPF,
    d3d_format: D3DFormat,
    dxgi_format: DXGIFormat,
    mask: [u32; 4],
}

impl From<DxFormat> for gli::dx_format {

    fn from(v: DxFormat) -> gli::dx_format {

        gli::dx_format {
            DDPixelFormat: v.dd_pixel_format.0,
            D3DFormat: v.d3d_format.0,
            DXGIFormat: v.dxgi_format.into(),
            Mask: v.mask,
        }
    }
}

impl From<gli::dx_format> for DxFormat {

    fn from(v: gli::dx_format) -> DxFormat {

        DxFormat {
            dd_pixel_format: DDPF(v.DDPixelFormat),
            d3d_format: D3DFormat(v.D3DFormat),
            dxgi_format: v.DXGIFormat.into(),
            mask: v.Mask,
        }
    }
}
