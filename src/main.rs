extern crate glfw;
extern crate gl;
extern crate cgmath;

mod graphic;

use graphic::canvas;
use graphic::shader;
use cgmath::Vector3;

// settings
const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

// glm::vec3 vertices_rectangle[] = {
//     {-0.5f, 0.5f, 0.0f},
//     {-0.5f, -0.5f, 0.0f},
//     {0.5f, -0.5f, 0.0f},
//     {0.5f, 0.5f, 0.0f}};
// glm::uvec3 index_rectangle[] = {{0, 1, 2}, {2, 3, 0}};

// glm::vec3 vertices_cube[] = {
//     // front up left 0
//     {-0.5f, 0.5f, 0.5f},
//     // front down left 1
//     {-0.5f, -0.5f, 0.5f},
//     // front down right 2
//     {0.5f, -0.5f, 0.5f},
//     // front up right 3
//     {0.5f, 0.5f, 0.5f},

//     // back up left 4
//     {-0.5f, 0.5f, -0.5f},
//     // back down left 5
//     {-0.5f, -0.5f, -0.5f},
//     // back down right 6
//     {0.5f, -0.5f, -0.5f},
//     // back up right 7
//     {0.5f, 0.5f, -0.5f},
// };
// glm::uvec3 index_cube[] = {
//     // front face
//     {0, 1, 2},
//     {2, 3, 0},
//     // back face
//     {7, 6, 5},
//     {5, 4, 6},
//     // left face
//     {4, 5, 1},
//     {1, 0, 4},
//     // right face
//     {3, 2, 6},
//     {6, 7, 3},
// };

// // float pyramide
// glm::vec3 vertices_pyramide[] = {
//     // corner
//     {0.0f, 0.5f, 0.0f},
//     // front left
//     {0.5f, -0.5f, 0.0f},
//     // front right
//     {-0.5f, -0.5f, 0.0f},
//     // back
//     {0.0f, -0.5f, 0.5f},

// };

// glm::uvec3 index_pyramide[] = {
//     {0, 1, 2},
//     {0, 2, 3},
//     {0, 3, 1},
// };

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut canvas = canvas::Canvas::new(TITLE, WIDTH, HEIGHT);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    canvas.load_graphic_library_functions();

    let shader = shader::Shader::new("src/assets/vertex.shader.glsl", "src/assets/fragment.shader.glsl");

    let vertices_triangle: [Vector3<f32>; 3] = [
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(1.0, -1.0, 0.0),
        Vector3::new(-1.0, -1.0, 0.0),
    ];

    let index_triangle: [Vector3<i32>; 1] = [
        Vector3::new(0, 1, 2),
    ];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ibo = 0;

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ibo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        // glBufferData(gl::ARRAY_BUFFER, verticesSize, vertices, gl::STATIC_DRAW);

        // glVertexAttribPointer(0, 3, GL_FLOAT, GL_FALSE, 3 * sizeof(float), (void *)0);
        gl::EnableVertexAttribArray(0);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        // glBufferData(gl::ELEMENT_ARRAY_BUFFER, indicesSize, indicies, gl::STATIC_DRAW);
    }

    // render loop
    // -----------
    while !canvas.should_close {

        let delta_time: f64 = unsafe {
            glfw::ffi::glfwGetTime() as f64
        };

        // events
        // -----
        canvas.process_events();

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        canvas.swap_buffers();
        canvas.poll_events();
    }
}