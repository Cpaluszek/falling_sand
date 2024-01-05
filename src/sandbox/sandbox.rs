use bevy::prelude::Component;

use super::particle::Particle;

#[derive(Component)]
pub struct Sandbox {
    width: usize,
    height: usize,
    particles: Vec<Option<Particle>>,
}

impl Sandbox {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            particles: vec![None; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Particle> {
        let index = self.to_index(x, y);
        self.particles[index].as_ref()
    }

    pub fn checked_get(&self, x: usize, y: usize) -> Option<&Particle> {
        if self.out_of_bounds_usize(x, y) {
            None
        } else {
            self.get(x, y)
        }
    }
    pub fn checked_get_i32(&self, x: i32, y: i32) -> Option<&Particle> {
        if self.out_of_bounds_i32(x, y) {
            None
        } else {
            self.get(x as usize, y as usize)
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Particle> {
        let index = self.to_index(x, y);
        self.particles[index].as_mut()
    }

    pub fn set(&mut self, x: usize, y: usize, particle: Option<Particle>) {
        let index = self.to_index(x, y);
        if index >= self.particles.len() {
            return;
        }

        self.particles[index] = particle;
    }

    pub fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let index1 = self.to_index(x1, y1);
        let index2 = self.to_index(x2, y2);

        let particle1 = self.get(x1, y1).copied();
        let particle2 = self.get(x2, y2).copied();

        self.particles[index1] = particle2;
        self.particles[index2] = particle1;
    }

    pub fn mark_updated(&mut self, x: usize, y: usize) {
        let index = self.to_index(x, y);
        if let Some(particle) = self.particles[index].as_mut() {
            particle.updated = true;
        }
    }

    pub fn to_index(&self, x: usize, y: usize) -> usize {
        (y * self.width) + x
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    pub fn reset_updated(&mut self) {
        for particle in self.particles.iter_mut().filter_map(|x| x.as_mut()) {
            particle.updated = false;
        }
    }

    pub fn out_of_bounds_i32(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32
    }
    pub fn out_of_bounds_usize(&self, x: usize, y: usize) -> bool {
        x >= self.width || y >= self.height
    }
}
