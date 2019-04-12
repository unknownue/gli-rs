
use gli_rs as gli;
use gli::{Texture2D, GliTexture, GliImage};

use std::path::Path;


fn main() {

    let texture: Texture2D = gli::load_ktx(Path::new("examples/kueken7_r5g6b5_unorm.ktx"))
        .expect("Failed to load ktx texture.");

    println!("KTX Texture info:");
    println!("\tExtent: ({}, {})", texture.extent(0).width, texture.extent(0).height);
    println!("\tFaces  count: {}", texture.faces());
    println!("\tLayers count: {}", texture.layers());
    println!("\tLevels count: {}", texture.levels());
    println!("\tSize: {}", texture.size());
    println!("\tAddress: {:?}", texture.data());
    println!("\tFormat: {}", texture.format());
    println!("\tTarget: {}", texture.target());
    println!();

    { // access the base level image of the texture.
        let image: GliImage = texture.get_level(0);
        println!("Base level image info:");
        println!("\tExtent: ({}, {})", image.extent().width, image.extent().height);
        println!("\tFormat: {}", image.format());
        println!("\tSize: {}", image.size());
        println!("\tAddress: {:?}", image.data());
        println!();
    }

    { // access the level 1 image of the texture.
        let image: GliImage = texture.get_level(1);
        println!("Level 1 image info:");
        println!("\tExtent: ({}, {})", image.extent().width, image.extent().height);
        println!("\tFormat: {}", image.format());
        println!("\tSize: {}", image.size());
        println!("\tAddress: {:?}", image.data());
        println!();
    }

    gli::save_dds(&texture, Path::new("kueken7_r5g6b5_unorm.dds"))
        .expect("Failed to save dds texture.");
    println!("Texture was saved to kueken7_r5g6b5_unorm.dds.");
}
