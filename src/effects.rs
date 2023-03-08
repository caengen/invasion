use bevy::{
    prelude::*,
    time::{Time, Timer},
};
use bevy_prototype_lyon::prelude::DrawMode;
use derive_more::From;

use crate::LIGHT;

#[derive(Debug, Component, Default, From)]
pub struct Flick {
    pub switch_timer: Timer,
    pub duration: Timer,
}

#[derive(Debug, Component)]
pub struct TimedRemoval(pub Timer);

#[derive(Debug, Component)]
pub struct Darken(pub Timer);
#[derive(Debug, Component)]
pub struct Shrink(pub Timer);

#[derive(Debug, Component)]
pub struct DelayedVisibility(pub Timer);

pub fn _flick_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Visibility, &mut Flick)>,
    time: Res<Time>,
) {
    for (entity, mut visibility, mut flick) in query.iter_mut() {
        flick.duration.tick(time.delta());
        flick.switch_timer.tick(time.delta());

        if flick.duration.finished() {
            visibility.is_visible = true;
            commands.entity(entity).remove::<Flick>();
        } else if flick.switch_timer.just_finished() {
            visibility.is_visible = !visibility.is_visible;
        }
    }
}

/**
 * Shrink the component by subtracting the scale vector each time the timer finishes
 */
pub fn _shrink_system(mut shrinking: Query<(&mut Transform, &mut Shrink)>, time: Res<Time>) {
    for (mut transform, mut shrink) in shrinking.iter_mut() {
        shrink.0.tick(time.delta());

        if shrink.0.just_finished() && transform.scale.x > 0.0 && transform.scale.y > 0.0 {
            transform.scale *= 1.0 - (0.9 * time.delta_seconds());
        }
    }
}

pub fn _darken_system(
    mut query: Query<(&mut DrawMode, &mut Darken, Without<DelayedVisibility>)>,
    time: Res<Time>,
) {
    for (mut draw_mode, mut darken, _) in query.iter_mut() {
        darken.0.tick(time.delta());

        if darken.0.just_finished() {
            if let DrawMode::Outlined {
                ref mut fill_mode,
                ref mut outline_mode,
            } = *draw_mode
            {
                let pc = 0.70;
                let new_cor = Color::rgb(LIGHT.r() * pc, LIGHT.g() * pc, LIGHT.b() * pc);
                fill_mode.color = new_cor;
                outline_mode.color = new_cor;
            }
        }
    }
}

pub fn _timed_removal_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut TimedRemoval, Without<DelayedVisibility>)>,
) {
    for (entity, mut removal, _) in query.iter_mut() {
        removal.0.tick(time.delta());

        if removal.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn _delayed_visibility_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut DelayedVisibility, &mut Visibility)>,
) {
    for (entity, mut delay, mut visibility) in query.iter_mut() {
        delay.0.tick(time.delta());

        if delay.0.finished() {
            commands.entity(entity).remove::<DelayedVisibility>();
            visibility.is_visible = true;
        }
    }
}
