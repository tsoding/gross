use gl;
use std;
use result::Result;

pub struct Program {
    pub id: u32
}

impl Program {
    pub fn from_shaders(shaders: Vec<Shader>) -> Result<Program> {
        Ok(Program { id: 0 })
    }

    pub fn use_program(&self) {
    }
}

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn from_str(shader_type: u32, source_code: &str) -> Result<Shader> {
        let c_source_code = std::ffi::CString::new(source_code)?;
        let p = c_source_code.as_ptr() as *const i8;

        let mut id = 0;
        unsafe {
            id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &p, std::ptr::null());
            gl::CompileShader(id);

            let mut params: i32 = -1;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut params as *mut i32);
            if gl::TRUE as i32 != params {
                let mut max_length: i32 = 0;
                gl::GetShaderiv(id,
                                gl::INFO_LOG_LENGTH,
                                &mut max_length as *mut i32);

                let mut error_log: Vec<u8> = vec![0; max_length as usize];

                gl::GetShaderInfoLog(id,
                                     max_length,
                                     &mut max_length as *mut i32,
                                     error_log.as_mut_ptr() as *mut i8);

                Err(std::str::from_utf8(&error_log)?.into())
            } else {
                Ok(Shader { id: id })
            }
        }
    }
}

pub struct VertexBuffer {
    pub id: u32,
    pub components_size: u32
}

impl VertexBuffer {
    pub fn new(components_size: u32) -> Result<VertexBuffer> {
        unimplemented!()
    }

    pub fn buffer_data(&self, data: Vec<f32>) {

    }
}

pub struct VertexArray {
    pub id: u32
}

impl VertexArray {
    pub fn new() -> Result<VertexArray> {
        unimplemented!()
    }

    pub fn vertex_attrib_array(&self, buffer: &VertexBuffer) {
        unimplemented!()
    }

    pub fn bind(&self) {
    }
}
