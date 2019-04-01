
pub use self::sampler1d::FSampler1D;

mod sampler1d;

use crate::ffi::root::gli;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Wrap(pub(crate) gli::wrap);

impl Wrap {
    pub const WRAP_CLAMP_TO_EDGE          : Wrap = Wrap(gli::wrap_WRAP_CLAMP_TO_EDGE         );
    pub const WRAP_FIRST                  : Wrap = Wrap(gli::wrap_WRAP_FIRST                 );
    pub const WRAP_CLAMP_TO_BORDER        : Wrap = Wrap(gli::wrap_WRAP_CLAMP_TO_BORDER       );
    pub const WRAP_REPEAT                 : Wrap = Wrap(gli::wrap_WRAP_REPEAT                );
    pub const WRAP_MIRROR_REPEAT          : Wrap = Wrap(gli::wrap_WRAP_MIRROR_REPEAT         );
    pub const WRAP_MIRROR_CLAMP_TO_EDGE   : Wrap = Wrap(gli::wrap_WRAP_MIRROR_CLAMP_TO_EDGE  );
    pub const WRAP_MIRROR_CLAMP_TO_BORDER : Wrap = Wrap(gli::wrap_WRAP_MIRROR_CLAMP_TO_BORDER);
    pub const WRAP_LAST                   : Wrap = Wrap(gli::wrap_WRAP_LAST                  );
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Filter(pub(crate) gli::filter);

impl Filter {
    pub const WRAP_CLAMP_TO_EDGE          : Filter = Filter(gli::wrap_WRAP_CLAMP_TO_EDGE         );
    pub const WRAP_FIRST                  : Filter = Filter(gli::wrap_WRAP_FIRST                 );
    pub const WRAP_CLAMP_TO_BORDER        : Filter = Filter(gli::wrap_WRAP_CLAMP_TO_BORDER       );
    pub const WRAP_REPEAT                 : Filter = Filter(gli::wrap_WRAP_REPEAT                );
    pub const WRAP_MIRROR_REPEAT          : Filter = Filter(gli::wrap_WRAP_MIRROR_REPEAT         );
    pub const WRAP_MIRROR_CLAMP_TO_EDGE   : Filter = Filter(gli::wrap_WRAP_MIRROR_CLAMP_TO_EDGE  );
    pub const WRAP_MIRROR_CLAMP_TO_BORDER : Filter = Filter(gli::wrap_WRAP_MIRROR_CLAMP_TO_BORDER);
    pub const WRAP_LAST                   : Filter = Filter(gli::wrap_WRAP_LAST                  );
}
