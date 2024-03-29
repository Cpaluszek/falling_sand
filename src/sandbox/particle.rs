use bevy::{render::color::Color, utils::default};
use rand::*;
use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
pub struct Particle {
    pub lifespan: Option<LifeSpan>,
    pub health: i32,
    pub velocity: Velocity,
    pub density: Density,
    pub color: (u8, u8, u8, u8),
    pub movement_type: MovementType,
    pub spread_rate: i32,
    pub use_gravity: bool,
    pub acidity: Option<Acidity>,
    pub corrodable: Option<Corrodable>,
    pub temperature: Option<Temperature>,
    pub temperature_changer: Option<TemperatureChanger>,
    pub burnable: Option<Burnable>,
    pub updated: bool,
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
pub struct LifeSpan {
    pub replacement: ParticleReplacement,
}

impl LifeSpan {
    fn new(replacement: ParticleReplacement) -> Self {
        Self { replacement }
    }
}

#[derive(Clone, Copy)]
pub struct Corrodable(pub i32);

#[derive(Clone, Copy)]
pub struct ParticleReplacement {
    pub material: Option<Material>,
    pub probability: f32,
}

impl ParticleReplacement {
    fn new(material: Option<Material>, probability: f32) -> Self {
        Self {
            material,
            probability,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Acidity(pub i32);

#[derive(Clone, Copy)]
pub struct Temperature {
    pub current: i32,
    pub start_temperature: i32,
    pub coolable: bool,
    pub heatable: bool,
    pub critical_on_cool: bool,
    pub replacement_on_critical: ParticleReplacement,
    pub explosion_radius: i32,
}

impl Temperature {
    pub fn new(
        start_temperature: i32,
        coolable: bool,
        heatable: bool,
        critical_on_cool: bool,
        replacement_on_critical: ParticleReplacement,
        explosion_radius: i32,
    ) -> Self {
        Self {
            current: start_temperature,
            start_temperature,
            coolable,
            heatable,
            critical_on_cool,
            replacement_on_critical,
            explosion_radius,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TemperatureChanger(pub i32);

#[derive(Clone, Copy)]
pub struct Burnable {
    pub burn_temperature: i32,
    pub burn_ticks: i32,
    pub burn_color: (u8, u8, u8, u8),
    pub cooled_color: (u8, u8, u8, u8),
    pub burning: bool,
    pub emission: Option<Material>,
    pub emit_smoke: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
    Sand,
    Water,
    Stone,
    Steam,
    Wood,
    Acid,
    Lava,
    Smoke,
    Spark,
    Igneous,
    Ash,
    Oil,
    Glass,
    Gunpowder,
    Tnt,
}

// https://lospec.com/palette-list/endesga-32
pub const SAND_COLOR: Color = Color::hsl(36.0, 0.99, 0.60);
pub const STONE_COLOR: Color = Color::hsl(220.0, 0.20, 0.44);
pub const WATER_COLOR: Color = Color::hsla(198.0, 1.00, 0.43, 0.7);
pub const STEAM_COLOR: Color = Color::hsl(217.0, 0.21, 0.63);
pub const WOOD_COLOR: Color = Color::hsl(5.0, 0.34, 0.34);
pub const ACID_COLOR: Color = Color::hsla(109.0, 0.52, 0.54, 0.7);
pub const LAVA_COLOR: Color = Color::hsl(357.0, 0.76, 0.56);
pub const SMOKE_COLOR: Color = Color::hsl(216.0, 0.29, 0.81);
pub const IGNEOUS_COLOR: Color = Color::hsl(334.0, 0.23, 0.20);
pub const ASH_COLOR: Color = Color::hsl(220.0, 0.20, 0.44);
pub const OIL_COLOR: Color = Color::hsl(39.0, 0.60, 0.84);
pub const GLASS_COLOR: Color = Color::hsla(184.0, 0.81, 0.57, 0.7);
pub const GUNPOWDER_COLOR: Color = Color::hsl(216.0, 0.29, 0.81);
pub const TNT_COLOR: Color = Color::hsl(354.0, 0.62, 0.39);

pub const SPARK_COLORS: [Color; 3] = [
    Color::hsl(51.0, 0.99, 0.69),
    Color::hsl(36.0, 0.99, 0.60),
    Color::hsl(24.0, 0.93, 0.55),
];

pub const WOOD_BURN_COLORS: [Color; 3] = [
    Color::hsl(354.0, 0.62, 0.39),
    Color::hsl(24.0, 0.93, 0.55),
    Color::hsl(36.0, 0.99, 0.60),
];

pub fn get_particle(material: Material) -> Particle {
    let mut particle = match material {
        Material::Sand => Particle {
            health: 50,
            color: format_and_variate_color(SAND_COLOR, 0.04),
            density: Density(u32::MAX),
            use_gravity: true,
            temperature: Some(Temperature::new(
                50,
                true,
                true,
                false,
                ParticleReplacement::new(Some(Material::Glass), 1.),
                0,
            )),
            corrodable: Some(Corrodable(50)),
            ..default()
        },
        Material::Glass => Particle {
            health: 50,
            color: format_and_variate_color(GLASS_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            ..default()
        },

        Material::Water => Particle {
            health: 50,
            color: format_and_variate_color(WATER_COLOR, 0.005),
            movement_type: MovementType::Liquid,
            spread_rate: 2,
            density: Density(1),
            temperature: Some(Temperature::new(
                30,
                false,
                true,
                false,
                ParticleReplacement::new(Some(Material::Steam), 0.8),
                0,
            )),
            temperature_changer: Some(TemperatureChanger(-5)),
            use_gravity: true,
            ..default()
        },
        Material::Stone => Particle {
            health: 50,
            color: format_and_variate_color(STONE_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            corrodable: Some(Corrodable(50)),
            ..default()
        },
        Material::Steam => {
            let health = thread_rng().gen_range(100..120);
            Particle {
                health,
                lifespan: Some(LifeSpan::new(ParticleReplacement {
                    material: Some(Material::Water),
                    probability: 0.1,
                })),
                color: format_and_variate_color(STEAM_COLOR, 0.04),
                movement_type: MovementType::Gas,
                density: Density(0),
                use_gravity: true,
                ..default()
            }
        }
        Material::Wood => {
            let rand_index = thread_rng().gen_range(0..WOOD_BURN_COLORS.len());
            Particle {
                health: 50,
                color: format_and_variate_color(WOOD_COLOR, 0.04),
                movement_type: MovementType::Solid,
                density: Density(u32::MAX),
                use_gravity: true,
                temperature: Some(Temperature::new(
                    30,
                    true,
                    true,
                    false,
                    ParticleReplacement::new(Some(Material::Ash), 0.3),
                    0,
                )),
                burnable: Some(Burnable {
                    burn_temperature: 100,
                    burn_ticks: 50,
                    burn_color: format_and_variate_color(WOOD_BURN_COLORS[rand_index], 0.04),
                    cooled_color: format_and_variate_color(WOOD_COLOR, 0.04),
                    burning: false,
                    emission: Some(Material::Spark),
                    emit_smoke: true,
                }),
                corrodable: Some(Corrodable(50)),
                ..default()
            }
        }
        Material::Acid => Particle {
            health: 50,
            color: format_and_variate_color(ACID_COLOR, 0.04),
            movement_type: MovementType::Liquid,
            spread_rate: 1,
            density: Density(2),
            acidity: Some(Acidity(5)),
            use_gravity: true,
            ..default()
        },
        Material::Lava => Particle {
            health: 50,
            color: format_and_variate_color(LAVA_COLOR, 0.005),
            movement_type: MovementType::Liquid,
            density: Density(5),
            temperature: Some(Temperature::new(
                50,
                true,
                false,
                true,
                ParticleReplacement::new(Some(Material::Igneous), 0.9),
                0,
            )),
            temperature_changer: Some(TemperatureChanger(5)),
            use_gravity: true,
            ..default()
        },
        Material::Smoke => {
            let health = thread_rng().gen_range(40..55);
            Particle {
                health,
                lifespan: Some(LifeSpan::new(ParticleReplacement::new(None, 1.))),
                color: format_and_variate_color(SMOKE_COLOR, 0.05),
                movement_type: MovementType::Gas,
                density: Density(0),
                use_gravity: true,
                ..default()
            }
        }
        Material::Spark => {
            let health = thread_rng().gen_range(5..10);
            let rand_index = thread_rng().gen_range(0..SPARK_COLORS.len());
            Particle {
                health,
                lifespan: Some(LifeSpan::new(ParticleReplacement::new(None, 1.))),
                color: format_and_variate_color(SPARK_COLORS[rand_index], 0.),
                movement_type: MovementType::Gas,
                density: Density(1),
                temperature_changer: Some(TemperatureChanger(5)),
                use_gravity: true,
                ..default()
            }
        }
        Material::Igneous => Particle {
            health: 50,
            color: format_and_variate_color(IGNEOUS_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            use_gravity: true,
            corrodable: Some(Corrodable(50)),
            ..default()
        },
        Material::Ash => Particle {
            health: 50,
            color: format_and_variate_color(ASH_COLOR, 0.02),
            movement_type: MovementType::Powder,
            density: Density(u32::MAX),
            use_gravity: true,
            corrodable: Some(Corrodable(50)),
            ..default()
        },
        Material::Oil => Particle {
            health: 50,
            color: format_and_variate_color(OIL_COLOR, 0.),
            movement_type: MovementType::Liquid,
            density: Density(2),
            temperature: Some(Temperature::new(
                5,
                false,
                true,
                false,
                ParticleReplacement::new(Some(Material::Spark), 1.),
                0,
            )),
            burnable: Some(Burnable {
                burn_temperature: 42,
                burn_ticks: 15,
                burn_color: (204, 146, 95, 255),
                cooled_color: format_and_variate_color(OIL_COLOR, 0.),
                burning: false,
                emission: None,
                emit_smoke: false,
            }),
            use_gravity: true,
            ..default()
        },
        Material::Gunpowder => Particle {
            health: 50,
            color: format_and_variate_color(GUNPOWDER_COLOR, 0.),
            movement_type: MovementType::Powder,
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(
                1,
                true,
                true,
                false,
                ParticleReplacement::new(None, 1.),
                5,
            )),
            burnable: Some(Burnable {
                burn_temperature: 32,
                burn_ticks: 15,
                burn_color: (204, 146, 95, 255),
                cooled_color: format_and_variate_color(GUNPOWDER_COLOR, 0.),
                burning: false,
                emission: None,
                emit_smoke: true,
            }),
            use_gravity: true,
            corrodable: Some(Corrodable(50)),
            ..default()
        },
        Material::Tnt => Particle {
            health: 50,
            color: format_and_variate_color(TNT_COLOR, 0.),
            movement_type: MovementType::Solid,
            density: Density(u32::MAX),
            temperature: Some(Temperature::new(
                1,
                true,
                true,
                false,
                ParticleReplacement::new(None, 1.),
                15,
            )),
            use_gravity: true,
            corrodable: Some(Corrodable(50)),
            ..default()
        },
    };

    // Particle spread on spawm
    let random_velocity_x: i32;
    let random_velocity_y: i32;
    match (material, particle.movement_type) {
        (Material::Spark, _) => {
            random_velocity_x = thread_rng().gen_range(-6..=6);
            random_velocity_y = thread_rng().gen_range(-6..=6);
        }
        (_, MovementType::Powder | MovementType::Liquid) => {
            random_velocity_x = thread_rng().gen_range(-3..=3);
            random_velocity_y = -2;
        }
        (_, _) => {
            return particle;
        }
    }
    particle.velocity = Velocity::new(random_velocity_x, random_velocity_y);
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
