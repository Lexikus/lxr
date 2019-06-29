extern crate glfw;
extern crate gl;
use self::gl::types::*;

extern crate cgmath;

mod graphic;

use graphic::canvas;
use graphic::shader;

use std::mem;
use std::os::raw::c_void;
use std::ptr;
use std::str;

// settings
const TITLE: &str = "OpenGL";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

pub fn main() {
    // glfw: initialize and configure
    // ------------------------------
    let mut canvas = canvas::Canvas::new(TITLE, WIDTH, HEIGHT);

    // gl: load all OpenGL function pointers
    // ---------------------------------------
    canvas.load_graphic_library_functions();

    let shader = shader::Shader::new(
        "src/assets/vertex.shader.glsl",
        "src/assets/fragment.shader.glsl",
    );

    let vertices_triangle: [f32; 9] = [
        0.0, 1.0, 0.0,
        1.0, -1.0, 0.0,
        -1.0, -1.0, 0.0
    ];
    let index_triangle = [0, 1, 2];

    let mut vao = 0;
    let mut vbo = 0;
    let mut ibo = 0;

   unsafe {
        // set up vertex data (and buffer(s)) and configure vertex attributes
        // ------------------------------------------------------------------
        // HINT: type annotation is crucial since default for float literals is f64

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ibo);
        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices_triangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &vertices_triangle as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (index_triangle.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &index_triangle as *const i32 as *const c_void,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );

        gl::EnableVertexAttribArray(0);
    };

    // render loop
    // -----------
    while !canvas.should_close {
        // events
        // -----
        canvas.process_events();

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // draw our first triangle
            shader.use_program();

            gl::BindVertexArray(vao); // seeing as we only have a single VAO there's no need to bind it every time, but we'll do so to keep things a bit more organized
                                      // gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            // glBindVertexArray(0); // no need to unbind it every time
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        canvas.swap_buffers();
        canvas.poll_events();
    }
}
