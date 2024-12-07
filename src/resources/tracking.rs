use bevy::{prelude::*, utils::HashMap};
#[derive(Resource)]
pub struct ParticleTracking(pub HashMap<u32, bool>);
