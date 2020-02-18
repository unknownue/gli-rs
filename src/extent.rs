
// Extent definition.

#[derive(Debug, Clone, Copy, Default)]
pub struct Extent1d {
    pub width: u32,
}

impl From<Extent1d> for u32 {

    fn from(v: Extent1d) -> u32 {
        v.width
    }
}

impl From<u32> for Extent1d {

    fn from(v: u32) -> Extent1d {
        Extent1d { width: v }
    }
}

impl From<[u32; 2]> for Extent1d {

    fn from(v: [u32; 2]) -> Extent1d {
        Extent1d { width: v[0] }
    }
}

impl From<[u32; 3]> for Extent1d {

    fn from(v: [u32; 3]) -> Extent1d {
        Extent1d { width: v[0] }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Extent2d {
    pub width : u32,
    pub height: u32,
}

impl From<Extent2d> for [u32; 2] {

    fn from(v: Extent2d) -> [u32; 2] {
        [v.width, v.height]
    }
}

impl From<[u32; 2]> for Extent2d {

    fn from(v: [u32; 2]) -> Extent2d {
        Extent2d { width : v[0], height: v[1] }
    }
}

impl From<[u32; 3]> for Extent2d {

    fn from(v: [u32; 3]) -> Extent2d {
        Extent2d { width : v[0], height: v[1] }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Extent3d {
    pub width : u32,
    pub height: u32,
    pub depth : u32,
}

impl From<Extent3d> for [u32; 3] {

    fn from(v: Extent3d) -> [u32; 3] {
        [v.width, v.height, v.depth]
    }
}

impl From<[u32; 3]> for Extent3d {

    fn from(v: [u32; 3]) -> Extent3d {
        Extent3d { width: v[0], height: v[1], depth: v[2] }
    }
}


// Normalize Coordinate definition.

#[derive(Debug, Clone, Copy, Default)]
pub struct NormalizeCoord1d {
    pub x: f32,
}

impl From<NormalizeCoord1d> for f32 {

    fn from(v: NormalizeCoord1d) -> f32 {
        v.x
    }
}

impl From<f32> for NormalizeCoord1d {

    fn from(v: f32) -> NormalizeCoord1d {
        NormalizeCoord1d { x: v }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct NormalizeCoord2d {
    pub x: f32,
    pub y: f32,
}

impl From<NormalizeCoord2d> for [f32; 2] {

    fn from(v: NormalizeCoord2d) -> [f32; 2] {
        [v.x, v.y]
    }
}

impl From<[f32; 2]> for NormalizeCoord2d {

    fn from(v: [f32; 2]) -> NormalizeCoord2d {
        NormalizeCoord2d { x: v[0], y: v[1] }
    }
}


#[derive(Debug, Clone, Copy, Default)]
pub struct NormalizeCoord3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<NormalizeCoord3d> for [f32; 3] {

    fn from(v: NormalizeCoord3d) -> [f32; 3] {
        [v.x, v.y, v.z]
    }
}

impl From<[f32; 3]> for NormalizeCoord3d {

    fn from(v: [f32; 3]) -> NormalizeCoord3d {
        NormalizeCoord3d { x: v[0], y: v[1], z: v[2] }
    }
}


impl From<[f32; 4]> for crate::ffi::root::bindings::TexelType4F {

    fn from(v: [f32; 4]) -> crate::ffi::root::bindings::TexelType4F {
        crate::ffi::root::bindings::TexelType4F {
            content: v,
        }
    }
}
