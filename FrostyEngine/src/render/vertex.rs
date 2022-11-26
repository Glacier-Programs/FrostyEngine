use wgpu;
pub trait VertexTrait{
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    // returns the name of the shader that this vertex 
    // is specified for
    fn get_shader() -> String;
}

pub struct DefaultVertex{
    scene_coords: [f32; 2],
    color: [f32; 3]
}