use wgpu;

use crate::render::shader;

pub(crate) fn create_render_pipeline(device: &wgpu::Device, layout: &wgpu::PipelineLayout, shader: shader::Shader, config: &wgpu::SurfaceConfiguration, vertex_buffers: &[wgpu::VertexBufferLayout]) -> wgpu::RenderPipeline{
    let (vertex_entry, fragment_entry) = shader.get_entrances();
    device.create_render_pipeline(
        &wgpu::RenderPipelineDescriptor{
            label: Some("Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: shader.get_module(),
                entry_point: vertex_entry,
                buffers: vertex_buffers,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader.get_module(),
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
    )
}