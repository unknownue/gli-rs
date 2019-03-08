
#[cfg(feature = "bindings")]
const OUTPUT_LOCATION: &'static str = "build/bindings.rs";

fn main() {
    build_gli_lib();
    generate_bindings();
}

fn build_gli_lib() {

    use std::env;

    let mut build = cc::Build::new();

    build.include("./vendors/gli/gli")
        .include("./vendors/gli/external")
        .file("./wrapper/gli_lib.cpp");

    let target = env::var("TARGET")
        .expect("Target not found.");
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
}

#[cfg(feature = "bindings")]
fn generate_bindings() {

    bindgen::Builder::default()
        .header("./wrapper/gli_lib.cpp")
        .clang_args(&[
            "-I./vendors/gli/external",
            "-I./vendors/gli/gli",
            "-std=c++11",
        ])
        .whitelist_type("gli::texture.*")
        .whitelist_type("gli::image.*")
        .whitelist_function("gli::is_.*")
        .whitelist_function("gli::load.*")
        .whitelist_function("gli::save.*")
        .whitelist_function("gli::destroy_.*")
        .opaque_type("__darwin_.*")
        .opaque_type("glm::.*")
        .opaque_type("std::.*")
        .enable_cxx_namespaces()
        .disable_untagged_union()
        .derive_debug(true)
        .derive_copy(false)
        .rustfmt_bindings(true)
        .trust_clang_mangling(true)
        .layout_tests(false)
        .generate().expect("Failed to generate bindings!")
        .write_to_file(::std::path::Path::new(OUTPUT_LOCATION))
        .expect("Failed to write bindings!");
}

#[cfg(not(feature = "bindings"))]
fn generate_bindings() {}
