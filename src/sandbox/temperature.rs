use bevy::prelude::Vec2;
use rand::{thread_rng, Rng};

use super::{
    particle::{get_particle, Material, TemperatureChanger, Velocity},
    sandbox::Sandbox,
};

pub fn step_temperature(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    apply_temperature_to_neighbors(x, y, sandbox);

    if step_self(x, y, sandbox) {
        return true;
    }

    step_burning(x, y, sandbox);
    spark_if_ignited(x, y, sandbox);
    false
}

pub fn apply_temperature_to_neighbors(x: usize, y: usize, sandbox: &mut Sandbox) {
    let temp_changer = match sandbox.get(x, y).unwrap().temperature_changer {
        Some(changer) => changer.0,
        None => return,
    };

    for (neighbor_x, neighbor_y) in [
        (x.overflowing_sub(1).0, y),
        (x + 1, y),
        (x, y.overflowing_sub(1).0),
        (x, y + 1),
    ] {
        if let Some(particle) = sandbox.checked_get_mut(neighbor_x, neighbor_y) {
            if let Some(temperature) = &mut particle.temperature {
                if temp_changer.is_positive() && !temperature.heatable {
                    continue;
                }
                if temp_changer.is_negative() && !temperature.coolable {
                    continue;
                }

                temperature.current = temperature.current + temp_changer;
            }
        }
    }
}

fn step_self(x: usize, y: usize, sandbox: &mut Sandbox) -> bool {
    let temperature = match sandbox.get(x, y).unwrap().temperature {
        Some(t) => t,
        None => return false,
    };

    let health = &mut sandbox.get_mut(x, y).unwrap().health;

    if (temperature.critical_on_cool && temperature.current <= 0)
        || (!temperature.critical_on_cool && temperature.current >= 100)
    {
        if temperature.explosion_radius > 0 {
            explode(x, y, temperature.explosion_radius, sandbox);
            return true;
        }

        *health -= 1;

        if *health <= 0 {
            match temperature.replacement_on_critical.material {
                Some(mat) => {
                    if thread_rng().gen_bool(temperature.replacement_on_critical.probability as f64)
                    {
                        sandbox.set(x, y, Some(get_particle(mat)));
                    } else {
                        sandbox.set(x, y, None);
                    }
                }
                None => sandbox.set(x, y, None),
            };
            return true;
        }
    }
    false
}

fn explode(cx: usize, cy: usize, radius: i32, sandbox: &mut Sandbox) {
    let min_x = cx as i32 - radius;
    let max_x = cx as i32 + radius;
    let min_y = cy as i32 - radius;
    let max_y = cy as i32 + radius;

    for x in (min_x - radius)..=(max_x + radius) {
        for y in (min_y - radius)..=(max_y + radius) {
            if sandbox.out_of_bounds_i32(x, y) {
                continue;
            }

            if let Some(particle) = sandbox.get_mut(x as usize, y as usize) {
                if x < min_x || x > max_x || y < min_y || y > max_y {
                    let force = (Vec2::new(x as f32, y as f32) - Vec2::new(cx as f32, cy as f32))
                        .normalize()
                        * 10.0;
                    particle.velocity = Velocity::new(force.x as i32, force.y as i32);
                    continue;
                }

                sandbox.set(x as usize, y as usize, Some(get_particle(Material::Spark)));
            }
        }
    }
}

fn step_burning(x: usize, y: usize, sandbox: &mut Sandbox) {
    let particle = sandbox.get_mut(x, y).unwrap();

    if let Some(burnable) = &mut particle.burnable {
        let mut temp = particle.temperature.unwrap();
        if !burnable.burning && temp.current > burnable.burn_temperature {
            burnable.burning = true;
            particle.temperature_changer = Some(TemperatureChanger(2));
            if particle.health < burnable.burn_ticks {
                particle.health = burnable.burn_ticks;
            }
            particle.color = burnable.burn_color;
        } else if burnable.burning && temp.current < burnable.burn_temperature {
            burnable.burning = false;
            particle.temperature_changer = None;
            particle.color = burnable.cooled_color;
            temp.current = temp.start_temperature;
        }
    }
}

fn spark_if_ignited(x: usize, y: usize, sandbox: &mut Sandbox) {
    let burnable = match sandbox.get_mut(x, y).unwrap().burnable {
        Some(burnable) => {
            if !burnable.burning {
                return;
            }
            burnable
        }
        None => return,
    };

    if !burnable.emit_smoke && burnable.emission.is_none() {
        return;
    }

    for (neighbor_x, neighbor_y) in [
        (x, y + 1),
        (x + 1, y),
        (x.overflowing_sub(1).0, y),
        (x, y.overflowing_sub(1).0),
    ] {
        if sandbox.checked_get(neighbor_x, neighbor_y).is_none()
            && !sandbox.out_of_bounds_usize(neighbor_x, neighbor_y)
        {
            if let Some(material) = burnable.emission {
                let new_p = if thread_rng().gen_ratio(2, 3) {
                    get_particle(material)
                } else if burnable.emit_smoke {
                    get_particle(Material::Smoke)
                } else {
                    continue;
                };
                sandbox.set(neighbor_x, neighbor_y, Some(new_p));
            } else if thread_rng().gen_ratio(2, 3) {
                    sandbox.set(neighbor_x, neighbor_y, Some(get_particle(Material::Smoke)));
            }
        }
    }
}
