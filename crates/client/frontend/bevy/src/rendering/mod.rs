//! Rendering systems for tiles, actors, and other game entities.

mod tiles;
mod actors;

pub use tiles::*;
pub use actors::*;

use bevy::prelude::*;

/// Plugin for game rendering systems.
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(
                Update,
                (
                    spawn_tiles,
                    spawn_actors,
                    update_actor_positions,
                    update_camera_follow,
                )
                    .chain(),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    use crate::components::MainCamera;

    commands.spawn((
        Camera2d::default(),
        MainCamera,
    ));
}
