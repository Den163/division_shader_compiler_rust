use division_shader_compiler_rust::{ShaderCompiler, ShaderType};
use std::fs;
use walkdir::{DirEntry, WalkDir};

struct GlslToMslShader {
    glsl_path: String,
    entry_point: String,
    shader_type: ShaderType,
    out_msl_path: String,
}

fn main() {
    unsafe {
        compile_glsl_to_metal();
    }
}

unsafe fn compile_glsl_to_metal() {
    let compiler = ShaderCompiler::new();

    let glsl_to_msl_shaders = WalkDir::new("examples/shaders")
        .into_iter()
        .map(make_glsl_to_msl_shader)
        .filter_map(|o| o);

    for s in glsl_to_msl_shaders {
        let shader_src =
            fs::read_to_string(s.glsl_path).expect("Can't read shader source from file");

        let msl_src = compiler
            .compile_glsl_to_metal(&shader_src.as_str(), &s.entry_point, s.shader_type)
            .expect("Failed to compile shader");

        fs::write(s.out_msl_path, msl_src).expect("Failed to write metal source to the file");
    }
}

fn make_glsl_to_msl_shader(entry: Result<DirEntry, walkdir::Error>) -> Option<GlslToMslShader> {
    if entry.is_err() {
        return None;
    }

    let entry = entry.ok().unwrap();
    let extension = entry.path().extension();
    if extension.is_none() {
        return None;
    }

    let extension = extension.unwrap();

    let shader_type: ShaderType;
    let entry_point: String;
    if extension == "vert" {
        shader_type = ShaderType::Vertex;
        entry_point = "vert".to_string();
    } else if extension == "frag" {
        shader_type = ShaderType::Fragment;
        entry_point = "frag".to_string();
    } else {
        return None;
    }

    return Some(GlslToMslShader {
        shader_type,
        entry_point,
        glsl_path: String::from(entry.path().to_str().unwrap()),
        out_msl_path: String::from(
            entry
                .path()
                .with_extension(format!("{}.metal", extension.to_str().unwrap()))
                .to_str()
                .unwrap(),
        ),
    });
}
