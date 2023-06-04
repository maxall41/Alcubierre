use cgmath::{ortho, perspective, vec2, vec4, Angle, Matrix4, Rad, SquareMatrix};
use wgpu_glyph::orthographic_projection;
use winit::dpi::PhysicalSize;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

#[derive(Debug)]
pub struct Camera {
    pub(crate) eye: cgmath::Point3<f32>,
    pub(crate) target: cgmath::Point3<f32>,
    pub(crate) up: cgmath::Vector3<f32>,
}

impl Camera {
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up)

        // Matrix4::look_to_rh(
        //     self.position,
        //     Vector3::new(
        //         cos_pitch * cos_yaw,
        //         sin_pitch,
        //         cos_pitch * sin_yaw
        //     ).normalize(),
        //     Vector3::unit_y(),
        // )
    }
}

// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    // We can't use cgmath with bytemuck directly so we'll have
    // to convert the Matrix4 into a 4x4 f32 array
    pub view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        use cgmath::SquareMatrix;
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub(crate) fn update_view_proj(&mut self, camera: &Camera, projection: &Projection) {
        // self.view_position = camera.position.to_homogeneous().into();
        let view = camera.calc_matrix();
        let proj = projection.calc_matrix();
        self.view_proj = (proj * view).into();
    }
}

pub struct Projection {
    pub(crate) aspect: f32,
    pub(crate) fovy: Rad<f32>,
    znear: f32,
    zfar: f32
}

impl Projection {
    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    pub fn calc_matrix(&self) -> Matrix4<f32> {
        // OPENGL_TO_WGPU_MATRIX * perspective(self.fovy, self.aspect, self.znear, self.zfar)
        OPENGL_TO_WGPU_MATRIX * ortho(-7.0*self.aspect, 7.0*self.aspect, -7.0, 7.0, 0.001, 1000.0)
    }
}

pub fn screen_space_to_ndc_space(
    x: u32,
    y: u32,
    window_width: f32,
    window_height: f32,
) -> (f32, f32) {
    let new_x = ((x as f32 / window_width as f32) - 0.5) * 2.0;
    let new_y = ((y as f32 / window_height as f32) - 0.5) * 2.0;

    (new_x, new_y)
}

pub fn screen_space_to_view_space(
    x: u32,
    y: u32,
    window_width: f32,
    window_height: f32,
    proj_matrix: Matrix4<f32>,
) -> (f32, f32) {
    // Based off of https://stackoverflow.com/questions/46749675/opengl-mouse-coordinates-to-space-coordinates/46752492#46752492

    let (ndc_x, ndc_y) = screen_space_to_ndc_space(x, y, window_width, window_height);

    let inverted_proj_matrix = proj_matrix.invert().unwrap();

    let projected = inverted_proj_matrix * vec4(ndc_x, ndc_y, 0.0, 1.0);

    (projected.x, -projected.y)
}
