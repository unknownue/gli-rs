
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
