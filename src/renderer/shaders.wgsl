
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
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let fragCoord = in.clip_position.xy;

    let roundedRectSize = vec2<f32>(0.8, 0.4);  // Adjust size as needed
    let cornerRadii = vec4<f32>(0.1, 0.1, 0.1, 0.1);  // Adjust radii as needed

    let distance = box(fragCoord, roundedRectSize, cornerRadii);

    let colorInside = vec4<f32>(0.9559735, 0.45078585, 0.2422812, 1.0);  // Color inside the shape
    let colorOutside = vec4<f32>(0.01298, 0.01298, 0.02732, 1.0);  // Background color

    let color = mix(colorInside, colorOutside, step(0.0, distance));

    return vec4<f32>(color.rgb, 1.0);
}

fn box(point: vec2<f32>, size: vec2<f32>, cornerRadii: vec4<f32>) -> f32 {
    var cornerRadiiRight: vec2<f32> = cornerRadii.xy;

    if (point.x > 0.0) {
        cornerRadiiRight = cornerRadii.xy;
    } else {
        cornerRadiiRight = cornerRadii.zw;
    }

    var cornerRadiusX: f32 = cornerRadii.x;

    if (point.y > 0.0) {
        cornerRadiusX = cornerRadii.x;
    } else {
        cornerRadiusX = cornerRadii.y;
    }

    let distanceFromEdges: vec2<f32> = abs(point) - size + cornerRadiusX;
    return min(max(distanceFromEdges.x, distanceFromEdges.y), 0.0) + length(max(distanceFromEdges, vec2<f32>(0.0, 0.0)) - cornerRadiusX);
}
