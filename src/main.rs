extern crate gl;
extern crate glfw;
extern crate cgmath as cgm;
extern crate image;

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

use graphic::texture::Texture;
use graphic::texture::TextureError;

const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub fn main() {
    let mut canvas = Canvas::new(TITLE, WIDTH, HEIGHT).expect("Window failed");

    let vertex_shader = match Shader::new(
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

    let fragment_shader = match Shader::new(
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

    let program = match Program::new(vertex_shader, fragment_shader) {
        Ok(program) => program,
        Err(ProgramError::FailedLinkingShader(error)) => {
            println!("Linking program failed: \n{}", error);
            return;
        }
    };

    let vertices_cube: [cgm::Vector3<f32>; 24] = [
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

    let color_cube: [cgm::Vector4<f32>; 24] = [
        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),

        cgm::Vector4::new( 1.0, 0.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 1.0, 0.0, 1.0 ),
        cgm::Vector4::new( 0.0, 0.0, 1.0, 1.0 ),
        cgm::Vector4::new( 1.0, 1.0, 1.0, 1.0 ),
    ];

    let index_cube = [
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

    let uvs_cube: [cgm::Vector2<f32>; 24] = [
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

    let mut data_buffer = DataBuffer::new(vertices_cube.as_ptr(), vertices_cube.len() * std::mem::size_of::<cgm::Vector3<f32>>());
    let data_element = BufferElement::new(BufferDataType::Float3, "aPos", false);

    let mut data_buffer_color = DataBuffer::new(color_cube.as_ptr(), color_cube.len() * std::mem::size_of::<cgm::Vector4<f32>>());
    let data_element_color = BufferElement::new(BufferDataType::Float4, "aCol", false);

    let mut data_buffer_texture = DataBuffer::new(uvs_cube.as_ptr(), uvs_cube.len() * std::mem::size_of::<cgm::Vector2<f32>>());
    let data_element_texture = BufferElement::new(BufferDataType::Float2, "aUV", false);

    data_buffer.add_element(data_element);
    data_buffer.configure_by_name(program.id);

    data_buffer_color.add_element(data_element_color);
    data_buffer_color.configure_by_name(program.id);

    data_buffer_texture.add_element(data_element_texture);
    data_buffer_texture.configure_by_name(program.id);

    let _index_buffer = IndexBuffer::new(index_cube.as_ptr(), index_cube.len() * std::mem::size_of::<cgm::Vector3<i32>>());

    let projection = cgm::perspective(cgm::Deg(45.0), (WIDTH / HEIGHT) as f32, 0.1, 1000.0);
    let mut model = cgm::Matrix4::<f32>::from_translation(cgm::Vector3::new(0.0, 0.0, 0.0));
    model += cgm::Matrix4::<f32>::from_axis_angle(cgm::Vector3::new(0.0, 1.0, 0.0), cgm::Deg(45.0));
    let view = cgm::Matrix4::<f32>::from_translation(cgm::Vector3::new(0.0, 0.0, -10.0));

    program.bind();
    program.set_mat4f("projection", &projection);
    program.set_mat4f("model", &model);
    program.set_mat4f("view", &view);

    let texture = match Texture::new("src/assets/crate.jpg") {
        Ok(texture) => texture,
        Err(TextureError::OpeningTextureFailed) => {
            println!("Loading texture failed");
            return;
        }
    };
    texture.bind();

    // settings
    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::MIRRORED_REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::MIRRORED_REPEAT as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    }

    while !canvas.should_close() {
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let delta_time: f32 = unsafe {
            glfw::ffi::glfwGetTime() as f64
        } as f32;

        model = model * cgm::Matrix4::<f32>::from_angle_y(cgm::Deg(delta_time / 1000.0));
        program.set_mat4f("model", &model);

        program.bind();
        vertex_array.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 1000, gl::UNSIGNED_INT, std::ptr::null());
        }

        canvas.on_update();
    }
}
