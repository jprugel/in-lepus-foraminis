use bevy::prelude::*;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
use concursus::*;
use physica::*;
use physica::systems::*;

use in_lepus_foraminis::{input::*, player::*, setup, setup_enemy, speed::*};

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
        .add_systems(FixedUpdate, update_collider_center)
        .add_systems(FixedUpdate, detect_collisions)
        .add_systems(FixedUpdate, simulate_rigidbodies)
        .add_systems(FixedUpdate, collision_response)
        .add_systems(Update, draw_collider_gizmos)
        .add_systems(Update, debug_collisions)
        .add_event::<OnCollisionEvent>()
        .register_type::<Speed>()
        .register_type::<MoveDir>()
        .register_type::<Collider>()
        .register_type::<Rigidbody>()
        .run();
}

fn move_player(mut query: Query<(&mut Rigidbody, &MoveDir, &Speed)>) {
    if let Ok((mut rigidbody, move_dir, speed)) = query.get_single_mut() {
        rigidbody.add_force(ForceType::Impulse, move_dir.get() * speed.current);
    }
}
