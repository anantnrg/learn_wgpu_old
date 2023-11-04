pub fn rgb_to_srgb(rgb_color: (u8, u8, u8)) -> [f32; 3] {
	let (r, g, b) = rgb_color;

	let r = f32::from(r) / 255.0;
	let g = f32::from(g) / 255.0;
	let b = f32::from(b) / 255.0;

	let srgb_r = if r <= 0.04045 { r / 12.92 } else { ((r + 0.055) / 1.055).powf(2.4) };
	let srgb_g = if g <= 0.04045 { g / 12.92 } else { ((g + 0.055) / 1.055).powf(2.4) };
	let srgb_b = if b <= 0.04045 { b / 12.92 } else { ((b + 0.055) / 1.055).powf(2.4) };

	[srgb_r, srgb_g, srgb_b]
}
