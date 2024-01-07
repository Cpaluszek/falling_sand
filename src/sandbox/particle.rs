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
    pub updated: bool,
    pub use_gravity: bool,
    pub particle_death: Option<ParticleDeath>,
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
    pub replace_on_death: Option<ParticleTypes>,
    pub probability: Option<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleTypes {
    Sand,
    Water,
    Stone,
    Steam,
    Wood,
}

// Endesga palette
pub const SAND_COLOR: Color = Color::hsl(36.0, 0.99, 0.60);
pub const STONE_COLOR: Color = Color::hsl(220.0, 0.20, 0.44);
pub const WATER_COLOR: Color = Color::hsl(198.0, 1.00, 0.43);
pub const STEAM_COLOR: Color = Color::hsl(217.0, 0.21, 0.63);
pub const WOOD_COLOR: Color = Color::hsl(5.0, 0.34, 0.34);

pub fn get_particle(particle_type: ParticleTypes) -> Particle {
    match particle_type {
        ParticleTypes::Sand => Particle {
            color: add_color_variation(SAND_COLOR, 0.04),
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },
        ParticleTypes::Water => Particle {
            color: add_color_variation(WATER_COLOR, 0.),
            movement_type: MovementType::Liquid,
            density: Density(3),
            use_gravity: true,
            ..default()
        },
        ParticleTypes::Stone => Particle {
            color: add_color_variation(STONE_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },
        ParticleTypes::Steam => {
            let health = thread_rng().gen_range(100..120);
            Particle {
                health: ParticleHealth::new(health, false),
                color: add_color_variation(STEAM_COLOR, 0.),
                movement_type: MovementType::Gas,
                density: Density(0),
                use_gravity: true,
                particle_death: Some(ParticleDeath {
                    replace_on_death: Some(ParticleTypes::Water),
                    probability: Some(0.1),
                }),
                ..default()
            }
        },
        ParticleTypes::Wood => {
            Particle {
                color: add_color_variation(WOOD_COLOR, 0.04),
                movement_type: MovementType::Solid,
                density: Density(u32::MAX),
                use_gravity: true,
                // TODO: add burnable
                ..default()
            }
        }
    }
}

fn add_color_variation(color: Color, range: f32) -> (u8, u8, u8, u8) {
    let mut c: Color = color;
    if range != 0.0 {
        let mut rng = rand::thread_rng();
        c.set_l(c.l() + rng.gen_range(-0.04..=0.04));
    }
    (
        (c.r() * 255.0) as u8,
        (c.g() * 255.) as u8,
        (c.b() * 255.) as u8,
        (c.a() * 255.) as u8,
    )
}
