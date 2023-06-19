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
pub struct TargetLock;

#[derive(Component)]
pub struct Missile {
    pub dest: Vec2,
}
