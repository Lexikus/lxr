#![allow(dead_code)]

extern crate cgmath as cgm;

use cgm::Vector3;
use cgm::Vector2;

use crate::graphic::data_buffer::buffer_element::BufferDataType;
use crate::graphic::data_buffer::buffer_element::BufferElement;
use crate::graphic::data_buffer::DataBuffer;
use crate::graphic::index_buffer::IndexBuffer;
use crate::graphic::vertex_array::VertexArray;

pub struct Plane {
    vertex_array: VertexArray,
    data_buffer: DataBuffer,
    index_buffer: IndexBuffer,
    data: Vec<f32>,
    indices: Vec<i32>,
}

impl Plane {
    pub fn new(width: f32, length: f32) -> Plane {
        let res_x: usize = 2;
        let res_z: usize = 2;

        let capacity = res_x * res_z;

        let mut vertices = Vec::with_capacity(capacity);
        super::fill(&mut vertices, Vector3::<f32>::new(0.0, 0.0, 0.0), capacity);

        for z in 0..res_z {
            let z_pos: f32 = (z as f32 / (res_z - 1) as f32 - 0.5) * length;

            for x in 0..res_x {
                let x_pos = (x as f32 / (res_x - 1) as f32 - 0.5) * width;
                vertices[x + z * res_x] = Vector3::<f32>::new(x_pos, 0.0, z_pos);
            }
        }

        let mut normals = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            normals.push(Vector3::<f32>::unit_y());
        }

        let mut uvs = Vec::with_capacity(capacity);
        super::fill(&mut uvs, Vector2::<f32>::new(0.0, 0.0), capacity);
        for v in 0..res_z {
            for u in 0..res_x {
                uvs[u + v * res_x] = Vector2::new(
                    u as f32 / (res_x - 1) as f32,
                    v as f32 / (res_z - 1) as f32,
                );
            }
        }

        let faces: i32 = (res_x as i32 - 1) * (res_z as i32 - 1);
        let mut indices = Vec::with_capacity(faces as usize * 6);
        super::fill(&mut indices, 0, faces as usize * 6);

        let mut t: usize = 0;
        for face in 0..faces {
            let i: i32 = face % (res_x - 1) as i32 + (face / (res_z - 1) as i32 * res_x as i32);

            indices[t] = i + res_x as i32;
            t += 1;
            indices[t] = i + 1;
            t += 1;
            indices[t] = i;
            t += 1;

            indices[t] = i + res_x as i32;
            t += 1;
            indices[t] = i + res_x as i32 + 1;
            t += 1;
            indices[t] = i + 1;
            t += 1;
        }

        let vertex_array = VertexArray::new();
        vertex_array.bind();

        let mut data =
            Vec::with_capacity(capacity * 3 + capacity * 3 + capacity * 2 + capacity * 4);
        for (i, vertex) in vertices.iter().enumerate() {
            data.push(vertex.x);
            data.push(vertex.y);
            data.push(vertex.z);

            let normal = normals[i];
            data.push(normal.x);
            data.push(normal.y);
            data.push(normal.z);

            let uv = uvs[i];
            data.push(uv.x);
            data.push(uv.y);

            // color
            for _ in 0..4 {
                data.push(1.0);
            }
        }

        let mut data_buffer =
            DataBuffer::new(data.as_ptr(), data.len() * std::mem::size_of::<f32>());

        let buffer_element_position = BufferElement::new(BufferDataType::Float3, "aPos", false);
        data_buffer.add_element(buffer_element_position);

        let buffer_element_normal = BufferElement::new(BufferDataType::Float3, "aNor", false);
        data_buffer.add_element(buffer_element_normal);

        let buffer_element_uv = BufferElement::new(BufferDataType::Float2, "aUV", false);
        data_buffer.add_element(buffer_element_uv);

        let buffer_element_color = BufferElement::new(BufferDataType::Float4, "aCol", false);
        data_buffer.add_element(buffer_element_color);
        data_buffer.configure_by_index();

        let index_buffer =
            IndexBuffer::new(indices.as_ptr(), indices.len() * std::mem::size_of::<i32>());

        vertex_array.unbind();
        data_buffer.unbind();
        index_buffer.unbind();

        Plane {
            vertex_array: vertex_array,
            data_buffer: data_buffer,
            index_buffer: index_buffer,
            data: data,
            indices: indices,
        }
    }

    pub fn bind(&self) {
        self.vertex_array.bind();
    }

    pub fn unbind(&self) {
        self.vertex_array.unbind();
    }
}
