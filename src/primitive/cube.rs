#![allow(dead_code)]

use crate::component::entity::Entity;
use crate::graphic::program::Program;

pub struct Cube {
    entity: Entity,
}

impl Cube {
    pub fn new() -> Cube {
        let data: Vec<f32> = vec!(
            //~~~~verices~~~    ~~~~~normals~~~~   ~~~~uv~~~   ~~~~~~~color~~~~~~~~
            // forward
            -1.0, -1.0,  1.0,    0.0,  0.0,  1.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0, -1.0,  1.0,    0.0,  0.0,  1.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0,  1.0,    0.0,  0.0,  1.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            -1.0,  1.0,  1.0,    0.0,  0.0,  1.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            // back
            -1.0, -1.0, -1.0,    0.0,  0.0, -1.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0, -1.0, -1.0,    0.0,  0.0, -1.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0, -1.0,    0.0,  0.0, -1.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            -1.0,  1.0, -1.0,    0.0,  0.0, -1.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            // right
             1.0, -1.0,  1.0,    1.0,  0.0,  0.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0, -1.0, -1.0,    1.0,  0.0,  0.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0, -1.0,    1.0,  0.0,  0.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0,  1.0,    1.0,  0.0,  0.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            // left
            -1.0, -1.0,  1.0,   -1.0,  0.0,  0.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0,   -1.0,  0.0,  0.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
            -1.0,  1.0, -1.0,   -1.0,  0.0,  0.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            -1.0,  1.0,  1.0,   -1.0,  0.0,  0.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            // top
            -1.0,  1.0,  1.0,    0.0,  1.0,  0.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0,  1.0,    0.0,  1.0,  0.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0,  1.0, -1.0,    0.0,  1.0,  0.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            -1.0,  1.0, -1.0,    0.0,  1.0,  0.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            // bottom
            -1.0, -1.0,  1.0,    0.0, -1.0,  0.0,   0.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0, -1.0,  1.0,    0.0, -1.0,  0.0,   1.0, 0.0,   1.0, 1.0, 1.0, 1.0,
             1.0, -1.0, -1.0,    0.0, -1.0,  0.0,   1.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            -1.0, -1.0, -1.0,    0.0, -1.0,  0.0,   0.0, 1.0,   1.0, 1.0, 1.0, 1.0,
            //~~~~verices~~~    ~~~~~normals~~~~   ~~~~uv~~~   ~~~~~~~color~~~~~~~~
        );

        let indices: Vec<i32> = vec!(
             0,  1,  2,
             0,  2,  3,
             8,  9, 10,
             8, 10, 11,
             5,  4,  7,
             5,  7,  6,
            13, 12, 15,
            13, 15, 14,
            16, 17, 18,
            16, 18, 19,
            21, 20, 23,
            21, 23, 22,
        );

        Cube {
            entity: Entity::new(data, indices),
        }
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn entity_mut(&mut self) -> &mut Entity {
        &mut self.entity
    }

    pub fn draw(&self, program: &Program) {
        program.set_mat4f("model", self.entity.transform().matrix());
        program.set_mat4f("uTangentToWorld", &self.entity.transform().matrix_tangent());
        self.entity.mesh().bind();

        unsafe {
            gl::DrawElements(gl::TRIANGLES, 1000, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}