extern crate pollster;
use FrostyEngine::{
    render::window
};

async fn run() {
    let mut win = window::Window::new_default_size().await;
    win.run();
}

fn main(){
    pollster::block_on( run() );
}
