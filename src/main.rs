extern crate cgmath as cgm;
extern crate gl;
extern crate glfw;

mod base;
mod graphic;
mod light;
mod primitive;
mod component;

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
use graphic::cube_map::CubeMap;
use graphic::cube_map::CubeMapError;

use graphic::camera::Camera;

use primitive::cube::Cube;
use primitive::plane::Plane;
use primitive::sphere::Sphere;
use primitive::sky_box::SkyBox;

use light::Light;

const TITLE: &str = "OpenGL";
const WIDTH: u32 = 1920;
const HEIGHT: u32 = 1080;

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

    let default_vertex_shader =
        match Shader::new("assets/shaders/pbr.vertex.glsl", ShaderType::VertexShader) {
            Ok(v) => v,
            Err(ShaderError::FailedOpeningFile) => {
                println!("Failed opening vertex shader file, file may not exist or the path is wrong");
                return;
            }
            Err(ShaderError::FailedReadingFile) => {
                println!("Vertex shader file was not readable, check the file content or permission");
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

    let skybox_vertex_shader =
        match Shader::new("assets/shaders/skybox.vertex.glsl", ShaderType::VertexShader) {
            Ok(v) => v,
            Err(ShaderError::FailedOpeningFile) => {
                println!("Failed opening vertex shader file, file may not exist or the path is wrong");
                return;
            }
            Err(ShaderError::FailedReadingFile) => {
                println!("Vertex shader file was not readable, check the file content or permission");
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

    let default_fragment_shader = match Shader::new(
        "assets/shaders/pbr.fragment.glsl",
        ShaderType::FragmentShader,
    ) {
        Ok(v) => v,
        Err(ShaderError::FailedOpeningFile) => {
            println!("Failed opening fragment shader file, file may not exist or the path is wrong");
            return;
        }
        Err(ShaderError::FailedReadingFile) => {
            println!("Fragment shader file was not readable, check the file content or permission");
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

    let skybox_fragment_shader = match Shader::new(
        "assets/shaders/skybox.fragment.glsl",
        ShaderType::FragmentShader,
    ) {
        Ok(v) => v,
        Err(ShaderError::FailedOpeningFile) => {
            println!("Failed opening fragment shader file, file may not exist or the path is wrong");
            return;
        }
        Err(ShaderError::FailedReadingFile) => {
            println!("Fragment shader file was not readable, check the file content or permission");
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

    let default_program = match Program::new(default_vertex_shader, default_fragment_shader) {
        Ok(program) => program,
        Err(ProgramError::FailedLinkingShader(error)) => {
            println!("Linking program failed: \n{}", error);
            return;
        }
    };

    let skybox_program = match Program::new(skybox_vertex_shader, skybox_fragment_shader) {
        Ok(program) => program,
        Err(ProgramError::FailedLinkingShader(error)) => {
            println!("Linking program failed: \n{}", error);
            return;
        }
    };

    let mut cube = Cube::new();
    cube.entity_mut().transform_mut().translate(cgm::Vector3::new(0.0, 0.0, -7.0));

    let mut plane = Plane::new(2.0, 2.0);
    plane.entity_mut().transform_mut().translate(cgm::Vector3::new(3.0, -1.0, -7.0));

    let mut sphere = Sphere::new(1.0, 10, 10);
    sphere.entity_mut().transform_mut().translate(cgm::Vector3::new(-3.0, 0.0, -7.0));

    let skybox = SkyBox::new();

    let mut camera = Camera::perspective(45.0, (WIDTH / HEIGHT) as f32, 0.1, 1000.0);

    let pbr_texture_images = [
        "assets/textures/albedo.png",
        "assets/textures/normal.png",
        "assets/textures/metallic.png",
        "assets/textures/roughness.png",
        "assets/textures/ao.png"
    ];

    let mut pbr_textures: Vec<Texture> = Vec::with_capacity(pbr_texture_images.len());

    for image in pbr_texture_images.iter() {
        let texture = match Texture::new(image) {
            Ok(texture) => texture,
            Err(TextureError::OpeningTextureFailed) => {
                println!("Loading {} pbr texture failed", image);
                return;
            }
        };

        pbr_textures.push(texture);
    }

    let skybox_map_texture = match CubeMap::new(
        "assets/textures/skybox_right.png",
        "assets/textures/skybox_left.png",
        "assets/textures/skybox_top.png",
        "assets/textures/skybox_bottom.png",
        "assets/textures/skybox_back.png",
        "assets/textures/skybox_front.png"
    ) {
        Ok(cube_map) => cube_map,
        Err(CubeMapError::OpeningTextureFailed(error_message)) => {
            println!("{}", error_message);
            return;
        }
    };


    let mut light = Light::new(
        cgm::Vector3::new(0.0, 1.0, 0.0),
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

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let mut dir = 0.0;

        // rotation all entities
        if input.is_key_pressed_down(&Key::X) {
            dir = -1.0;
        } else if input.is_key_pressed_down(&Key::C) {
            dir = 1.0;
        }

        // translation of the camera
        if input.is_key_pressed_down(&Key::W) {
            camera.translate(-cgm::Vector3::unit_y() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::S) {
            camera.translate(cgm::Vector3::unit_y() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::D) {
            camera.translate(-cgm::Vector3::unit_x() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::A) {
            camera.translate(cgm::Vector3::unit_x() * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::Q) {
            camera.rotate_y(-20.0 * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::E) {
            camera.rotate_y(20.0 * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::R) {
            camera.rotate_x(-20.0 * tick.delta_time());
        }  else if input.is_key_pressed_down(&Key::F) {
            camera.rotate_x(20.0 * tick.delta_time());
        }

        // move of the light
        if input.is_key_pressed_down(&Key::I) {
            light.add_to_position(cgm::Vector3::unit_y() * 50.0 * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::K) {
            light.add_to_position(-cgm::Vector3::unit_y() * 50.0 * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::L) {
            light.add_to_position(cgm::Vector3::unit_x() * 50.0 * tick.delta_time());
        } else if input.is_key_pressed_down(&Key::J) {
            light.add_to_position(-cgm::Vector3::unit_x() * 50.0 * tick.delta_time());
        }

        // skybox
        unsafe {
            gl::Disable(gl::DEPTH_TEST);
        }

        skybox_program.bind();
        skybox_map_texture.bind();

        skybox_program.set_mat4f("projection", camera.get_projection());
        skybox_program.set_mat4f("view", camera.get_view());

        skybox.draw();

        // entities
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
        }

        default_program.bind();

        default_program.set_int("uAlbedoMap", 0);
        default_program.set_int("uNormalMap", 1);
        default_program.set_int("uMetallicMap", 2);
        default_program.set_int("uRoughnessMap", 3);
        default_program.set_int("uAOMap", 4);

        for (index, pbr_texture) in pbr_textures.iter().enumerate() {
            pbr_texture.bind_at_position(index as u32);
        }

        default_program.set_mat4f("projection", camera.get_projection());

        default_program.set_mat4f("view", camera.get_view());
        default_program.set_vec3f("uCameraPos", camera.position());
        default_program.set_mat4f("uLight", &light.as_matrix());


        // Plane
        plane.entity_mut().transform_mut().rotate_y(dir * 200.0 * tick.delta_time());
        plane.draw(&default_program);

        // Cube
        cube.entity_mut().transform_mut().rotate_y(dir * 200.0 * tick.delta_time());
        cube.draw(&default_program);

        // Sphere
        sphere.entity_mut().transform_mut().rotate_y(dir * 200.0 * tick.delta_time());
        sphere.draw(&default_program);

        canvas.on_update_end();
    }
}
