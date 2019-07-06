extern crate gl;
extern crate glfw;

extern crate cgmath;

mod graphic;

use graphic::canvas::Canvas;

use graphic::shader::Shader;
use graphic::shader::ShaderType;
use graphic::shader::ShaderError;

use graphic::program::Program;
use graphic::program::ProgramError;

use graphic::vertex_array::VertexArray;

use graphic::data_buffer::DataBuffer;
use graphic::data_buffer::buffer_element::BufferDataType;
use graphic::data_buffer::buffer_element::BufferElement;

use graphic::index_buffer::IndexBuffer;

const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() {
    let mut canvas = Canvas::new(TITLE, WIDTH, HEIGHT).expect("Window failed");

    let _vertex_shader = match Shader::new(
        "src/assets/vertex.shader.glsl",
        ShaderType::VertexShader,
    ) {
        Ok(v) => v,
        Err(ShaderError::FailedOpeningFile) => {
            println!("Failed opening file, file may not exist or the path is wrong");
            return;
        }
        Err(ShaderError::FailedReadingFile) => {
            println!("File was not readable, check file content and permissions");
            return;
        }
        Err(ShaderError::FailedCompilingShader(error)) => {
            println!("Compiling vertex shader failed, check shader content\n{}", error);
            return;
        }
    };

    let _fragment_shader = match Shader::new(
        "src/assets/fragment.shader.glsl",
        ShaderType::FragmentShader,
    ) {
        Ok(v) => v,
        Err(ShaderError::FailedOpeningFile) => {
            println!("Failed opening file, file may not exist or the path is wrong");
            return;
        }
        Err(ShaderError::FailedReadingFile) => {
            println!("File was not readable, check file content and permissions");
            return;
        }
        Err(ShaderError::FailedCompilingShader(error)) => {
            println!("Compiling fragment shader failed, check shader content:\n{}", error);
            return;
        }
    };

    let program = match Program::new(_vertex_shader, _fragment_shader) {
        Ok(program) => program,
        Err(ProgramError::FailedLinkingShader(error)) => {
            println!("Linking program failed: \n{}", error);
            return;
        }
    };

    let _vertices_triangle: [cgmath::Vector3<f32>; 3] = [
        cgmath::Vector3::new(0.0, 1.0, 0.0),
        cgmath::Vector3::new(1.0, -1.0, 0.0),
        cgmath::Vector3::new(-1.0, -1.0, 0.0),
    ];

    let _color_triangle: [cgmath::Vector4<f32>; 3] = [
        cgmath::Vector4::new(1.0, 0.0, 0.0, 1.0),
        cgmath::Vector4::new(0.0, 1.0, 0.0, 1.0),
        cgmath::Vector4::new(0.0, 0.0, 1.0, 1.0),
    ];

    let _index_triangle = [
        cgmath::Vector3::new(0, 1, 2)
    ];

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let mut _data_buffer = DataBuffer::new(_vertices_triangle.as_ptr(), _vertices_triangle.len() * std::mem::size_of::<cgmath::Vector3<f32>>());
    let _data_element = BufferElement::new(BufferDataType::Float3, "aPos", false);

    let mut _data_buffer_color = DataBuffer::new(_color_triangle.as_ptr(), _color_triangle.len() * std::mem::size_of::<cgmath::Vector4<f32>>());
    let _data_element_color = BufferElement::new(BufferDataType::Float4, "aCol", false);

    _data_buffer.add_element(_data_element);
    _data_buffer.configure_by_name(program.id);

    _data_buffer_color.add_element(_data_element_color);
    _data_buffer_color.configure_by_name(program.id);

    let _index_buffer = IndexBuffer::new(_index_triangle.as_ptr(), _index_triangle.len() * std::mem::size_of::<cgmath::Vector3<i32>>());

    program.bind();
    program.set_float("uFloat", 1.0);

    while !canvas.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        program.bind();
        vertex_array.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 1000, gl::UNSIGNED_INT, std::ptr::null());
        }

        canvas.on_update();
    }
}
