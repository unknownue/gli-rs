
#[cfg(feature = "bindings")]
const OUTPUT_LOCATION: &'static str = "build/bindings.rs";

fn main() {
    build_gli_lib();
    generate_bindings();
}

fn build_gli_lib() {

    use std::env;

    let mut build = cc::Build::new();

    build
        .include("./vendors/gli/gli/core")
        .include("./vendors/gli/external")
        .include("./wrapper/bindings")
        .file("./wrapper/gli_lib.cpp");

    let target = env::var("TARGET")
        .expect("Target not found.");
    if target.contains("darwin") {
        build
            .flag("-std=c++11")
            .flag("-Wno-return-type-c-linkage")
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
            .flag("-Wno-return-type-c-linkage")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-return-type-c-linkage")
            .flag("-Wno-ignored-qualifiers")
            .flag("-Wno-type-limits")
            .cpp_link_stdlib("stdc++")
            .cpp(true);
    } else {
        build
            //.flag("-Wno-return-type-c-linkage")
            .cpp(true);
    }

    build.compile("libgli.a");
}

#[cfg(feature = "bindings")]
fn generate_bindings() {

    let mut builder = bindgen::Builder::default()
        .header("./wrapper/gli_lib.cpp")
        .clang_args(&[
            "-I./vendors/gli/external",
            "-I./vendors/gli/gli/core",
            "-I./wrapper/bindings/",
            "-std=c++11",
        ]);

    builder = builder
        .whitelist_function("bindings::.*")
        .whitelist_function("gli::is_.*");

    builder = builder
        .opaque_type("__darwin_.*")
        .opaque_type("glm::.*")
        .opaque_type("std::.*");

    builder = builder
        .enable_cxx_namespaces()
        .disable_untagged_union()
        .derive_debug(true)
        .derive_copy(false)
        .derive_default(true)
        .rustfmt_bindings(true)
        .trust_clang_mangling(false)
        .layout_tests(false);

    builder
        .generate().expect("Failed to generate bindings!")
        .write_to_file(::std::path::Path::new(OUTPUT_LOCATION))
        .expect("Failed to write bindings!");
}

#[cfg(not(feature = "bindings"))]
fn generate_bindings() {}
