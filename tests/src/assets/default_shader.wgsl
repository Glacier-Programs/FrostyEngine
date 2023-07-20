// Input: >DefaultVertex<s and Camera uniform
// Output: Rendered Screen

// just has lighting, position, and texture values
struct VertexInput{
    scene_coords: vec2<f32>,
    color: vec4<f32>,
    lighting: i32
};

struct VertexOutput {
    // clip position is screen coordinates
    @builtin(position) clip_position: vec4<f32>,
    // a value between 0 and 1. 0 is completely black, 1.0 is completely white, and 0.5 is the texture value
    //@location(0) lighting: f32,
    //@location(1) texture_coords: vec3<f32>
};

// vertex
@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32
) -> VertexOutput{
    var vo: VertexOutput;
    vo.clip_position = vec4<f32>(0.0, 0.0, 1.0, 1.0);
    //vo.lighting = 0.5;
    //vo.texture_coords = vec3<f32>(0.0, 0.0, 0.0);
    return vo;
}

// fragment
@fragment
fn fs_main(frag_in: VertexOutput) -> @location(0) vec4<f32>{
    return vec4<f32>( 1.0, 1.0, 1.0, 1.0 );
}