struct VertexInput {
    @location(0) v_pos: vec2<f32>,
    @location(1) v_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,   // required for position
    @location(0) frag_color: vec4<f32>,
};

struct Uniforms {
    m_mvp: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4<f32>(input.v_pos, 0.0, 1.0);
    out.frag_color = input.v_color;
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(input.frag_color);
}
