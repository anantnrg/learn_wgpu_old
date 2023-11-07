pub mod primitives;
pub mod render;
pub mod viewport;

use crate::structs::render::Vertex;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;
use winit::{
	event::WindowEvent,
	window::Window,
};

use self::{
	primitives::r#box::{
		Box,
		BoxRaw,
	},
	viewport::{
		Viewport,
		ViewportUniform,
	},
};

const VERTICES: &[Vertex] = &[
	Vertex { position: [-0.4, 0.2, 0.0], color: [0.9559735, 0.45078585, 0.2422812] },
	Vertex { position: [-0.4, -0.2, 0.0], color: [0.9559735, 0.45078585, 0.2422812] },
	Vertex { position: [0.4, -0.2, 0.0], color: [0.9559735, 0.45078585, 0.2422812] },
	Vertex { position: [0.4, 0.2, 0.0], color: [0.9559735, 0.45078585, 0.2422812] },
];

const INDICES: &[u16] = &[0, 1, 2, 2, 3, 0];

const NUM_INSTANCES_PER_ROW: u32 = 1;
const INSTANCE_DISPLACEMENT: cgmath::Vector3<f32> = cgmath::Vector3::new(0.0, 0.0, 0.0);

pub struct State {
	pub surface: wgpu::Surface,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,
	pub render_pipeline: wgpu::RenderPipeline,
	pub vertex_buffer: wgpu::Buffer,
	pub index_buffer: wgpu::Buffer,
	pub num_indices: u32,
	pub viewport: viewport::Viewport,
	pub viewport_uniform: ViewportUniform,
	pub viewport_buffer: wgpu::Buffer,
	pub viewport_bind_group: wgpu::BindGroup,
	instances: Vec<primitives::r#box::Box>,
	instance_buffer: wgpu::Buffer,
	pub window: Window,
}

