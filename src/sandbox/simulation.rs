use bevy::prelude::*;

use super::{movement::tick_movement, particle::get_particle, sandbox::*};

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
        Some(p) => {
            if p.updated {
                return;
            }
        }
        None => return,
    }

    if tick_life(x, y, sandbox) {
        return;
    }

    tick_movement(x, y, sandbox);
}

pub fn tick_life(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let replacement = match sandbox.get(x, y).unwrap().particle_death {
        Some(new_p) => new_p.replace_on_death,
        None => return false,
    };

    let health = &mut sandbox.get_mut(x, y).unwrap().health;
    health.amount -= 1;

    if health.amount <= 0 {
        let replacement = replacement.map(get_particle);
        sandbox.set(x, y, replacement);
        return true;
    }

    false
}
