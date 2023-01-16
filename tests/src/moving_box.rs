use FrostyEngine::{
    rect::{RectBuilder, RectRenderComponentBuilder},
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
    let mut app = App::default_new(include_str!("assets/triangle_shader.wgsl").into()).await;
    // use the app to get the current scene which should be empty. Empty it just in case
    let scene = app.get_mut_active_scene().dump_all();
    // create a player entity inside the scene
    let rect_builder = RectBuilder{ x: 0, y: 0, width: 10, height: 10 };
    let player_sprite_builder = RectRenderComponentBuilder{};
    let mut player = ecs::Entity::new();
    player.build_component(&rect_builder);
    player.build_component(&player_sprite_builder);
    // add the components
    //player.add_component::<Rect>(player_rect);
    //player.add_component::<RectRenderComponent>(player_sprite);
    // add player to scnee
    scene.add_entity(player);
    // finally, run it
    app.run();
}