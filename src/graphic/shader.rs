#![allow(dead_code)]
extern crate gl;

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::ptr;

pub enum ShaderError {
    FailedOpeningFile,
    FailedReadingFile,
    FailedCompilingShader,
}

pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

pub struct Shader {
    pub id: u32,
}

impl Shader {
    pub fn new(path: &str, shader_type: ShaderType) -> Result<Shader, ShaderError> {
        let mut shader_file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err(ShaderError::FailedOpeningFile),
        };

        let mut shader_code = String::new();
        match shader_file.read_to_string(&mut shader_code) {
            Ok(number) => number,
            Err(_) => return Err(ShaderError::FailedOpeningFile),
        };

        let shader = CString::new(shader_code.as_bytes()).unwrap();

        let id: u32 = unsafe {
            let id = match shader_type {
                ShaderType::VertexShader => gl::CreateShader(gl::VERTEX_SHADER),
                ShaderType::FragmentShader => gl::CreateShader(gl::FRAGMENT_SHADER),
            };

            gl::ShaderSource(id, 1, &shader.as_ptr(), ptr::null());

            id
        };

        let mut success = unsafe {
            let mut success = 0;
            gl::CompileShader(id);
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            success
        };

        if success == 0 {
            return Err(ShaderError::FailedCompilingShader);
        }

        Ok(Shader { id: id })
    }

    pub fn delete(&self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}