impl State {
	pub async fn new(window: Window) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::PRIMARY,
			dx12_shader_compiler: Default::default(),
		});

		let surface = unsafe { instance.create_surface(&window) }.unwrap();

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptions {
				power_preference: wgpu::PowerPreference::HighPerformance,
				compatible_surface: Some(&surface),
				force_fallback_adapter: false,
			})
			.await
			.unwrap();

		let (device, queue) = adapter
			.request_device(
				&wgpu::DeviceDescriptor {
					features: wgpu::Features::empty(),
					limits: wgpu::Limits::default(),
					label: None,
				},
				None,
			)
			.await
			.unwrap();

		let surface_caps = surface.get_capabilities(&adapter);
		let surface_format = surface_caps
			.formats
			.iter()
			.copied()
			.find(|f| f.is_srgb())
			.unwrap_or(surface_caps.formats[0]);
		let config = wgpu::SurfaceConfiguration {
			usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
			format: surface_format,
			width: size.width,
			height: size.height,
			present_mode: surface_caps.present_modes[0],
			alpha_mode: surface_caps.alpha_modes[0],
			view_formats: vec![],
		};
		surface.configure(&device, &config);

		let shader = device.create_shader_module(wgpu::include_wgsl!("shaders.wgsl"));

		let viewport = Viewport {
			eye: (0.0, 0.0, 12.0).into(),
			target: (0.0, 0.0, 0.0).into(),
			up: cgmath::Vector3::unit_y(),
			aspect: config.width as f32 / config.height as f32,
			fovy: 45.0,
			znear: 0.1,
			zfar: 100.0,
		};

		let mut viewport_uniform = viewport::ViewportUniform::new();
		viewport_uniform.update_view_proj(&viewport);

		let viewport_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Viewport Buffer"),
			contents: bytemuck::cast_slice(&[viewport_uniform]),
			usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
		});

		let viewport_bind_group_layout =
			device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
				entries: &[wgpu::BindGroupLayoutEntry {
					binding: 0,
					visibility: wgpu::ShaderStages::VERTEX,
					ty: wgpu::BindingType::Buffer {
						ty: wgpu::BufferBindingType::Uniform,
						has_dynamic_offset: false,
						min_binding_size: None,
					},
					count: None,
				}],
				label: Some("viewport_bind_group_layout"),
			});

		let viewport_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
			layout: &viewport_bind_group_layout,
			entries: &[wgpu::BindGroupEntry {
				binding: 0,
				resource: viewport_buffer.as_entire_binding(),
			}],
			label: Some("camera_bind_group"),
		});

		// let instances = (0..NUM_INSTANCES_PER_ROW)
		// 	.flat_map(|z| {
		// 		(0..NUM_INSTANCES_PER_ROW).map(move |x| {
		// 			let position = cgmath::Vector3 { x: x as f32, y: 0.0, z: z as f32 }
		// 				- INSTANCE_DISPLACEMENT;

		// 			let rotation = if position.is_zero() {
		// 				cgmath::Quaternion::from_axis_angle(
		// 					cgmath::Vector3::unit_z(),
		// 					cgmath::Deg(0.0),
		// 				)
		// 			} else {
		// 				cgmath::Quaternion::from_axis_angle(position.normalize(), cgmath::Deg(45.0))
		// 			};

		// 			primitives::r#box::Box { position, rotation }
		// 		})
		// 	})
		// 	.collect::<Vec<_>>();

		let instances = vec![Box {
			position: cgmath::Vector3::new(0.0, 0.0, 0.0),
			rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
			corner_radius: 0.4,
		}];

		let instance_data =
			instances.iter().map(primitives::r#box::Box::to_raw).collect::<Vec<_>>();
		let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Instance Buffer"),
			contents: bytemuck::cast_slice(&instance_data),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let render_pipeline_layout =
			device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
				label: Some("Render Pipeline Layout"),
				bind_group_layouts: &[&viewport_bind_group_layout],
				push_constant_ranges: &[],
			});

		let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
			label: Some("Render Pipeline"),
			layout: Some(&render_pipeline_layout),
			vertex: wgpu::VertexState {
				module: &shader,
				entry_point: "vs_main",
				buffers: &[Vertex::desc(), BoxRaw::desc()],
			},
			fragment: Some(wgpu::FragmentState {
				module: &shader,
				entry_point: "fs_main",
				targets: &[Some(wgpu::ColorTargetState {
					format: config.format,
					blend: Some(wgpu::BlendState {
						color: wgpu::BlendComponent::REPLACE,
						alpha: wgpu::BlendComponent::REPLACE,
					}),
					write_mask: wgpu::ColorWrites::ALL,
				})],
			}),
			primitive: wgpu::PrimitiveState {
				topology: wgpu::PrimitiveTopology::TriangleList,
				strip_index_format: None,
				front_face: wgpu::FrontFace::Ccw,
				cull_mode: Some(wgpu::Face::Back),
				polygon_mode: wgpu::PolygonMode::Fill,
				unclipped_depth: false,
				conservative: false,
			},
			depth_stencil: None,
			multisample: wgpu::MultisampleState {
				count: 1,
				mask: !0,
				alpha_to_coverage_enabled: false,
			},
			multiview: None,
		});

		let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Vertex Buffer"),
			contents: bytemuck::cast_slice(VERTICES),
			usage: wgpu::BufferUsages::VERTEX,
		});

		let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
			label: Some("Index Buffer"),
			contents: bytemuck::cast_slice(INDICES),
			usage: wgpu::BufferUsages::INDEX,
		});
		let num_indices = INDICES.len() as u32;

		Self {
			window,
			surface,
			device,
			queue,
			config,
			size,
			render_pipeline,
			vertex_buffer,
			index_buffer,
			num_indices,
			viewport,
			viewport_bind_group,
			viewport_buffer,
			viewport_uniform,
			instances,
			instance_buffer,
		}
	}

	pub fn window(&self) -> &Window {
		&self.window
	}

	pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
		if new_size.width > 0 && new_size.height > 0 {
			self.size = new_size;
			self.config.width = new_size.width;
			self.config.height = new_size.height;
			self.surface.configure(&self.device, &self.config);
		}
	}

	pub fn input(&mut self, _event: &WindowEvent) -> bool {
		false
	}

	pub fn update(&mut self) {
		// will update later
	}
}
