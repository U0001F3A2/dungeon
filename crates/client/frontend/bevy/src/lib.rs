//! Bevy-based graphical frontend for the dungeon game.
//!
//! This crate provides a 2D tile-based rendering frontend using the Bevy game engine.
//! It integrates with the runtime via `RuntimeHandle` and presents the game state
//! using the shared `ViewModel` from `client-frontend-core`.

mod app;
mod assets;
mod components;
mod cursor;
mod events;
mod input;
mod provider;
mod rendering;
mod resources;
mod ui;

pub use app::BevyFrontend;
pub use assets::SpriteAssets;
pub use cursor::{CursorPlugin, CursorState, HoverState, HoverTarget};
pub use provider::BevyActionProvider;
