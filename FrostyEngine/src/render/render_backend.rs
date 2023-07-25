use std::{
    borrow::Cow,
    rc::Rc
};

use wgpu;
use winit;
use hashbrown::HashMap;

use super::vertex::DefaultVertex;
use super::shader::Shader;
use super::sprite_component::ReturnsBuffer;
use super::gpu_package::GPUPackage;

/*
 * Render Pipeline explained:
 *      (RenderableComponent) => [VertexTrait, VertexTrait, VertexTrait, ...] = verts
 *      verts => VertexBuffer
 *      shader<vertex> || render_pass
 *      (Vertex_buffer) || render_pass.draw( )
 */

/*
 * This is an object that takes care of rendering and 
 * Dealing with the GPU. It is set for a specific
 * Vertex Type
 */
pub(crate) struct RenderBackend{
    // gpu handlers
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // shaders
    shader_names: HashMap<String, usize>, // stores name and index in >shaders< bc shader can't derive Hash
    shaders: Vec<Shader>,
    // rendering
    fill_color: wgpu::Color
}

impl RenderBackend{
    
    // constructors
    
    pub async fn new<'a>(winit_window: &winit::window::Window, default_shader: Cow<'a, str>) -> Self{
        let size = winit_window.inner_size();
        let gpu_instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe{ gpu_instance.create_surface(&winit_window) };
        let adapter = gpu_instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();
    
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
                label: None,
            },
            None
        ).await.unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
        };
        surface.configure(&device, &config);

        // loading shader
        let default_shader_mod = device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some("default shader"),
                source: wgpu::ShaderSource::Wgsl(default_shader),
            }
        );

        let default_shader = Shader::new::<DefaultVertex>("default", default_shader_mod, "vs_main", "fs_main", &device, &config);
        let mut shader_names: HashMap<String, usize> = HashMap::new();
        shader_names.insert("default".into(), 0usize);
        let shaders = vec![default_shader];

        // non gpu stuff
        let fill_color = wgpu::Color{ r: 0.1, g: 0.2, b: 0.3, a: 1.0};

        Self {
            surface,
            device,
            queue,
            config,
            size,
            shader_names,
            shaders,
            fill_color,
        }
    }

    // methods for loading objects into gpu

    pub fn load_shader<'a>(&mut self, shader_name: &str, shader_location: Cow<'a, str>){
        let shader_mod = self.device.create_shader_module(
            wgpu::ShaderModuleDescriptor {
                label: Some(shader_name),
                source: wgpu::ShaderSource::Wgsl(shader_location),
            }
        );
        //let shader = ProtoShader::new(shader_mod, "vs_main", "fs_main");
    }

    pub fn get_gpu_package(&mut self) -> GPUPackage{
        GPUPackage { 
            device: &mut self.device 
        }
    }

    // windowing methods

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self, components: Vec<Rc<dyn ReturnsBuffer>>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Encoder"),
        });
        // get buffers from components
        let mut vertex_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut index_buffers: Vec<wgpu::Buffer> = Vec::new();
        let mut shader_names: Vec<String> = Vec::new();
        let mut num_indices_in_buffer: Vec<u32> = Vec::new();
        for comp in components{
            let (vert_buffer,index_buffer) = comp.get_buffers(&self.device);
            let shader = comp.get_shader();
            let num_indices = comp.get_num_indices();
            vertex_buffers.push(vert_buffer);
            index_buffers.push(index_buffer);    
            shader_names.push(shader);
            num_indices_in_buffer.push(num_indices);
        }
        // go through each component and render it
        // the vertex buffer, index buffer, and shader name should all line up in the Vecs
        let mut current_rendering = 0usize;
        let load_ops = vec![
            wgpu::LoadOp::Clear(self.fill_color), // first render we want to clear the screenn
            wgpu::LoadOp::Load // everything after, we don't
        ];
        while current_rendering < vertex_buffers.len() as usize{
            // access data from vecs
            let shader_name = shader_names.get(current_rendering).unwrap();
            let num_indices = num_indices_in_buffer.get(current_rendering).unwrap();
            let vertex_buffer = vertex_buffers.get(current_rendering).unwrap();
            let index_buffer = index_buffers.get(current_rendering).unwrap();
            let load_op = (current_rendering != 0usize) as usize;

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: load_ops[load_op],
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None // unneeded since its all 2d
            });

            render_pass.set_pipeline( 
                self.shaders[
                    *self.shader_names.get(
                        shader_name
                    ).unwrap()
                ].get_pipeline() 
            );
            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32); // 1.
            render_pass.draw_indexed(0..*num_indices, 0, 0..1);
            current_rendering += 1;
        }
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}