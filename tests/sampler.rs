
#[cfg(test)]
mod sampler {

    extern crate gli_rs as gli;

    use std::path::Path;
    use self::gli::{GliTexture, Texture2D};
    use self::gli::sampler::{Wrap, Filter, FSampler2D};
    use self::gli::Extent2d;

    #[test]
    fn sampler2d_test() {

        // the size of this dds is 256x256.
        const TEST_DDS_PATH: &'static str = "./vendors/gli/data/kueken7_bgra8_unorm.dds";

        let texture_loaded: Texture2D = gli::load_dds(Path::new(TEST_DDS_PATH))
            .unwrap();

        assert_eq!(texture_loaded.levels(), 1);

        let mut test_sampler = FSampler2D::new(&texture_loaded, Wrap::CLAMP_TO_EDGE, Filter::LINEAR, Filter::LINEAR);

        let test_fetch = test_sampler.texel_fetch(Extent2d { width: 0, height: 0 }, 0);
        assert_ne!(test_fetch, [0.0; 4]);
        let test_sample = test_sampler.texel_lod([0.0, 0.0].into(), 0);

        println!("Test fetch  texel: {:?}", test_fetch);
        println!("Test sample texel: {:?}", test_sample);

        // test base level.
        test_sampler.texel_write(Extent2d { width: 255, height: 255 }, 0, [0.0; 4].into());
        assert_eq!(test_sampler.texel_fetch(Extent2d { width: 255, height: 255 }, 0), [0.0; 4]);

        // TODO: Test for other level generated.

        // test level 1.
        //test_sampler.generate_mipmaps(Filter::NEAREST);
        //test_sampler.texel_write(Extent2d { width: 127, height: 127 }, 0, [0.0; 4].into());
        //assert_eq!(test_sampler.texel_fetch(Extent2d { width: 127, height: 127 }, 0), [0.0; 4]);
    }


//    #[cfg(feature = "rc_debug")]
//    #[test]
//    fn sampler_rc_test() {
//
//    }
}
