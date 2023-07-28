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

const MAX_RAY_STEPS = 64;
const MAX_VOXEL_DATA = 512;

//@group(0) @binding(0)
//var v2_screen_size : vec2<u32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    
    /*
    var screenPos : vec2<f32> = in.clip_position.xy / v2_screen_size * 2.0 - 1.0;
    var cameraDir : vec3<f32> = vec3(0.0, 0.0, 0.8);
    var cameraPlaneU = vec3(1.0, 0.0, 0.0);
    var cameraPlaneV = vec3(0.0, 1.0, 0.0) * v2_screen_size.y / v2_screen_size.x;
    var rayDir = cameraDir + screenPos.x * cameraPlaneU + screenPos.y * cameraPlaneV;
    var rayPos = vec3(0.0, 0.0, -12.0);

    var mapPos : vec3<i32> = vec3<i32>(floor(rayPos));
    var deltaDist = abs(normalize(rayDir));
    var rayStep : vec3<i32> = vec3<i32>(sign(rayDir));
    var sideStep : vec3<f32> = (sign(rayDir) * (floor(rayPos) - rayPos) + (sign(rayPos) * 0.5) + 0.5) * deltaDist;

    var mask : vec3<bool>;

    */
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

