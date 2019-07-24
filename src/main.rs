extern crate gl;
extern crate glfw;
extern crate cgmath as cgm;

mod base;
mod graphic;
mod primitive;

use base::canvas::Canvas;
use base::input::Input;

use graphic::shader::Shader;
use graphic::shader::ShaderType;
use graphic::shader::ShaderError;

use graphic::program::Program;
use graphic::program::ProgramError;

use primitive::cube::Cube;
use primitive::plane::Plane;

use graphic::texture::Texture;
use graphic::texture::TextureError;

use graphic::camera::Camera;

use base::keyboard::Key;

const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

pub fn main() {
    let mut canvas = match Canvas::new(TITLE, WIDTH, HEIGHT) {
        Ok(canvas) => canvas,
        Err(_) => {
            println!("Canvas failed");
            return;
        }
    };
    let mut input = Input::new();

    let vertex_shader = match Shader::new(
        "assets/shaders/light.vertex.glsl",
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
        "assets/shaders/light.fragment.glsl",
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

    let cube = Cube::new();
    let plane = Plane::new(2.0, 2.0);

    let camera = Camera::perspective(45.0, (WIDTH / HEIGHT) as f32, 0.1, 1000.0);
    let mut model = cgm::Matrix4::<f32>::from_translation(cgm::Vector3::new(0.0, 0.0, -10.0));
    model += cgm::Matrix4::<f32>::from_axis_angle(cgm::Vector3::new(0.0, 1.0, 0.0), cgm::Deg(45.0));

    program.bind();
    program.set_mat4f("projection", camera.get_projection());
    program.set_mat4f("model", &model);
    program.set_mat4f("view", camera.get_view());

    let texture = match Texture::new("assets/textures/crate.jpg") {
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
        canvas.on_update_begin(&mut input);
        let mut dir = 0;

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let delta_time: f32 = unsafe {
            glfw::ffi::glfwGetTime() as f64
        } as f32;

        if input.is_key_pressed_down(&Key::A) {
            dir = -1;
        } else if input.is_key_pressed_down(&Key::D) {
            dir = 1;
        }

        model = model * cgm::Matrix4::<f32>::from_angle_y(cgm::Deg((dir * 2) as f32));
        program.set_float("uBrightness", delta_time.sin());
        program.set_float("uContrast", delta_time.sin());
        program.set_float("uGrayscale", delta_time.sin().abs());
        program.set_mat4f("model", &model);

        program.bind();
        plane.bind();
        // cube.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 1000, gl::UNSIGNED_INT, std::ptr::null());
        }

        canvas.on_update_end();
    }
}
