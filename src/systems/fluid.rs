use bevy::prelude::*;

use crate::{
    components::window::{WINDOW_HEIGHT, WINDOW_WIDTH},
    entities::fluid::{
        FluidParticle, FLUID_DYNAMICS_VISCOSITY, FLUID_PRESSURE_RATE_X, FLUID_VELOCITY_X,
    },
    resources::tracking::ParticleTracking,
};

const FLUID_ASSET_PATH: &str = "fluid.png";

pub fn initial_state(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut particles_tracking: ResMut<ParticleTracking>,
) {
    let fluid_asset = asset_server.load(FLUID_ASSET_PATH);
    let particle_size = 1.;
    let mut id = 1;

    for row in 0..(WINDOW_HEIGHT / 2.) as usize {
        if row % 32 == 0 {
            let y = row as f32 * particle_size;
            let x = -(WINDOW_WIDTH / 2.) * particle_size;

            let bottom_pos = Vec2::new(x, -y);
            let top_pos = Vec2::new(x, y);

            let bottom_particle = FluidParticle::new(id, bottom_pos);
            let top_particle = FluidParticle::new(id + 1, top_pos);

            commands.spawn((
                bottom_particle,
                Sprite {
                    image: fluid_asset.clone(),
                    ..default()
                },
                Transform::from_translation(bottom_pos.extend(1.)),
            ));

            commands.spawn((
                top_particle,
                Sprite {
                    image: fluid_asset.clone(),
                    ..default()
                },
                Transform::from_translation(top_pos.extend(1.)),
            ));

            particles_tracking.0.insert(id, false);
            particles_tracking.0.insert(id + 1, false);

            id += 2;
        }
    }

    info!("Initial state setup complete");
}

pub fn flow(
    time: Res<Time>,
    mut fluid_query: Query<(&mut Transform, &mut FluidParticle)>,
    mut particles_tracking: ResMut<ParticleTracking>,
) {
    for (mut transform, mut fluid) in fluid_query.iter_mut() {
        let position = fluid.position;
        let new_pos_x = velocity_x_calculation(fluid.position.y) * time.delta_secs();

        let pos_x = position.x + new_pos_x;
        let pos_y = position.y;

        transform.translation = Vec3::new(pos_x, pos_y, 1.0);
        fluid.position = Vec2::new(pos_x, pos_y);

        if fluid.position.x > (WINDOW_WIDTH / 2.) {
            particles_tracking.0.entry(fluid.id).and_modify(|e| {
                *e = true;
            });
        }
    }
}

pub fn simulation_repeat(
    mut fluid_query: Query<(&mut Transform, &mut FluidParticle)>,
    mut particles_tracking: ResMut<ParticleTracking>,
) {
    if particles_tracking.0.iter_mut().all(|p| *p.1) {
        particles_tracking.0.iter_mut().for_each(|p| {
            *p.1 = false;
        });

        fluid_query
            .iter_mut()
            .for_each(|(mut transform, mut fluid)| {
                let pos_x = -(WINDOW_WIDTH / 2.);
                let pos_y = fluid.position.y;

                transform.translation = Vec3::new(pos_x, pos_y, 1.0);
                fluid.position = Vec2::new(pos_x, pos_y);
            });
    }
}

fn velocity_x_calculation(pos_y: f32) -> f32 {
    let term_1 = (1. / (2. * FLUID_DYNAMICS_VISCOSITY))
        * FLUID_PRESSURE_RATE_X
        * (pos_y.powf(2.) - (WINDOW_HEIGHT) * pos_y);

    let term_2 = FLUID_VELOCITY_X * (1. - (pos_y / (WINDOW_HEIGHT)));

    term_1 + term_2
}
