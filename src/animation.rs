use bevy::prelude::*;

// Handles the timing of animations
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

// Handle th indice(or step) of the animation
pub struct AnimationIndices {
    pub first: usize, // Index of the first sprite in the animation.
    pub last: usize,  // Index of the last sprite  in the animation.
}

impl AnimationIndices {
    pub fn new(range: (usize, usize)) -> Self {
        Self {
            first: range.0,
            last: range.1,
        }
    }
}
