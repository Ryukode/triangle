struct VertexInput {
    @location(0) v_pos: vec3<f32>,
    @location(1) v_normal: vec3<f32>,
    @location(2) v_color: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec4<f32>,
    @location(1) frag_color: vec4<f32>,
};

struct Uniforms {
    ambient: vec4<f32>,
    diffuse: vec4<f32>,
    specular: vec4<f32>,

    m_model: mat4x4<f32>,
    m_view: mat4x4<f32>,
    m_projection: mat4x4<f32>,

    light_dir: vec3<f32>,
    eye_pos: vec3<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    let m = uniforms.m_model;
    let v = uniforms.m_view;
    let p = uniforms.m_projection;
    var pos = p*v*m*vec4<f32>(input.v_pos.xyz, 1);

    var out: VertexOutput;
    out.position = pos;
    out.normal = m * vec4<f32>(input.v_normal, 1);
    out.frag_color = vec4<f32>(input.v_pos.xyz,1);
    return out;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    //diffuse
    var d = max(0, dot(normalize(input.normal.xyz), normalize(-uniforms.light_dir)));
    //specular
    let e = uniforms.eye_pos - input.position.xyz;
    let h = 0.5 * (e - uniforms.light_dir);
    let s = pow(max(0, dot(normalize(input.normal.xyz), normalize(h))), 15);

    let col = uniforms.ambient + d * uniforms.diffuse + s * uniforms.specular;
    return col;
}
