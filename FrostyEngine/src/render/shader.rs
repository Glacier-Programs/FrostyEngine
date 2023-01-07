use wgpu::util::DeviceExt;
use winit;

use super::vertex::{VertexTrait, VertexType};


// TODO:
//     - Describe vertices described to it 
//     - Render vertices given to it
//     - Have a shader component that renders an objects > RenderableComponent <

//
// Shaders are constructed to keep track of different render pipelines
// Meshes are passed into shaders so that the shader can then render them
//

#[derive(Debug)]
pub struct Shader{
    pipeline: wgpu::RenderPipeline,
    shader_module: wgpu::ShaderModule
}

impl Shader{
    pub fn new<V: VertexTrait>(name: &str, shader_module: wgpu::ShaderModule, vertex_entry: &str, fragment_entry: &str, device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self{
        // a basic constructor
        let vertex_type = V::get_type();

        // this is focusing on the RenderPipeline

        let vertex_slice = &[V::desc()];
        let mut vertex_state: wgpu::VertexState; 
        if vertex_type == VertexType::Render{
            vertex_state = wgpu::VertexState {
                module: &shader_module,
                entry_point: vertex_entry,
                buffers: vertex_slice
            };
        }
        else{
            vertex_state = wgpu::VertexState {
                module: &shader_module,
                entry_point: vertex_entry,
                buffers: &[]
            };
        }

        let pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some(name),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }       
        );
        let pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor{
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: vertex_state,
                fragment: Some(wgpu::FragmentState {
                    module: &shader_module,
                    entry_point: fragment_entry,
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // draw counter clockwise 
                    cull_mode: Some(wgpu::Face::Back), // don't draw backs
                    // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                    polygon_mode: wgpu::PolygonMode::Fill,
                    // Requires Features::DEPTH_CLIP_CONTROL
                    unclipped_depth: false,
                    // Requires Features::CONSERVATIVE_RASTERIZATION
                    conservative: false,
                },
                depth_stencil: None, // uneeded since graphics are only 2d
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }
        );

        Self { 
            pipeline, 
            shader_module,
        }
    }

    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline{
        &self.pipeline
    }
}