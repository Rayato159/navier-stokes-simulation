use bevy::{log::LogPlugin, prelude::*, utils::hashbrown::HashMap, window::WindowResolution};
use navier_stokes_simulation::{
    components::{
        colors::BACKGROUND_COLOR,
        window::{WINDOW_HEIGHT, WINDOW_WIDTH},
    },
    resources::tracking::ParticleTracking,
    states::game_state::{self, GameState},
    systems::{fluid, scene},
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(LogPlugin {
                level: bevy::log::Level::INFO,
                filter: "wgpu=warn,bevy_ecs=info".to_string(),
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Navier-Stokes Fluid Simulation".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(ParticleTracking(HashMap::new()))
        .insert_state(GameState::default())
        .add_systems(Startup, (scene::scene_setup, fluid::initial_state))
        .add_systems(Update, fluid::flow.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            fluid::simulation_repeat.run_if(in_state(GameState::Playing)),
        )
        .add_systems(Update, game_state::toggle_pause)
        .run();
}
