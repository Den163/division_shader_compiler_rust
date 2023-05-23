use std::ffi::{c_char, c_ulong, c_void, CStr, CString};
use std::fs;
use std::ptr::null_mut;
use division_shader_compiler_rust::interface::*;
use walkdir::{WalkDir, DirEntry};

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
    let mut ctx: *mut DivisionShaderCompilerContext = null_mut();

    division_shader_compiler_alloc(&mut ctx);

    let glsl_to_msl_shaders = WalkDir::new("examples/shaders")
        .into_iter()
        .map(make_glsl_to_msl_shader)
        .filter_map(|o| o);

    for s in glsl_to_msl_shaders {
        let shader_src = CString::new(
            fs::read_to_string(s.glsl_path).expect("Can't read shader source from file")
        ).unwrap();
        let entry_point = CString::new(s.entry_point).unwrap();
        let mut spirv_bytes: c_ulong = 0;
        let mut msl_size: c_ulong = 0;

        assert!(division_shader_compiler_compile_glsl_to_spirv(
            ctx,
            shader_src.as_ptr(), shader_src.as_bytes().len() as i32,
            s.shader_type,
            entry_point.as_ptr(),
            &mut spirv_bytes,
        ));

        assert!(division_shader_compiler_compile_spirv_to_metal(
            ctx,
            (*ctx).spirv_buffer, spirv_bytes,
            s.shader_type,
            entry_point.as_ptr(),
            &mut msl_size,
        ));

        let msl_str = CStr::from_ptr((*ctx).output_src_buffer).to_str().unwrap();

        fs::write(s.out_msl_path, msl_str)
            .expect("Failed to write metal source to the file");

    }

    division_shader_compiler_free(ctx);
}

fn make_glsl_to_msl_shader(entry: Result<DirEntry, walkdir::Error>) -> Option<GlslToMslShader> {
    if entry.is_err() { return None; }

    let entry = entry.ok().unwrap();
    let extension = entry.path().extension();
    if extension.is_none() { return None; }

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
            entry.path().with_extension(
                format!("{}.metal", extension.to_str().unwrap())
            ).to_str().unwrap()
        ),
    });
}