use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct Speed {
    pub minimum: f32,
    pub current: f32,
    pub maximum: f32,
}

impl Speed {
    pub fn new(minimum: f32, current: f32, maximum: f32) -> Self {
        Self {
            minimum,
            current,
            maximum,
        }
    }
}
