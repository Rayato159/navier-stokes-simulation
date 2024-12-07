use bevy::prelude::*;

pub fn scene_setup(mut commands: Commands) {
    commands.spawn((Camera2d, Camera { ..default() }));
    info!("Scene setup complete");
}
