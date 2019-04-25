
#[cfg(test)]
mod texture {

    extern crate gli_rs as gli;

    use std::path::Path;
    use self::gli::{Texture2D, GliTexture};

    fn print_texture_info(texture: &impl GliTexture) {

        println!("\tFaces  count: {}", texture.faces());
        println!("\tLayers count: {}", texture.layers());
        println!("\tLevels count: {}", texture.levels());
        println!("\tSize: {}", texture.size());
        println!("\tFormat: {}", texture.format());
        println!("\tTarget: {}", texture.target());
    }

    #[test]
    fn load_and_save_dds() {

        const TEST__DDS_PATH: &'static str = "./vendors/gli/data/array_r8_uint.dds";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.dds"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_dds(Path::new(TEST__DDS_PATH))
            .unwrap();

        assert!(!texture_loaded.empty(), "DDS texture is empty.");

        let extent = texture_loaded.extent(0);

        println!("DDS Texture info:");
        println!("\tExtent: ({}, {})", extent.width, extent.height);
        print_texture_info(&texture_loaded);

        gli::save_dds(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }

    #[test]
    fn load_and_save_ktx() {

        const TEST__KTX_PATH: &'static str = "./vendors/gli/data/array_r8_uint.ktx";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.ktx"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_ktx(Path::new(TEST__KTX_PATH))
            .unwrap();

        assert!(!texture_loaded.empty(), "KTX texture is empty.");

        let extent = texture_loaded.extent(0);

        println!("KTX Texture info:");
        println!("\tExtent: ({}, {})", extent.width, extent.height);
        print_texture_info(&texture_loaded);

        gli::save_ktx(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }

    #[cfg(feature = "rc_debug")]
    #[test]
    fn shared_ptr_test() {

        use crate::texture::gli::texture::inner::TextureAccessible;
        use self::gli::ffi::root::bindings;

        const TEST_KTX_PATH: &'static str = "./vendors/gli/data/array_r8_uint.ktx";

        let mut texture_loaded: Texture2D = gli::load_ktx(Path::new(TEST_KTX_PATH))
            .unwrap();

        unsafe {
            let raw_texture = texture_loaded.raw_texture_mut();
            raw_texture.is_print_shared_storage_count = true;
            assert_eq!(bindings::Texture::get_texture_shared_storage_count(raw_texture), 1);
        }

        unsafe {
            let base_level_image = texture_loaded.get_level(0);
            let raw_texture = texture_loaded.raw_texture_mut();
            assert_eq!(bindings::Texture::get_texture_shared_storage_count(raw_texture), 2);
            assert_eq!(bindings::Image::get_image_shared_storage_count(&base_level_image.ffi), 2);
        }

        unsafe {
            let raw_texture = texture_loaded.raw_texture();
            assert_eq!(bindings::Texture::get_texture_shared_storage_count(raw_texture), 1);
        }
    }
}
