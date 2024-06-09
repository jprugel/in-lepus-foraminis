pub mod animation;
pub mod input;
pub mod player;
pub mod speed;
pub mod velocity;
pub mod physics;

use crate::animation::*;
use crate::input::*;
use crate::player::*;
use crate::speed::*;
use crate::velocity::*;
use crate::physics::*;

use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::math::Vec3A;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("textures/sprite_sheet.png");
    let layout = TextureAtlasLayout::from_grid(
        Vec2::new(25., 25.), // Tile Size,
        6,                   // Columns,
        6,                   // Rows,
        Some(Vec2::new(0., 1.)),              // Padding,
        None,                // Offset
    );
    let texture_atlas_layout = texture_atlas_layout.add(layout);

    commands.spawn(Camera2dBundle::default());
    commands.spawn(PlayerBundle {
        speed: Speed::new(0., 2.5, 128.),
        move_dir: MoveDir::default(),
        sprites: SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform::from_scale(Vec3::splat(3.)),
            ..default()
        },
        anim_state: AnimationState::Idle(IdleVariant::Front),
        anim_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        velocity: Velocity::default(),
        face: MostRecentFace { is_front: true },
        collider: Collider::new(Vec3A::default(), Vec3A::new(32., 36., 0.)),
        rigidbody: Rigidbody {
            mass: 5.,
            ..default()
        },
    });
}

pub fn setup_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default().with_scale(Vec3::splat(64.)).with_translation(Vec3::splat(128.)),
            material: materials.add(Color::PURPLE),
            ..default()
        },
        Collider::new(Vec3A::default(), Vec3A::new(32., 32., 0.)),
        Rigidbody::default(),
    ));
}
