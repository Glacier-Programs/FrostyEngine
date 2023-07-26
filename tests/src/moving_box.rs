use FrostyEngine::{
    defaults::rect::{
        RectBuilder,
        RectRenderComponentBuilder, 
        RectRenderComponent
    },
    ecs,
    app::App
};

pub async fn moving_box_example(){
    // first create an app struct
    let mut app = App::default_new(include_str!("assets/default_shader.wgsl").into()).await;
    // create a player entity inside the scene
    let rect_builder = RectBuilder{ x: -1.0, y: 1.0, width: 1.0, height: 1.0 };
    let player_sprite_builder = RectRenderComponentBuilder{ rect_reference: None };

    let mut player = ecs::Entity::new();
    println!("Building Rect");
    player.build_component(&rect_builder);
    println!("\nBuilding Rect Sprite");
    player.build_sprite_component(&player_sprite_builder, app.get_gpu_handles());

    // use the app to get the current scene which should be empty. Empty it just in case
    let scene = app.get_mut_active_scene().dump_all();
    scene.add_entity(player);
    // finally, run it
    app.run::<RectRenderComponent>();
}