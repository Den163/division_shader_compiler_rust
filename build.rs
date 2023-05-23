use std::env;
use std::path::Path;
use cmake::Config;

fn main() {
    let lib_name = "division_shader_compiler";
    let out_dir = env::var("OUT_DIR").unwrap();
    let install_dir = Path::new(&out_dir).join("lib");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=division_shader_compiler");

    println!("cargo:rustc-link-search=native={}", install_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib={}", "c++");

    let static_deps = [
        "division_shader_compiler",
        "MachineIndependent",
        "OSDependent",
        "OGLCompiler",
        "GenericCodeGen",
        "glslang-default-resource-limits",
        "glslang",
        "SPIRV",
        "spirv-cross-core",
        "spirv-cross-cpp",
        "spirv-cross-glsl",
        "spirv-cross-msl"
    ];

    for lib in static_deps {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    Config::new(lib_name)
        .target(lib_name)
        .out_dir(out_dir)
        .build();
}