use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;

use in_lepus_foraminis::{
    input::*, 
    player::*, 
    setup, 
    setup_enemy, 
    speed::*, 
    velocity::*,
    physics::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(WorldInspectorPlugin::new())
        .init_gizmo_group::<DefaultGizmoConfigGroup>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_enemy)
        .add_systems(Update, handle_input)
        .add_systems(Update, move_player)
        .add_systems(Update, animation_handler)
        .add_systems(Update, animation_state_handler)
        .add_systems(FixedUpdate, handle_velocity)
        .add_systems(FixedUpdate, update_collider_center)
        .add_systems(FixedUpdate, detect_collisions)
        .add_systems(Update, debug_collisions)
        .add_systems(Update, draw_collider_gizmos)
        .add_event::<OnCollisionEvent>()
        .register_type::<Speed>()
        .register_type::<MoveDir>()
        .register_type::<Collider>()
        .run();
}

fn move_player(mut query: Query<(&mut Velocity, &MoveDir, &Speed)>) {
    if let Ok((mut velocity, move_dir, speed)) = query.get_single_mut() {
        **velocity = move_dir.get() * speed.current as f32
    }
}
