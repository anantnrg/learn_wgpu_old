struct ViewportUniform {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> viewport: ViewportUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = viewport.view_proj * vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}

fn box(in_coords: vec2<f32>, in_size: vec2<f32>, in_radius: vec4<f32>) -> f32 {
    var radius: vec4<f32>;
    var q: vec2<f32>;
    if (in_coords[0] > 0.0) {
        radius.x = radius.x;
        radius.y = radius.y;
    } else {
        radius.x = radius.z;
        radius.y = radius.w;
    }

    if (in_coords[1] > 0.0) {
        radius.z = radius.x;
        radius.w = radius.y;
    } else {
        radius.z = radius.y;
        radius.w = radius.x;
    }

    q = abs(in_coords) - in_size + radius.xy;

    var min_q = min(q.x, q.y);
    var max_q = max(q.x, q.y);

    var distance = min(max_q, 0.0) + length(max(q, vec2<f32>(0.0, 0.0)) - radius.x);

    return distance;
}