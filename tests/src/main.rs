extern crate pollster;
use FrostyEngine::{
    render::window
};

async fn run() {
    let mut win = window::Window::new(800u32, 600u32).await;
}

fn main(){
    pollster::block_on( run() );
}
