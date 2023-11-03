use winit::{
	event::WindowEvent,
	window::Window,
};

pub struct Application {
	pub surface: wgpu::Surface,
	pub device: wgpu::Device,
	pub queue: wgpu::Queue,
	pub config: wgpu::SurfaceConfiguration,
	pub size: winit::dpi::PhysicalSize<u32>,
	pub window: Window,
}

impl Application {
	pub async fn new(window: Window) -> Self {
		let size = window.inner_size();

		let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
			backends: wgpu::Backends::SECONDARY,
			dx12_shader_compiler: Default::default(),
		});

		let surface = unsafe { instance.create_surface(&window) }.unwrap();

		let adapter = instance
			.request_adapter(&wgpu::RequestAdapterOptionsBase {
				power_preference: wgpu::PowerPreference::HighPerformance,
				force_fallback_adapter: false,
				compatible_surface: Some(&surface),
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

		Self { window, surface, device, queue, config, size }
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

	pub fn input(&mut self, event: &WindowEvent) -> bool {
		false
	}

	pub fn update(&mut self) {
		// will update later
	}

	pub fn render(&mut self) -> anyhow::Result<(), wgpu::SurfaceError> {
		todo!()
	}
}
