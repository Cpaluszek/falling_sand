use bevy::prelude::*;
use rand::{thread_rng, Rng};

use super::{
    movement::step_movement, particle::get_particle, sandbox::*, temperature::step_temperature,
};

pub fn update_particles(mut sandbox_query: Query<&mut Sandbox>) {
    let mut sandbox = sandbox_query
        .get_single_mut()
        .expect("Sandbox should exists");

    for x in 0..sandbox.width() {
        for y in 0..sandbox.height() {
            step_particle(x, y, &mut sandbox);
        }
    }

    sandbox.reset_updated();
}

pub fn step_particle(x: usize, y: usize, sandbox: &mut Sandbox) {
    match sandbox.get(x, y) {
        Some(p) if p.updated || p.health.amount <= 0 => {
            if p.health.amount <= 0 {
                sandbox.set(x, y, None);
            }
            return;
        }
        None => return,
        _ => {}
    }

    if step_acidity(x, y, sandbox) || step_temperature(x, y, sandbox) || step_health(x, y, sandbox)
    {
        return;
    }

    step_movement(x, y, sandbox);
}

pub fn step_health(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let particle = match sandbox.get(x, y) {
        Some(p) => p,
        None => return false,
    };

    let (replacement, probability) = match &particle.health.replacement {
        Some(new_p) => (new_p.material, new_p.probability),
        None => return false,
    };

    let health = &mut sandbox.get_mut(x, y).unwrap().health;
    health.amount -= 1;

    if health.amount <= 0 {
        if thread_rng().gen_bool(probability.into()) {
            let replacement = replacement.map(get_particle);
            sandbox.set(x, y, replacement);
        }
        return true;
    }
    false
}

pub fn step_acidity(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let acidity = match sandbox.get(x, y).unwrap().acidity {
        Some(a) => a.0,
        None => return false,
    };

    if acidity <= 0 {
        return false;
    }

    let mut acid_ticks = 0;
    for (neighbor_x, neighbor_y) in [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ] {
        if let Some(particle) = sandbox.checked_get_mut(neighbor_x, neighbor_y) {
            if let Some(corrodable) = particle.corrodable.as_mut() {
                corrodable.0 -= acidity;
                acid_ticks += 1;

                if corrodable.0 <= 0 {
                    sandbox.set(neighbor_x, neighbor_y, None);
                }
            }
        }
    }
    let acid_health = &mut sandbox.get_mut(x, y).unwrap().health;
    acid_health.amount -= acid_ticks;

    if acid_health.amount <= 0 {
        sandbox.set(x, y, None);
        return true;
    }

    false
}
