use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use derive_more::From;

#[derive(From)]
pub enum Scoring {
    Missile = 50,
    Ufo = 1000,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum PhysicsSet {
    Movement,
    CollisionDetection,
}

#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Ufo(pub Vec2);
#[derive(Component)]
pub struct Enemy;
#[derive(Component)]
pub struct HealthBar;
#[derive(Component)]
pub struct Health {
    pub current: u8,
    pub max: u8,
}
#[derive(Debug, Component, From)]
pub struct Vel(pub Vec2);

#[derive(Debug, Component, From)]
pub struct Pos(pub Vec2);

#[derive(Debug, Component, From)]
pub struct Bounding(pub f32);

pub struct AnimationStep;
pub struct FlameRadius;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub timer: Timer,
}
#[derive(Component)]
pub struct Stepper<T, U> {
    pub marker: T,
    pub current: usize,
    pub steps: Vec<U>,
    pub timer: Timer,
}

impl<T, U> Stepper<T, U> {
    pub fn next(&mut self) -> Option<&U> {
        if self.current <= self.steps.len() {
            let step = &self.steps[self.current];
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

#[derive(Component, Clone)]
pub struct Missile {
    pub dest: Vec2,
    pub lock_id: usize,
    pub vel: f32,
}

#[derive(Component)]
pub struct Explosion {
    pub score: usize,
    pub combo: usize,
}
impl Explosion {
    pub fn new() -> Self {
        Self { score: 0, combo: 0 }
    }

    pub fn add_score(&mut self, score: Scoring) {
        self.score += score as usize;
        self.combo += 1;
    }

    pub fn calculated_score(&self) -> usize {
        self.score * self.combo
    }
}

#[derive(Resource)]
pub struct IdCounter(pub usize);
#[derive(Resource)]
pub struct Score(pub usize);

#[derive(Resource)]
pub struct EnemySpawn(pub Timer);

impl IdCounter {
    pub fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

#[derive(Component)]
pub struct Engulfable;

#[derive(Event)]
pub struct MissileArrivalEvent {
    pub entity: Entity,
    pub missile: Missile,
    pub is_enemy: bool,
}

#[derive(Event)]
pub struct MissileExplosionEvent {
    pub entity: Entity,
}
