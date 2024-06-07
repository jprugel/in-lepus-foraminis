use bevy::prelude::*;

#[derive(Debug, Default, Component, Reflect)]
pub struct MoveDir(Vec3);

#[allow(dead_code)]
impl MoveDir {
    pub fn get(&self) -> Vec3 {
        self.0
    }
    fn get_x(&self) -> f32 {
        self.0.x
    }
    fn get_y(&self) -> f32 {
        self.0.y
    }
    fn set_x(&mut self, x: f32) {
        self.0.x = x
    }
    fn set_y(&mut self, y: f32) {
        self.0.y = y
    }
    fn normalize(&mut self) {
        self.0 = self.0.normalize_or_zero()
    }
}

pub fn handle_input(keys: Res<ButtonInput<KeyCode>>, mut query: Query<&mut MoveDir>) {
    if let Ok(mut md) = query.get_single_mut() {
        if keys.pressed(KeyCode::KeyW) {
            md.set_y(1.);
        }
        if keys.just_released(KeyCode::KeyW) {
            md.set_y(0.);
        }
        if keys.pressed(KeyCode::KeyA) {
            md.set_x(-1.);
        }
        if keys.just_released(KeyCode::KeyA) {
            md.set_x(0.);
        }
        if keys.pressed(KeyCode::KeyS) {
            md.set_y(-1.);
        }
        if keys.just_released(KeyCode::KeyS) {
            md.set_y(0.);
        }
        if keys.pressed(KeyCode::KeyD) {
            md.set_x(1.);
        }
        if keys.just_released(KeyCode::KeyD) {
            md.set_x(0.);
        }
        md.normalize();
    }
}
