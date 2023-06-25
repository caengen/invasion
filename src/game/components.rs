use bevy::prelude::*;
use derive_more::From;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PhysicsSet {
    Movement,
    CollisionDetection,
}

#[derive(Debug, Component, From)]
pub struct Vel(pub Vec2);

#[derive(Debug, Component, From)]
pub struct Pos(pub Vec2);

#[derive(Debug, Component, From)]
pub struct Bounding(pub f32);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationSteps {
    pub current: usize,
    pub steps: Vec<usize>,
}

impl AnimationSteps {
    pub fn next(&mut self) -> Option<usize> {
        if self.current <= self.steps.len() {
            let step = self.steps[self.current];
            self.current += 1;
            Some(step)
        } else {
            None
        }
    }
    pub fn is_finished(&self) -> bool {
        self.current == self.steps.len()
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct TargetLock(pub usize);

#[derive(Component)]
pub struct Missile {
    pub dest: Vec2,
    pub lock_id: usize,
    pub vel: f32,
}

#[derive(Component)]
pub struct Explosion;

#[derive(Resource)]
pub struct IdCounter(pub usize);
#[derive(Resource)]
pub struct EnemySpawn(pub Timer);

impl IdCounter {
    pub fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}
