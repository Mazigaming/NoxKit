struct GlobalUniforms {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> globals: GlobalUniforms;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) rect_pos: vec2<f32>,
    @location(3) rect_size: vec2<f32>,
    @location(4) corner_radius: f32,
    @location(5) shape_type: f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) local_pos: vec2<f32>,
    @location(2) rect_size: vec2<f32>,
    @location(3) corner_radius: f32,
    @location(4) shape_type: f32,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = globals.view_proj * vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    let center = model.rect_pos + model.rect_size * 0.5;
    out.local_pos = model.position - center;
    out.rect_size = model.rect_size;
    out.corner_radius = model.corner_radius;
    out.shape_type = model.shape_type;
    return out;
}

fn sdRoundedBox(p: vec2<f32>, b: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - b + r;
    return length(max(q, vec2<f32>(0.0))) + min(max(q.x, q.y), 0.0) - r;
}

fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var dist: f32;
    let half_size = in.rect_size * 0.5;

    if (in.shape_type < 0.5) { // Rect
        dist = sdRoundedBox(in.local_pos, half_size, 0.0);
    } else if (in.shape_type < 1.5) { // Rounded Rect
        dist = sdRoundedBox(in.local_pos, half_size, in.corner_radius);
    } else { // Circle
        let radius = min(half_size.x, half_size.y);
        dist = sdCircle(in.local_pos, radius);
    }
    
    let smoothing = fwidth(dist);
    let alpha = 1.0 - smoothstep(-smoothing, smoothing, dist);
    
    if (alpha <= 0.0) {
        discard;
    }
    
    return vec4<f32>(in.color.rgb, in.color.a * alpha);
}
