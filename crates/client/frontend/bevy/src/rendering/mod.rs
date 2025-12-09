//! Rendering systems for tiles, actors, and other game entities.

mod actors;
mod items;
mod props;
mod tiles;

pub use actors::*;
pub use items::*;
pub use props::*;
pub use tiles::*;

use bevy::prelude::*;

use crate::assets::load_sprite_assets;

/// Plugin for game rendering systems.
pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (load_sprite_assets, setup_camera).chain())
            .add_systems(
                Update,
                (
                    spawn_tiles,
                    spawn_actors,
                    spawn_props,
                    spawn_items,
                    update_actor_positions,
                    update_prop_states,
                    update_item_positions,
                    update_camera_follow,
                )
                    .chain(),
            );
    }
}

fn setup_camera(mut commands: Commands) {
    use crate::components::MainCamera;

    commands.spawn((Camera2d::default(), MainCamera));
}
