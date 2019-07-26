#![allow(dead_code)]

extern crate cgmath as cgm;
extern crate gl;
pub enum CameraType {
    Perspective,
    Orthographic,
}

pub struct Camera {
    camera_type: CameraType,
    projection_matrix: cgm::Matrix4<f32>,
    view_matrix: cgm::Matrix4<f32>,
    view_projection_matrix: cgm::Matrix4<f32>,
    position: cgm::Vector3<f32>,
}

impl Camera {
    pub fn perspective(fovy_deg: f32, aspect: f32, near: f32, far: f32) -> Camera {
        if near <= 0.0 || far <= 0.0 {
            panic!("near or far needs to greater than zero");
        }

        let projection = cgm::perspective(cgm::Deg(fovy_deg), aspect, near, far);
        let position: cgm::Vector3<f32> = cgm::Vector3::new(0.0, -5.0, 0.0);
        let mut view: cgm::Matrix4<f32> = cgm::Matrix4::from_translation(position);
        view +=
            cgm::Matrix4::<f32>::from_axis_angle(cgm::Vector3::new(1.0, 0.0, 0.0), cgm::Deg(45.0));

        Camera {
            camera_type: CameraType::Perspective,
            projection_matrix: projection,
            view_matrix: view,
            view_projection_matrix: projection * view,
            position: position,
        }
    }

    pub fn get_projection(&self) -> &cgm::Matrix4<f32> {
        &self.projection_matrix
    }

    pub fn get_view(&self) -> &cgm::Matrix4<f32> {
        &self.view_matrix
    }

    pub fn get_view_projection(&self) -> &cgm::Matrix4<f32> {
        &self.view_projection_matrix
    }
}