use super::State;

impl State {
	pub fn render(&mut self) -> anyhow::Result<(), wgpu::SurfaceError> {
		let output = self.surface.get_current_texture()?;

		let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("Render Encoder"),
		});

		{
			let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
				label: Some("Render Pass"),
				color_attachments: &[Some(wgpu::RenderPassColorAttachment {
					view: &view,
					resolve_target: None,
					ops: wgpu::Operations {
						load: wgpu::LoadOp::Clear(wgpu::Color {
							r: 0.01298,
							g: 0.01298,
							b: 0.02732,
							a: 1.0,
						}),
						store: true,
					},
				})],
				depth_stencil_attachment: None,
			});
			render_pass.set_pipeline(&self.render_pipeline);
			render_pass.set_bind_group(0, &self.viewport_bind_group, &[]);
			render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
			render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
			render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
			render_pass.draw_indexed(0..self.num_indices, 0, 0..self.instances.len() as _);
		}

		self.queue.submit(std::iter::once(encoder.finish()));
		output.present();

		Ok(())
	}
}
