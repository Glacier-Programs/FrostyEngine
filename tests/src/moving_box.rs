use FrostyEngine::{
    render::window::Window,
    rect::Rect,
    ecs,
    app::{App, Runnable}
};

pub async fn moving_box_example(){
    /* previous method
    let win = Window::new_default_size(include_str!("assets/default_shader.wgsl").into()).await;
    let mut player = ecs::Entity::new();
    let rect = Rect::new(0, 0, 100, 300);
    player.add_component(Box::new(rect));
    win.run();
    */
    /* new method */
    // first create an app struct
    let mut app = App::default_new(include_str!("assets/default_shader.wgsl").into()).await;
    // use the app to get the current scene which should be empty. Empty it just in case
    let mut scene = app.get_mut_active_scene().dump_all();
    // create a player entity inside the scene
    let player = scene.spawn_entity();
    // finally, run it
    app.run();
}