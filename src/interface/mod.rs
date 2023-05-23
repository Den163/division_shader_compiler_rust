use std::ffi::{c_char, c_int, c_void, c_ulong};

#[repr(C)]
pub struct DivisionShaderCompilerContext {
    pub spirv_buffer: *mut c_void,
    pub output_src_buffer: *mut c_char,
    pub spirv_buffer_size: c_ulong,
    pub src_buffer_size: c_ulong,  
}

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum ShaderType {
    Vertex = 1,
    Fragment = 2
}

extern "C" {
    pub fn division_shader_compiler_alloc(ctx: *mut *mut DivisionShaderCompilerContext) -> bool;
    pub fn division_shader_compiler_free(ctx: *mut DivisionShaderCompilerContext);

    pub fn division_shader_compiler_compile_glsl_to_spirv(
        ctx: *mut DivisionShaderCompilerContext,
        source: *const c_char,
        source_size: i32,
        shader_type: ShaderType,
        spirv_entry_point_name: *const c_char,
        out_spirv_byte_count: *mut c_ulong
    ) -> bool;

    pub fn division_shader_compiler_compile_spirv_to_metal(
        ctx: *mut DivisionShaderCompilerContext,
        spirv_bytes: *const c_void,
        spirv_byte_count: c_ulong,
        shader_type: ShaderType,
        entry_point: *const c_char,
        out_metal_size: *mut c_ulong
    ) -> bool;
}