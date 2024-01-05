use bevy::utils::default;
use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub velocity: Velocity,
    pub density: Density,
    pub color: (u8, u8, u8, u8),
    pub movement_type: MovementType,
    pub updated: bool,
    pub use_gravity: bool,
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

#[derive(Debug, Clone, Copy)]
pub enum ParticleTypes {
    Sand,
    Water,
    Stone,
}

// Peach
pub const SAND_COLOR: (u8, u8, u8, u8) = (250, 179, 135, 255);
pub const WATER_COLOR: (u8, u8, u8, u8) = (137, 180, 250, 255);
pub const STONE_COLOR: (u8, u8, u8, u8) = (166, 173, 200, 255);

pub fn get_particle(particle_type: ParticleTypes) -> Particle {
    match particle_type {
        ParticleTypes::Sand => Particle {
            color: SAND_COLOR,
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },
        ParticleTypes::Water => Particle {
            color: WATER_COLOR, 
            movement_type: MovementType::Liquid,
            density: Density(3),
            use_gravity: true,
            ..default()
        },
        ParticleTypes::Stone => Particle {
            color: STONE_COLOR, 
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },
    }
}
