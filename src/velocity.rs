use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Component, Deref, DerefMut, Default, Debug)]
pub struct Velocity(pub Vec3);

pub fn handle_velocity(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += **velocity;
    }
}
