use wgpu;

// TODO:
//     - Describe vertices described to it 
//     - Render vertices given to it
//     - Have a shader component that renders an objects > RenderableComponent <

#[derive(Debug)]
pub struct Shader{
    shader_module: wgpu::ShaderModule,
    vertex_entry: String,
    fragment_entry: String,
}

impl Shader{
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