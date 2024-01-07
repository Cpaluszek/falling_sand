use bevy::{render::color::Color, utils::default};
use rand::*;
use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub health: ParticleHealth,
    pub velocity: Velocity,
    pub density: Density,
    pub color: (u8, u8, u8, u8),
    pub movement_type: MovementType,
    pub spread_rate: Option<i32>,
    pub updated: bool,
    pub use_gravity: bool,
    pub particle_death: Option<ParticleDeath>,
    pub acidity: Option<Acidity>,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum MovementType {
    Solid,
    #[default]
    Powder,
    Liquid,
    Gas,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}

impl Velocity {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn zero_out_x(&mut self) {
        match self.x.cmp(&0) {
            Ordering::Less => self.x += 1,
            Ordering::Greater => self.x -= 1,
            _ => (),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Density(pub u32);

#[derive(Clone, Copy)]
pub struct ParticleHealth {
    pub amount: i32,
    pub corrodable: bool,
}

impl ParticleHealth {
    fn new(amount: i32, corrodable: bool) -> Self {
        Self { amount, corrodable }
    }
}

impl Default for ParticleHealth {
    fn default() -> Self {
        Self {
            amount: 50,
            corrodable: true,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ParticleDeath {
    pub replace_on_death: Option<Material>,
    pub probability: Option<f32>,
}

#[derive(Clone, Copy)]
pub struct Acidity(pub i32);

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Sand,
    Water,
    Stone,
    Steam,
    Wood,
    Acid,
}

// Endesga palette
pub const SAND_COLOR: Color = Color::hsl(36.0, 0.99, 0.60);
pub const STONE_COLOR: Color = Color::hsl(220.0, 0.20, 0.44);
pub const WATER_COLOR: Color = Color::hsla(198.0, 1.00, 0.43, 0.7);
pub const STEAM_COLOR: Color = Color::hsl(217.0, 0.21, 0.63);
pub const WOOD_COLOR: Color = Color::hsl(5.0, 0.34, 0.34);
pub const ACID_COLOR: Color = Color::hsla(109.0, 0.52, 0.54, 0.7);

pub fn get_particle(material: Material) -> Particle {
    let mut particle = match material {
        Material::Sand => Particle {
            color: format_and_variate_color(SAND_COLOR, 0.04),
            density: Density(u32::MAX),
            use_gravity: true,
            velocity: Velocity::new(4, 0),
            ..default()
        },
        Material::Water => Particle {
            health: ParticleHealth::new(1, false),
            color: format_and_variate_color(WATER_COLOR, 0.),
            movement_type: MovementType::Liquid,
            spread_rate: Some(2),
            density: Density(1),
            use_gravity: true,
            ..default()
        },
        Material::Stone => Particle {
            color: format_and_variate_color(STONE_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },
        Material::Steam => {
            let health = thread_rng().gen_range(100..120);
            Particle {
                health: ParticleHealth::new(health, false),
                color: format_and_variate_color(STEAM_COLOR, 0.),
                movement_type: MovementType::Gas,
                density: Density(0),
                use_gravity: true,
                particle_death: Some(ParticleDeath {
                    replace_on_death: Some(Material::Water),
                    probability: Some(0.1),
                }),
                ..default()
            }
        }
        Material::Wood => {
            Particle {
                color: format_and_variate_color(WOOD_COLOR, 0.04),
                movement_type: MovementType::Solid,
                density: Density(u32::MAX),
                use_gravity: true,
                // TODO: add burnable
                ..default()
            }
        }
        Material::Acid => Particle {
            health: ParticleHealth::new(50, false),
            color: format_and_variate_color(ACID_COLOR, 0.0),
            movement_type: MovementType::Liquid,
            spread_rate: Some(1),
            density: Density(2),
            acidity: Some(Acidity(5)),
            use_gravity: true,
            ..default()
        },
    };

    // Particle spread on spawm
    let random_velocity_x = thread_rng().gen_range(-3..=3);
    particle.velocity = Velocity::new(random_velocity_x, -2);
    particle
}

fn format_and_variate_color(color: Color, range: f32) -> (u8, u8, u8, u8) {
    let mut c: Color = color;
    if range != 0.0 {
        let mut rng = rand::thread_rng();
        c.set_l(c.l() + rng.gen_range(-0.04..=0.04));
    }
    (
        (c.r() * 255.0) as u8,
        (c.g() * 255.0) as u8,
        (c.b() * 255.0) as u8,
        (c.a() * 255.0) as u8,
    )
}
