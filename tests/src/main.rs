extern crate pollster;
use std::borrow::Cow;
use FrostyEngine::{
    render::window,
};

mod moving_box;

async fn run() {
    let shader: Cow<'_, str> = include_str!("assets/default_shader.wgsl").into();
    let mut win = window::Window::new_default_size(shader).await;
    win.run();
}

fn main(){
    pollster::block_on( moving_box::moving_box_example() );
}
