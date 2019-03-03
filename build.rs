
#[cfg(feature = "bindings_generation")]
const OUTPUT_LOCARTION: &'static str = "gen/bindings.rs";
#[cfg(feature = "bindings_generation")]
use std::path::Path;

use std::env;

fn main() {

    let mut build = cc::Build::new();

    build.include("vendors/gli/gli");
    build.include("vendors/gli/external");
    build.include("wrapper");

    let source_files = ["wrapper/gli_lib.cpp"];
    build.files(&source_files);

    let target = env::var("TARGET").unwrap();
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .flag("-Wno-ignored-qualifiers")
            .cpp_link_stdlib("c++")
            .cpp_set_stdlib("c++")
            .cpp(true);
    } else if target.contains("linux") {
        build
            .flag("-std=c++11")
            .flag("-Wno-unused-variable")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-unused-private-field")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    }

    build.compile("libgli.a");

    generate_bindings();
}


#[cfg(feature = "bindings_generation")]
fn generate_bindings() {

    let mut bindings = bindgen::Builder::default()
        .header("vendors/gli/gli/gli.hpp")
        .clang_args(&[
            "-I./wrapper",
            "-I./vendors/gli/external",
            "-std=c++11",
        ])
        .whitelist_type(".*texture.*")
        .whitelist_function("gli::is_.*")
        .opaque_type("__darwin_.*")
        .opaque_type("std::.*")
        .derive_debug(true)
        .rustfmt_bindings(true)
        .trust_clang_mangling(false)
        .layout_tests(false);

    if cfg!(target_os = "macos") {
        // To disable the fixup bindgen applies which adds search
        // paths from clang command line in order to avoid potential
        // conflict with -stdlib=libc++.
        bindings = bindings
            .clang_arg("-stdlib=libc++")
            .clang_arg("--target=x86_64-apple-darwin");
    }

    let bindings_generated = bindings.generate()
        .expect("Failed to generate bindings!");

    bindings_generated.write_to_file(Path::new(OUTPUT_LOCARTION))
        .expect("Failed to write bindings!");
}

#[cfg(not(feature = "bindings_generation"))]
fn generate_bindings() {}
