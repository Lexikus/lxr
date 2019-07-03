extern crate gl;
extern crate glfw;

extern crate cgmath;

mod graphic;

use graphic::canvas;

use graphic::shader;
use graphic::vertexarray;

// settings
const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut canvas = canvas::Canvas::new(TITLE, WIDTH, HEIGHT).expect("Window failed");

    let vertex_shader = match shader::Shader::new(
        "src/assets/vertex.shader.glsl",
        shader::ShaderType::VertexShader,
    ) {
        Ok(v) => v,
        Err(shader::ShaderError::FailedOpeningFile) => {
            println!("Failed opening file, file may not exist or the path is wrong");
            return;
        }
        Err(shader::ShaderError::FailedReadingFile) => {
            println!("File was not readable, check file content and permissions");
            return;
        }
        Err(shader::ShaderError::FailedCompilingShader) => {
            println!("Compiling shader failed, check shader content");
            return;
        }
    };

    let fragment_shader = match shader::Shader::new(
        "src/assets/fragment.shader.glsl",
        shader::ShaderType::FragmentShader,
    ) {
        Ok(v) => v,
        Err(shader::ShaderError::FailedOpeningFile) => {
            println!("Failed opening file, file may not exist or the path is wrong");
            return;
        }
        Err(shader::ShaderError::FailedReadingFile) => {
            println!("File was not readable, check file content and permissions");
            return;
        }
        Err(shader::ShaderError::FailedCompilingShader) => {
            println!("Compiling shader failed, check shader content");
            return;
        }
    };

    let vertexarray = vertexarray::VertexArray::new();
    vertexarray.bind();

    let vertices_triangle: [f32; 9] = [0.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0];
    let index_triangle = [0, 1, 2];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ibo = 0;

    // render loop
    // -----------
    while !canvas.should_close() {
        unsafe {
            gl::ClearColor(1.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        canvas.on_update();
    }
}
