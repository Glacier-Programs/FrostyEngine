use wgpu;
use bytemuck;

// this exists for allowing special vertex uses in the future
// for now, it allows for the EmptyVertex type which
// can be passed into shaders to allow them to not
// depend on vertices for rendering
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum VertexType{
    Render,
    Empty 
}

pub trait VertexTrait: bytemuck::Pod + bytemuck::Zeroable {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
    fn get_type() -> VertexType;
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

    fn get_type() -> VertexType { VertexType::Render }

    fn get_shader() -> String { "default".into() }
}

// This is a special type of vertex that allows for 
// a shader to not require vertices to be passed into it
// If this vertex is used during a shaders construction,
// the shader will not need vertices for rendering
// This struct should not be instantiated
#[derive(Copy, Clone)]
pub struct EmptyVertex{}

unsafe impl bytemuck::Zeroable for EmptyVertex {}
unsafe impl bytemuck::Pod for EmptyVertex{}

impl VertexTrait for EmptyVertex{
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        // technically speaking, this desc() will never be used,
        // but it will still be created during shader instantiaton
        // so it has to return an actual VertexBufferLayout
        wgpu::VertexBufferLayout {
            // size of EmptyVertex struct
            array_stride: std::mem::size_of::<EmptyVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            // This object has no values
            attributes: &[ ]
        }
    }
    fn get_type() -> VertexType { VertexType::Empty }
    fn get_shader() -> String { "default".into() }
}