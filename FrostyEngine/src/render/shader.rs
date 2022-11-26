use wgpu;
use wgpu::util::DeviceExt;
use winit;

// TODO:
//     - Describe vertices described to it 
//     - Render vertices given to it
//     - Have a shader component that renders an objects > RenderableComponent <

#[derive(Debug)]
pub struct ProtoShader{
    shader_module: wgpu::ShaderModule,
    vertex_entry: String,
    fragment_entry: String,
}

impl ProtoShader{
    pub fn new(module: wgpu::ShaderModule, vertex_entry: &str, fragment_entry: &str) -> Self{
        Self { 
            shader_module: module,
            vertex_entry: vertex_entry.to_string(), 
            fragment_entry: fragment_entry.to_string(),
        }
    }

    // getters

    pub fn get_entrances(&self) -> (&str, &str){
        // (vertex, fragment) since thats the order vertices go to
        (self.vertex_entry.as_str(), self.fragment_entry.as_str())
    }

    pub fn get_module(&self) -> &wgpu::ShaderModule{
        &self.shader_module
    }
}

#[derive(Debug)]
pub struct Shader{
    pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer
}

impl Shader{
    /* 
    pub fn new(pipeline: wgpu::RenderPipeline, device: &wgpu::Device) -> Self{
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: &[],
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        Self { 
            pipeline, 
            vertex_buffer, 
            index_buffer 
        }
    }
    */
}