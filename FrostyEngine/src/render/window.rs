use winit;
use wgpu;

use crate::render_backend::RenderBackend;

pub struct Window{
    render_backend: RenderBackend
}