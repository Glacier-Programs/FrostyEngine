use std::{borrow::Cow};

use wgpu;
use winit;
use hashbrown::HashMap;

use super::vertex::VertexTrait;
use super::shader::{ProtoShader, Shader};
use crate::util::create_render_pipeline;

pub(crate) struct RenderBackend{
    // gpu handlers
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    // shaders
    shader_names: HashMap<String, usize>, // stores name and index in >shaders<
    shaders: Vec<wgpu::RenderPipeline>,
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
        let default_shader = ProtoShader::new(default_shader_mod, "vs_main", "fs_main");
        
        // making the pipeline
        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            }       
        );
        let render_pipeline = create_render_pipeline(
            &device,
            &render_pipeline_layout,
            default_shader, 
            &config, 
            &[]
        );

        let mut shader_names: HashMap<String, usize> = HashMap::new();
        shader_names.insert("main".into(), 0usize);
        let shaders = vec![render_pipeline];

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
        let shader = ProtoShader::new(shader_mod, "vs_main", "fs_main");
    }


    // windowing mthods

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Main Render Encoder"),
        });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(self.fill_color),
                            store: true,
                        },
                    })
                ],
                depth_stencil_attachment: None, // unneeded since its all 2d
            });

            render_pass.set_pipeline( &self.shaders[*self.shader_names.get("main").unwrap()] );
            render_pass.draw(0..3,0..1);
        }
        
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}