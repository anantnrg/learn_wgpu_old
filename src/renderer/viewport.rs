use cgmath::prelude::*;

pub struct Viewport {
	pub eye: cgmath::Point3<f32>,
	pub target: cgmath::Point3<f32>,
	pub up: cgmath::Vector3<f32>,
	pub aspect: f32,
	pub fovy: f32,
	pub znear: f32,
	pub zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportUniform {
	view_proj: [[f32; 4]; 4],
}

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

impl Viewport {
	pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
		let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);
		let proj = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

		return OPENGL_TO_WGPU_MATRIX * proj * view;
	}
}

impl ViewportUniform {
	pub fn new() -> Self {
		Self { view_proj: cgmath::Matrix4::identity().into() }
	}

	pub fn update_view_proj(&mut self, viewport: &Viewport) {
		self.view_proj = viewport.build_view_projection_matrix().into();
	}
}
