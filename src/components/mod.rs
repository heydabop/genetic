use specs::{Component, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Agent {
    score: i32,
}

impl Agent {
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

impl Default for Agent {
    fn default() -> Self {
        Self::new()
    }
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

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub heading: f32,
    pub magnitude: f32,
}
