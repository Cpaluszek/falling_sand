use bevy::prelude::*;

use super::{movement::tick_movement, sandbox::*};

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

    tick_movement(x, y, sandbox);
}
