use FrostyEngine::{
    render::window::Window,
    rect::{Rect, RectRenderComponent},
    ecs,
    app::{App, Runnable}
};

pub async fn moving_box_example(){
    /* previous method */
    /*
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
    let mut player_rect = Rect::new(0, 0, 100, 100);
    let mut player_sprite = RectRenderComponent::new();
    let mut player = ecs::Entity::new();
    // add the components
    player.add_component::<Rect>(player_rect);
    player.add_component::<RectRenderComponent>(player_sprite);
    // add player to scnee
    scene.add_entity(player);
    // finally, run it
    app.run();
}