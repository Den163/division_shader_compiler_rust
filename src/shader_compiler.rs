use std::{
    error::Error,
    ffi::{c_ulong, c_void, CStr, CString},
    fmt::Display,
    ptr::null_mut,
};

use super::interface::*;

pub use super::interface::ShaderType;

#[derive(Debug)]
pub struct CompileError;

impl Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Error for CompileError {}

pub struct ShaderCompiler {
    _ctx: *mut DivisionShaderCompilerContext,
}

impl ShaderCompiler {
    pub fn new() -> ShaderCompiler {
        unsafe {
            let mut _ctx: *mut DivisionShaderCompilerContext = null_mut();
            division_shader_compiler_alloc(&mut _ctx);

            ShaderCompiler { _ctx }
        }
    }

    pub fn compile_glsl_to_metal(
        &self,
        source: &str,
        msl_enty_point: &str,
        shader_type: ShaderType,
    ) -> Result<String, CompileError> {
        unsafe {
            let spirv_size = match self.glsl_to_spirv_save_to_ctx_get_size(
                source,
                msl_enty_point,
                shader_type,
            ) {
                Ok(v) => v,
                Err(_) => return Err(CompileError),
            };

            let c_entry_point = CString::new(msl_enty_point).unwrap_unchecked();
            let mut out_msl_size = 0 as c_ulong;

            if !division_shader_compiler_compile_spirv_to_metal(
                self._ctx,
                (*self._ctx).spirv_buffer,
                spirv_size,
                shader_type,
                c_entry_point.as_ptr(),
                &mut out_msl_size,
            ) {
                return Err(CompileError);
            }

            self.get_source_from_ctx()
        }
    }

    pub fn compile_glsl_to_spirv_source(
        &self,
        source: &str,
        output_spirv_entry_point: &str,
        shader_type: ShaderType,
    ) -> Result<Vec<u8>, CompileError> {
        unsafe {
            let out_spirv_byte_count = match self.glsl_to_spirv_save_to_ctx_get_size(
                source,
                output_spirv_entry_point,
                shader_type,
            ) {
                Ok(v) => v,
                Err(_) => return Err(CompileError),
            };

            let buff = (*self._ctx).spirv_buffer as *mut u8;
            let mut v = Vec::<u8>::with_capacity(out_spirv_byte_count as usize);
            buff.copy_to_nonoverlapping(v.as_mut_ptr(), v.capacity());

            Ok(v)
        }
    }

    unsafe fn glsl_to_spirv_save_to_ctx_get_size(
        &self,
        source: &str,
        output_spirv_entry_point: &str,
        shader_type: ShaderType,
    ) -> Result<u64, CompileError> {
        let c_source = CString::new(source);
        let c_entry_point = CString::new(output_spirv_entry_point);

        let c_source = match c_source {
            Ok(c) => c,
            Err(_) => return Err(CompileError),
        };
        let c_source = c_source.as_bytes();

        let c_entry_point = match c_entry_point {
            Ok(c) => c,
            Err(_) => return Err(CompileError),
        };

        let mut out_spirv_byte_count = 0 as c_ulong;
        if !division_shader_compiler_compile_glsl_to_spirv(
            self._ctx,
            c_source.as_ptr() as *const i8,
            c_source.len() as i32,
            shader_type,
            c_entry_point.as_ptr(),
            &mut out_spirv_byte_count,
        ) {
            return Err(CompileError);
        }

        Ok(out_spirv_byte_count)
    }

    pub fn compile_spirv_to_metal(
        &self,
        spirv_source: Vec<u8>,
        entry_point: &str,
        shader_type: ShaderType,
    ) -> Result<String, CompileError> {
        let c_entry_point = CString::new(entry_point);
        let c_entry_point = match c_entry_point {
            Ok(c) => c,
            Err(_) => return Err(CompileError),
        };

        let mut msl_size = 0 as c_ulong;
        unsafe {
            division_shader_compiler_compile_spirv_to_metal(
                self._ctx,
                spirv_source.as_ptr() as *const c_void,
                spirv_source.len() as c_ulong,
                shader_type,
                c_entry_point.as_ptr(),
                &mut msl_size,
            );

            self.get_source_from_ctx()
        }
    }

    unsafe fn get_source_from_ctx(&self) -> Result<String, CompileError> {
        let out_src = (*self._ctx).output_src_buffer;
        let out_src = CStr::from_ptr(out_src).to_str();

        match out_src {
            Ok(s) => Ok(s.to_string()),
            Err(_) => Err(CompileError),
        }
    }
}

impl Drop for ShaderCompiler {
    fn drop(&mut self) {
        unsafe {
            division_shader_compiler_free(self._ctx);
        }
    }
}
