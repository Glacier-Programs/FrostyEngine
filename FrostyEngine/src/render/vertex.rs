use wgpu;
use bytemuck;

pub trait VertexTrait: bytemuck::Pod + bytemuck::Zeroable {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    // returns the name of the shader that this vertex 
    // is specified for
    fn get_shader() -> String;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DefaultVertex{
    scene_coords: [f32; 2],
    color: [f32; 3]
}

// the specifics of these traits don't matter for some reason
unsafe impl bytemuck::Pod for DefaultVertex {}
unsafe impl bytemuck::Zeroable for DefaultVertex {}

impl VertexTrait for DefaultVertex{
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            // size of DefaultVertex struct
            array_stride: std::mem::size_of::<DefaultVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            // describe the attributes of DefaultVertex struct
            attributes: &[
                // this is > scene_coords <
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                // this is > color <
                wgpu::VertexAttribute {
                    // this is the size of > scene_coords <
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }

    fn get_shader() -> String { "default".into() }
}