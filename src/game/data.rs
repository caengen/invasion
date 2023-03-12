use bevy::prelude::*;
use derive_more::From;

#[derive(Resource)]
pub struct Paused(pub bool);

#[derive(Component)]
pub struct ExampleGameText;

#[derive(Component)]
pub struct PausedText;

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
