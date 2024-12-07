use bevy::prelude::*;

pub const FLUID_DYNAMICS_VISCOSITY: f32 = 1001600.0; // Pa·s
pub const FLUID_PRESSURE_RATE_X: f32 = 20.; // Pa / m
pub const FLUID_VELOCITY_X: f32 = 100.; // m/s

// This entity represents a fluid particle in the simulation 🌊.
#[derive(Debug, Clone, Component)]
pub struct FluidParticle {
    pub id: u32,
    pub position: Vec2, // m
}

impl FluidParticle {
    pub fn new(id: u32, position: Vec2) -> Self {
        Self { id, position }
    }
}
