extern crate gl;
extern crate glfw;

extern crate cgmath as cgm;

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

    let _vertices_cube: [cgm::Vector3<f32>; 24] = [
        cgm::Vector3::new( -1.0, -1.0, 1.0 ),
        cgm::Vector3::new(  1.0, -1.0, 1.0 ),
        cgm::Vector3::new(  1.0,  1.0, 1.0 ),
        cgm::Vector3::new( -1.0,  1.0, 1.0 ),

        cgm::Vector3::new( -1.0, -1.0, -1.0 ),
        cgm::Vector3::new(  1.0, -1.0, -1.0 ),
        cgm::Vector3::new(  1.0,  1.0, -1.0 ),
        cgm::Vector3::new( -1.0,  1.0, -1.0 ),

        cgm::Vector3::new(  1.0, -1.0,  1.0 ),
        cgm::Vector3::new(  1.0, -1.0, -1.0 ),
        cgm::Vector3::new(  1.0,  1.0, -1.0 ),
        cgm::Vector3::new(  1.0,  1.0,  1.0 ),

        cgm::Vector3::new( -1.0, -1.0,  1.0 ),
        cgm::Vector3::new( -1.0, -1.0, -1.0 ),
        cgm::Vector3::new( -1.0,  1.0, -1.0 ),
        cgm::Vector3::new( -1.0,  1.0,  1.0 ),

        cgm::Vector3::new( -1.0,  1.0,  1.0 ),
        cgm::Vector3::new(  1.0,  1.0,  1.0 ),
        cgm::Vector3::new(  1.0,  1.0, -1.0 ),
        cgm::Vector3::new( -1.0,  1.0, -1.0 ),

        cgm::Vector3::new( -1.0, -1.0,  1.0 ),
        cgm::Vector3::new( 1.0,  -1.0,  1.0 ),
        cgm::Vector3::new( 1.0,  -1.0, -1.0 ),
        cgm::Vector3::new( -1.0, -1.0, -1.0 ),
    ];

    let _color_cube: [cgm::Vector4<f32>; 24] = [
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
    ];

    let _index_cube = [
        cgm::Vector3::new( 0, 1, 2 ),
        cgm::Vector3::new( 0, 2, 3 ),

        cgm::Vector3::new( 8, 9, 10 ),
        cgm::Vector3::new( 8, 10, 11 ),

        cgm::Vector3::new( 5, 4, 7 ),
        cgm::Vector3::new( 5, 7, 6 ),

        cgm::Vector3::new( 13, 12, 15 ),
        cgm::Vector3::new( 13, 15, 14 ),

        cgm::Vector3::new( 16, 17, 18 ),
        cgm::Vector3::new( 16, 18, 19 ),

        cgm::Vector3::new( 21, 20, 23 ),
        cgm::Vector3::new( 21, 23, 22),
    ];

    let _uvs_cube: [cgm::Vector2<f32>; 24] = [
        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),

        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),

        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),

        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),

        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),

        cgm::Vector2::new( 0.0, 0.0 ),
        cgm::Vector2::new( 1.0, 0.0 ),
        cgm::Vector2::new( 1.0, 1.0 ),
        cgm::Vector2::new( 0.0, 1.0 ),
    ];

    let vertex_array = VertexArray::new();
    vertex_array.bind();

    let mut _data_buffer = DataBuffer::new(_vertices_cube.as_ptr(), _vertices_cube.len() * std::mem::size_of::<cgm::Vector3<f32>>());
    let _data_element = BufferElement::new(BufferDataType::Float3, "aPos", false);

    let mut _data_buffer_color = DataBuffer::new(_color_cube.as_ptr(), _color_cube.len() * std::mem::size_of::<cgm::Vector4<f32>>());
    let _data_element_color = BufferElement::new(BufferDataType::Float4, "aCol", false);

    _data_buffer.add_element(_data_element);
    _data_buffer.configure_by_name(program.id);

    _data_buffer_color.add_element(_data_element_color);
    _data_buffer_color.configure_by_name(program.id);

    let _index_buffer = IndexBuffer::new(_index_cube.as_ptr(), _index_cube.len() * std::mem::size_of::<cgm::Vector3<i32>>());

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
