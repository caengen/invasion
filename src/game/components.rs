use std::time::Duration;

use bevy::{prelude::*, utils::HashSet};
use derive_more::From;

pub const PLAYER_MISSILE_SPEED: f32 = 250.0;
pub const MAX_AMMO: u8 = 30;

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
pub struct Cannon;
#[derive(Component)]
pub struct CannonBase;
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
#[derive(Component)]
pub struct SpawnPoint(pub Vec2);
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
pub struct AnimeRemoveOnFinish;
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

    pub fn current(&self) -> Option<&U> {
        self.steps.get(self.current)
    }

    pub fn is_finished(&self) -> bool {
        self.current == self.steps.len()
    }
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

#[derive(Clone)]
pub struct ChainedMeta {
    pub timer: Timer,
    pub remaining: usize,
}

#[derive(Clone)]
pub enum ExplosionMode {
    Single,
    Chained(ChainedMeta),
}

#[derive(Component)]
pub struct Explosion {
    pub score: usize,
    pub combo: usize,
    pub mode: ExplosionMode,
}
impl Explosion {
    pub fn new(mode: ExplosionMode) -> Self {
        Self {
            score: 0,
            combo: 0,
            mode,
        }
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

impl IdCounter {
    pub fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

#[derive(Component)]
pub struct Engulfable;
#[derive(Component)]
pub struct Explodable;

#[derive(Event)]
pub struct MissileArrivalEvent {
    pub entity: Entity,
    pub missile: Missile,
    pub is_enemy: bool,
}

#[derive(Event)]
pub struct ExplosionEvent {
    pub pos: Vec3,
    pub mode: ExplosionMode,
}

#[derive(Component)]
pub struct Foreground;
#[derive(Component)]
pub struct TextColor;
#[derive(Component)]
pub struct DropBombTimer(pub Timer);
#[derive(Component)]
pub struct MissileReserve(pub u8);
#[derive(Component)]
pub struct City;
#[derive(Component)]
pub struct Destroyed;
