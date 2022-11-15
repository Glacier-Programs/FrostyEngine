use FrostyEngine::{
    render::window::Window,
    rect::Rect,
    ecs
};

pub async fn moving_box_example(){
    let win = Window::new_default_size(include_str!("assets/default_shader.wgsl").into()).await;
    let mut player = ecs::Entity::new();
    let rect = Rect::new(0, 0, 100, 300);
    player.add_component(Box::new(rect));
    win.run();
}