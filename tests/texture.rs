
#[cfg(test)]
mod texture {

    extern crate gli_rs as gli;

    use std::path::Path;
    use gli::texture::Texture2D;
    use gli::texture::GliTexture;

    #[test]
    fn load_and_save_dds() {

        const TEST__DDS_PATH: &'static str = "./vendors/gli/data/array_r8_uint.dds";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.dds"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_dds(Path::new(TEST__DDS_PATH))
            .unwrap();

        if texture_loaded.empty() {
            assert!(true, "DDS texture is empty.");
        } else {
            println!("DDS Texture info:");

            let extent = texture_loaded.extent(0);
            println!("\tExtent: ({}, {})", extent[0], extent[1]);
            println!("\tFaces  count: {}", texture_loaded.faces());
            println!("\tLayers count: {}", texture_loaded.layers());
            println!("\tLevels count: {}", texture_loaded.levels());
            println!("\tSize: {}", texture_loaded.size());
            println!("\tFormat: {:?}", texture_loaded.format());
        }

        gli::save_dds(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }

    #[test]
    fn load_and_save_ktx() {

        const TEST__KTX_PATH: &'static str = "./vendors/gli/data/array_r8_uint.ktx";
        const FILE_SAVE_PATH: &'static str = "./array_r8_uint.ktx"; // save to project directory.

        let texture_loaded: Texture2D = gli::load_ktx(Path::new(TEST__KTX_PATH))
            .unwrap();

        if texture_loaded.empty() {
            assert!(true, "KTX texture is empty.");
        } else {
            println!("KTX Texture info:");

            let extent = texture_loaded.extent(0);
            println!("\tExtent: ({}, {})", extent[0], extent[1]);
            println!("\tFaces  count: {}", texture_loaded.faces());
            println!("\tLayers count: {}", texture_loaded.layers());
            println!("\tLevels count: {}", texture_loaded.levels());
            println!("\tSize: {}", texture_loaded.size());
            println!("\tFormat: {:?}", texture_loaded.format());
        }

        gli::save_ktx(&texture_loaded, Path::new(FILE_SAVE_PATH))
            .unwrap();
    }
}
