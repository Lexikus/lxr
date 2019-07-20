#![allow(dead_code)]

use crate::graphic::vertex_array::VertexArray;
use crate::graphic::data_buffer::DataBuffer;
use crate::graphic::data_buffer::buffer_element::BufferDataType;
use crate::graphic::data_buffer::buffer_element::BufferElement;
use crate::graphic::index_buffer::IndexBuffer;

pub struct Cube {
    vertex_array: VertexArray,
    data_buffer: DataBuffer,
    index_buffer: IndexBuffer,
    data: [f32; 288],
    indices: [i32; 36],
}

impl Cube {
    pub fn new() -> Cube {
        let data: [f32; 288] = [
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
        ];

        let indices: [i32; 36] = [
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
        ];

        let vertex_array = VertexArray::new();
        vertex_array.bind();

        let mut data_buffer = DataBuffer::new(data.as_ptr(), data.len() * std::mem::size_of::<f32>());

        let buffer_element_position = BufferElement::new(BufferDataType::Float3, "aPos", false);
        data_buffer.add_element(buffer_element_position);

        let buffer_element_normal = BufferElement::new(BufferDataType::Float3, "aNor", false);
        data_buffer.add_element(buffer_element_normal);

        let buffer_element_uv = BufferElement::new(BufferDataType::Float2, "aUV", false);
        data_buffer.add_element(buffer_element_uv);

        let buffer_element_color = BufferElement::new(BufferDataType::Float4, "aCol", false);
        data_buffer.add_element(buffer_element_color);
        data_buffer.configure_by_index();

        let index_buffer = IndexBuffer::new(indices.as_ptr(), indices.len() * std::mem::size_of::<i32>());

        vertex_array.unbind();
        data_buffer.unbind();
        index_buffer.unbind();

        Cube {
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