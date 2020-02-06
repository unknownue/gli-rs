
#[cfg(test)]
mod sampler {

    extern crate gli_rs as gli;

    use std::path::Path;
    use self::gli::{GliTexture, Texture2D};
    use self::gli::sampler::{Wrap, Filter, FSampler2D};
    use self::gli::Extent2d;

    #[cfg(not(target_os = "windows"))]
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
        let test_sample = test_sampler.texel_lod([0.0, 0.0].into(), 0.0);

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

    // Run this test by 'cargo test --features rc_debug -- --nocapture sampler2d_memory_test'
    #[cfg(feature = "rc_debug")]
    #[test]
    fn sampler2d_memory_test() {

        use crate::sampler::gli::inner::TextureAccessible;
        use self::gli::ffi::root::bindings;

        const TEST_DDS_PATH: &'static str = "./vendors/gli/data/kueken7_bgra8_unorm.dds";
        let mut texture_loaded: Texture2D = gli::load_dds(Path::new(TEST_DDS_PATH))
            .unwrap();

        for i in 0..10 {
            let sampler = FSampler2D::new(&texture_loaded, Wrap::CLAMP_TO_EDGE, Filter::LINEAR, Filter::LINEAR);

//            unsafe {
//                let raw_texture = texture_loaded.raw_texture_mut();
//                raw_texture.is_print_shared_storage_count = true;
//                //println!("{} round - point count {}", i, bindings::Texture::get_texture_shared_storage_count(raw_texture));
//                assert_eq!(bindings::Texture::get_texture_shared_storage_count(raw_texture), 2);
//            }
            let _ = sampler.texel_fetch([0, 0].into(), 0);
        }

        unsafe {
            let raw_texture = texture_loaded.raw_texture();
            assert_eq!(bindings::Texture::get_texture_shared_storage_count(raw_texture), 1);
        }
    }


//    #[cfg(feature = "rc_debug")]
//    #[test]
//    fn sampler_rc_test() {
//
//    }
}
