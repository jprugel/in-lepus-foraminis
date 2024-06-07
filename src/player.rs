use bevy::prelude::*;
use std::cmp::*;

use crate::animation::*;
use crate::input::*;
use crate::speed::*;
use crate::velocity::*;
use crate::physics::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub move_dir: MoveDir,
    pub speed: Speed,
    pub sprites: SpriteSheetBundle,
    pub anim_state: AnimationState,
    pub anim_timer: AnimationTimer,
    pub velocity: Velocity,
    pub face: MostRecentFace,
    pub rb: Collider,
}

#[derive(Component)]
pub struct MostRecentFace {
    pub is_front: bool,
}

#[derive(Component, PartialEq, Debug)]
pub enum AnimationState {
    Idle(IdleVariant),
    Running(Direction),
}

#[derive(Debug, PartialEq)]
pub enum IdleVariant {
    Front,
    Back,
}

#[derive(Debug, PartialEq)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest
}

const IDLE_FRONT:          (usize, usize) = (00, 03);
const IDLE_BACK:           (usize, usize) = (06, 09);
const RUNNING_FRONT_RIGHT: (usize, usize) = (12, 17);
const RUNNING_BACK_RIGHT:  (usize, usize) = (18, 23);
const RUNNING_FRONT:       (usize, usize) = (24, 29);
const RUNNING_BACK:        (usize, usize) = (30, 35);

impl AnimationState {
    pub fn get_indices(&self) -> AnimationIndices {
        match self {
            AnimationState::Idle(IdleVariant::Front)      => AnimationIndices::new(IDLE_FRONT),
            AnimationState::Idle(IdleVariant::Back)       => AnimationIndices::new(IDLE_BACK),
            AnimationState::Running(Direction::North)     => AnimationIndices::new(RUNNING_BACK), // backwards towards camera
            AnimationState::Running(Direction::NorthEast) => AnimationIndices::new(RUNNING_BACK_RIGHT), // Right away from camera
            AnimationState::Running(Direction::East)      => AnimationIndices::new(RUNNING_FRONT_RIGHT), // Right towards camera
            AnimationState::Running(Direction::SouthEast) => AnimationIndices::new(RUNNING_FRONT_RIGHT), // Right towards camera
            AnimationState::Running(Direction::South)     => AnimationIndices::new(RUNNING_FRONT), // Towards camera
            AnimationState::Running(Direction::SouthWest) => AnimationIndices::new(RUNNING_FRONT_RIGHT), // Right towards camera flipped
            AnimationState::Running(Direction::West)      => AnimationIndices::new(RUNNING_FRONT_RIGHT),
            AnimationState::Running(Direction::NorthWest) => AnimationIndices::new(RUNNING_BACK_RIGHT), // Left
        }
    }
}

pub fn animation_handler(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &AnimationState, &mut TextureAtlas)>,
) {
    if let Ok((mut anim_timer, anim_state, mut ta)) = query.get_single_mut() {
        anim_timer.tick(time.delta());
        if anim_timer.just_finished() {
            if !((anim_state.get_indices().first..anim_state.get_indices().last).contains(&ta.index)) {
                ta.index = anim_state.get_indices().first;
            }
            if ta.index >= anim_state.get_indices().last {
                ta.index = anim_state.get_indices().first;
            } else {
                ta.index += 1;
            }
        }
    }
}

pub fn animation_state_handler(mut query: Query<(&Velocity, &mut AnimationState, &mut Transform, &mut MostRecentFace)>) {
    if let Ok((velocity, mut anim_state, mut transform, mut face)) = query.get_single_mut() {
        match velocity.length() {
            0. => *anim_state = match face.is_front {
                true => AnimationState::Idle(IdleVariant::Front),
                false => AnimationState::Idle(IdleVariant::Back),
            },
            _  => {
                if velocity.x == 0. && velocity.y > 0. {
                    face.is_front = false;
                    transform.scale.x = 3.;
                    *anim_state = AnimationState::Running(Direction::North);
                }
                if velocity.x > 0. && velocity.y > 0. {
                    face.is_front = false;
                    transform.scale.x = 3.;
                    *anim_state = AnimationState::Running(Direction::NorthEast);
                }
                if velocity.x > 0. && velocity.y == 0. {
                    face.is_front = true;
                    transform.scale.x = 3.;
                    *anim_state = AnimationState::Running(Direction::East);
                }
                if velocity.x > 0. && velocity.y < 0. {
                    face.is_front = true;
                    transform.scale.x = 3.;
                    *anim_state = AnimationState::Running(Direction::SouthEast);
                }
                if velocity.x == 0. && velocity.y < 0. {
                    face.is_front = true;
                    transform.scale.x = 3.;
                    *anim_state = AnimationState::Running(Direction::South);
                }
                if velocity.x < 0. && velocity.y < 0. {
                    face.is_front = true;
                    transform.scale.x = -3.;
                    *anim_state = AnimationState::Running(Direction::SouthWest);
                }
                if velocity.x < 0. && velocity.y == 0. {
                    face.is_front = true;
                    transform.scale.x = -3.;
                    *anim_state = AnimationState::Running(Direction::West);
                }
                if velocity.x < 0. && velocity.y > 0. {
                    face.is_front = false;
                    transform.scale.x = -3.;
                    *anim_state = AnimationState::Running(Direction::NorthWest);
                }
            }       
        }
    }
}
