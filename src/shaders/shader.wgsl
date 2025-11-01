struct VertexInput {
    @location(0) v_pos: vec3<f32>,
    @location(1) v_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) frag_color: vec4<f32>,
};

struct Uniforms {
    m_model: mat4x4<f32>,
    m_view: mat4x4<f32>,
    m_projection: mat4x4<f32>,
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let col = vec4<f32>(1,0,0,1);
    let m = uniforms.m_model;
    let v = uniforms.m_view;
    let p = uniforms.m_projection;
    var pos = p*v*m*vec4<f32>(input.v_pos.xyz, 1);

    var out: VertexOutput;
    out.position = pos;
    out.frag_color = col;
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.frag_color;
}
