use bevy::prelude::*;
use rand::*;

use crate::sandbox::particle::*;
use crate::utils::*;

use super::sandbox::Sandbox;

#[derive(Default)]
struct StepData {
    new_x: i32,
    new_y: i32,
    moved: bool,
    other_particle: Option<Particle>,
    other_x: i32,
    other_y: i32,
    swap: bool,
}

pub fn step_movement(x: usize, y: usize, sandbox: &mut Sandbox) {
    apply_gravity(x, y, sandbox);

    let step_data = get_step_data(x as i32, y as i32, sandbox);

    let (new_x, new_y) = (step_data.new_x as usize, step_data.new_y as usize);

    if step_data.swap {
        let current_particle = sandbox.get(x, y).unwrap();
        let (other_x, other_y) = (step_data.other_x as usize, step_data.other_y as usize);

        sandbox.set(new_x, new_y, Some(*current_particle));

        if new_x != x && new_y != y {
            sandbox.set(x, y, None);
        }

        sandbox.swap(new_x, new_y, other_x, other_y);
        sandbox.mark_updated(other_x, other_y);
        return;
    }

    if !step_data.moved {
        sandbox.get_mut(x, y).unwrap().velocity = Velocity::new(0, 0);
        return;
    }
    sandbox.swap(x, y, new_x, new_y);
    sandbox.mark_updated(new_x, new_y);
}

pub fn apply_gravity(x: usize, y: usize, sandbox: &mut Sandbox) {
    let particle = sandbox.get_mut(x, y).unwrap();

    if !particle.use_gravity {
        return;
    }

    match particle.movement_type {
        MovementType::Powder | MovementType::Liquid => particle.velocity.y -= 1,
        MovementType::Gas => particle.velocity.y += 1,
        MovementType::Solid => return,
    }
    particle.velocity.zero_out_x();
}

fn get_step_data(x: i32, y: i32, sandbox: &Sandbox) -> StepData {
    let particle = sandbox.get(x as usize, y as usize).unwrap();

    let rotation_type_amount = match particle.movement_type {
        MovementType::Powder => 3,
        MovementType::Liquid | MovementType::Gas => 5,
        MovementType::Solid => return StepData::default(),
    };

    let clockwise_priority = thread_rng().gen_bool(0.5);
    let movement_rotations = match clockwise_priority {
        true => vec![0, 1, 2, 3, 4],
        false => vec![0, 2, 1, 4, 3],
    };

    let valid_rotations = movement_rotations.iter().take(rotation_type_amount);
    let spread_rate = particle.spread_rate;
    for &i in valid_rotations {
        let mut step_data = line_with_rotation(
            x,
            y,
            particle.velocity.x,
            particle.velocity.y,
            spread_rate,
            sandbox,
            i,
        );

        if step_data.moved {
            return step_data;
        }

        if let Some(entity) = step_data.other_particle {
            if particle.density.0 > entity.density.0 {
                step_data.swap = true;
                return step_data;
            }
        }
    }

    StepData::default()
}

fn line_with_rotation(
    start_x: i32,
    start_y: i32,
    w: i32,
    h: i32,
    spread_rate: i32,
    matrix: &Sandbox,
    rotate_type: u32,
) -> StepData {
    let velocity = match rotate_type {
        0 => (w, h - spread_rate),
        1 => {
            let mut v = rotate_45_clockwise(w, h);
            v.0 -= spread_rate;
            v.1 -= spread_rate;
            v
        }
        2 => {
            let mut v = rotate_45_counterclockwise(w, h);
            v.0 += spread_rate;
            v.1 -= spread_rate;
            v
        }
        3 => {
            let mut v = rotate_90_clockwise_normalized(w, h);
            v.0 -= spread_rate;
            v
        }
        4 => {
            let mut v = rotate_90_counterclockwise_normalized(w, h);
            v.0 += spread_rate;
            v
        }
        _ => panic!("{} is not a rotation type. Should be 0-4.", rotate_type),
    };

    line(
        start_x,
        start_y,
        start_x + velocity.0,
        start_y + velocity.1,
        matrix,
    )
}
fn line(mut x1: i32, mut y1: i32, x2: i32, y2: i32, sandbox: &Sandbox) -> StepData {
    if x1 == x2 && y1 == y2 {
        return StepData::default();
    }

    let w = x2 - x1;
    let h = y2 - y1;
    let dx1 = if w < 0 { -1 } else { 1 };
    let dy1 = if h < 0 { -1 } else { 1 };
    let mut dx2 = 0;
    let mut dy2 = 0;
    if w < 0 {
        dx2 = -1;
    } else if w > 0 {
        dx2 = 1;
    }
    let mut longest = w.abs();
    let mut shortest = h.abs();
    if !(longest > shortest) {
        longest = h.abs();
        shortest = w.abs();
        if h < 0 {
            dy2 = -1;
        } else if h > 0 {
            dy2 = 1;
        }
        dx2 = 0;
    }

    let mut past_x = x1;
    let mut past_y = y1;
    let mut numerator = longest >> 1;
    for i in 0..=longest {
        let entity_at_position = sandbox.checked_get_i32(x1, y1);
        let out_of_bounds = sandbox.out_of_bounds_i32(x1, y1);
        if i >= 1 && (entity_at_position.is_some() || out_of_bounds) {
            return StepData {
                new_x: past_x,
                new_y: past_y,
                moved: i > 1,
                other_particle: entity_at_position.copied(),
                other_x: x1,
                other_y: y1,
                ..default()
            };
        }

        past_x = x1;
        past_y = y1;

        numerator += shortest;
        if !(numerator < longest) {
            numerator -= longest;
            x1 += dx1;
            y1 += dy1;
        } else {
            x1 += dx2;
            y1 += dy2;
        }
    }

    StepData {
        new_x: x2,
        new_y: y2,
        moved: true,
        ..default()
    }
}
