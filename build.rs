
const OUTPUT_FILE: &'static str = "gen/binding.rs";

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

    build.compile("gli_cpp");

    generate_bindings();
}


fn generate_bindings() {

    let mut bindings = bindgen::Builder::default()
        .header("vendors/gli/gli/gli.hpp")
        .clang_arg("-I./wrapper")
        .clang_arg("-I./vendors/gli/external")
        .clang_arg("-std=c++11")
        .rustfmt_bindings(true)
        .blacklist_type("__darwin_.*")
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

    bindings_generated.write_to_file(Path::new(OUTPUT_FILE))
        .expect("Failed to write bindings!");
}
