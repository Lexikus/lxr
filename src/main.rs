extern crate cgmath as cgm;
extern crate gl;
extern crate glfw;

use cgm::prelude::Matrix;
use cgm::prelude::SquareMatrix;

mod base;
mod graphic;
mod light;
mod primitive;

use base::canvas::Canvas;
use base::input::Input;
use base::keyboard::Key;
use base::tick::Tick;

use graphic::shader::Shader;
use graphic::shader::ShaderError;
use graphic::shader::ShaderType;

use graphic::program::Program;
use graphic::program::ProgramError;

use graphic::texture::Texture;
use graphic::texture::TextureError;

use graphic::camera::Camera;

use primitive::cube::Cube;
use primitive::plane::Plane;
use primitive::sphere::Sphere;

use light::Light;

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
    let mut tick = Tick::new();

    let vertex_shader =
        match Shader::new("assets/shaders/light.vertex.glsl", ShaderType::VertexShader) {
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
                println!(
                    "Compiling vertex shader failed, check shader content\n{}",
                    error
                );
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
            println!(
                "Compiling fragment shader failed, check shader content:\n{}",
                error
            );
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
    let sphere = Sphere::new(1.0, 10, 10);

    let mut camera = Camera::perspective(45.0, (WIDTH / HEIGHT) as f32, 0.1, 1000.0);

    let mut model = cgm::Matrix4::<f32>::from_translation(cgm::Vector3::new(0.0, 0.0, -7.0));
    let mut model_tangent: cgm::Matrix4<f32>;

    let texture = match Texture::new("assets/textures/crate.jpg") {
        Ok(texture) => texture,
        Err(TextureError::OpeningTextureFailed) => {
            println!("Loading texture failed");
            return;
        }
    };

    let light = Light::new(
        cgm::Vector3::new(0.0, 1.0, 1.0),
        cgm::Vector3::new(0.75, 0.75, 1.0),
        cgm::Vector3::new(0.75, 0.75, 1.0),
        0.5,
        1.0,
        0.8,
        1.0,
        1.0,
        1.0,
        64.0,
    );

    program.bind();
    program.set_mat4f("projection", camera.get_projection());

    texture.bind();

    // settings
    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::MIRRORED_REPEAT as i32,
        );

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
    }

    while !canvas.should_close() {
        canvas.on_update_begin(&mut input);
        tick.on_update();

        let mut dir = 0;

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        if input.is_key_pressed_down(&Key::Q) {
            dir = -1;
        } else if input.is_key_pressed_down(&Key::E) {
            dir = 1;
        }

        if input.is_key_pressed_down(&Key::W) {
            camera.translate(cgm::Vector3::unit_y() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::S) {
            camera.translate(-cgm::Vector3::unit_y() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::A) {
            camera.translate(-cgm::Vector3::unit_x() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::D) {
            camera.translate(cgm::Vector3::unit_x() * tick.delta_time());
        }

        model = model
            * cgm::Matrix4::<f32>::from_angle_y(cgm::Deg(dir as f32 * 100.0 * tick.delta_time()));
        model_tangent = model.invert().unwrap().transpose();

        program.set_float("uBrightness", tick.time().sin());
        program.set_float("uContrast", tick.time().sin());
        program.set_float("uGrayscale", tick.time().sin().abs());

        program.set_mat4f("model", &model);
        program.set_mat4f("uTangentToWorld", &model_tangent);

        program.set_mat4f("view", camera.get_view());
        program.set_vec3f("uViewPos", camera.position());
        program.set_mat4f("uLight", &light.as_matrix());

        // plane.bind();
        cube.bind();
        // sphere.bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 1000, gl::UNSIGNED_INT, std::ptr::null());
        }

        canvas.on_update_end();
    }
}
