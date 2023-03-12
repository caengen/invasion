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

// source: https://github.com/williamfiset/Algorithms/blob/master/src/main/java/com/williamfiset/algorithms/geometry/CircleCircleIntersectionPoints.js
pub fn circle_impact_position(a: &Transform, b: &Transform, ar: f32, br: f32) -> Option<Vec3> {
    let mut r: f32 = 0.0;
    let mut R: f32 = 0.0;
    let mut cx: f32 = 0.0;
    let mut Cx: f32 = 0.0;
    let mut cy: f32 = 0.0;
    let mut Cy: f32 = 0.0;
    if ar < br {
        r = ar;
        R = br;
        cx = a.translation.x;
        cy = a.translation.y;
        Cx = b.translation.x;
        Cy = b.translation.y;
    } else {
        r = ar;
        R = br;
        Cx = b.translation.x;
        Cy = b.translation.y;
        cx = a.translation.x;
        cy = a.translation.y;
    }

    let d = distance_between(&a.translation, &b.translation);

    if d < f32::EPSILON && (R - r).abs() < f32::EPSILON {
        return None;
    }
    // No intersection (circles centered at the
    // same place with different size)
    else if d < f32::EPSILON {
        return None;
    }

    let dx = cx - Cx;
    let dy = cy - Cy;
    let x = (dx / d) * R + Cx;
    let y = (dy / d) * R + Cy;
    let P = vec2(x, y);

    if (ar - br).abs() - d < f32::EPSILON || (ar - br + d).abs() < f32::EPSILON {
        return Some(vec3(P.x, P.y, 1.0));
    }

    // No intersection. Either the small circle contained within
    // big circle or circles are simply disjoint.
    if (d + r) < R || (R + r < d) {
        return None;
    };

    let C = vec2(Cx, Cy);
    let angle = ((r * r - d * d - R * R) / (-2.0 * d * R)).acos();
    let pt1 = rotate_point(C, P, angle.abs());
    let pt2 = rotate_point(C, P, -angle);
    let avg = vec3((pt1.x + pt2.x) / 2.0, (pt1.y + pt2.y) / 2.0, 1.0);

    return Some(avg);
}
