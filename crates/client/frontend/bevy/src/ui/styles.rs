//! UI styling constants and helpers.

use bevy::prelude::*;

/// Background color for panels.
pub const PANEL_BG: Color = Color::srgba(0.1, 0.1, 0.15, 0.9);

/// Border color for panels.
pub const PANEL_BORDER: Color = Color::srgb(0.3, 0.3, 0.4);

/// Text color for normal text.
pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

/// Text color for health.
pub const HEALTH_COLOR: Color = Color::srgb(0.8, 0.2, 0.2);

/// Text color for mana.
pub const MANA_COLOR: Color = Color::srgb(0.2, 0.4, 0.9);

/// Text color for turn info.
pub const TURN_COLOR: Color = Color::srgb(0.9, 0.8, 0.2);

/// Font size for headers.
pub const HEADER_FONT_SIZE: f32 = 20.0;

/// Font size for normal text.
pub const TEXT_FONT_SIZE: f32 = 16.0;

/// Font size for small text.
pub const SMALL_FONT_SIZE: f32 = 14.0;

/// Create a panel node style.
pub fn panel_style() -> Node {
    Node {
        padding: UiRect::all(Val::Px(10.0)),
        margin: UiRect::all(Val::Px(5.0)),
        flex_direction: FlexDirection::Column,
        row_gap: Val::Px(5.0),
        ..default()
    }
}

/// Create a text style with the given size and color.
pub fn text_style(size: f32, color: Color) -> (TextFont, TextColor) {
    (
        TextFont {
            font_size: size,
            ..default()
        },
        TextColor(color),
    )
}
