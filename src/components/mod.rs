use crate::neural::Network;
use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Agent {
    // This network should have an odd number of neurons in its first layer
    // so the agent can have a receptor centered in the direction its heading
    pub inputs: Option<Vec<f32>>,
    pub network: Network,
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Score {
    score: i32,
}

impl Score {
    pub fn new() -> Self {
        Self { score: 0 }
    }

    pub fn inc(&mut self) {
        self.score += 1;
    }

    pub fn score(&self) -> i32 {
        self.score
    }
}

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Force {
    pub rotation: f32,
    pub translation: f32,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Target;

#[derive(Clone, Component, Copy, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn distance(&self, b: &Position) -> f32 {
        ((b.x - self.x).powi(2) + (b.y - self.y).powi(2)).sqrt()
    }
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub heading: f32,
    pub magnitude: f32,
}
