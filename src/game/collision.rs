use bevy::{
    math::{vec2, vec3},
    prelude::*,
};

use super::data::Bounding;

pub fn distance_between(a: &Vec3, b: &Vec3) -> f32 {
    a.distance(*b)
}

pub fn circles_touching(a: &Transform, ar: &Bounding, b: &Transform, br: &Bounding) -> bool {
    distance_between(&a.translation, &b.translation) < (ar.0 + br.0)
}

pub fn distance_to_move(a: &Vec3, ar: f32, b: &Vec3, br: f32) -> f32 {
    ar + br - distance_between(a, b)
}

pub fn rotate_point(fp: Vec2, pt: Vec2, a: f32) -> Vec2 {
    let x = pt.x - fp.x;
    let y = pt.y - fp.y;
    let x_rot = x * a.cos() + y * a.sin();
    let y_rot = y * a.cos() - x * a.sin();

    vec2(fp.x + x_rot, fp.y + y_rot)
}
