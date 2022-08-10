struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
     var out: VertexOutput;
     let x = -1.0 + f32((in_vertex_index & 1u) << 2u);
     let y = -1.0 + f32((in_vertex_index & 2u) << 1u);
     out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
     return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}