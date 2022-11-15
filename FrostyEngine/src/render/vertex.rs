use wgpu;
pub trait VertexTrait{
    fn desc() -> wgpu::VertexFormat;
}

pub struct DefaultVertex{
    scene_coords: [f32; 2],
    color: [f32; 3]
}