//! UI panel systems for stats and message log.

use bevy::prelude::*;

use crate::components::{HealthText, ManaText, MessageEntry, MessageLogPanel, StatsPanel, TurnText, UiRoot};
use crate::resources::{GameMessageLog, GameViewModel};
use super::styles::*;

/// Setup the main UI layout.
pub fn setup_ui(mut commands: Commands) {
    // Root UI container
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            UiRoot,
        ))
        .with_children(|parent| {
            // Left panel: Stats
            spawn_stats_panel(parent);

            // Spacer (game view area)
            parent.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });

            // Right panel: Message log
            spawn_message_log_panel(parent);
        });
}

fn spawn_stats_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(200.0),
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                align_self: AlignSelf::FlexStart,
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor(PANEL_BORDER),
            StatsPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Player Stats"),
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).0,
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Health
            panel.spawn((
                Text::new("HP: --/--"),
                text_style(TEXT_FONT_SIZE, HEALTH_COLOR).0,
                text_style(TEXT_FONT_SIZE, HEALTH_COLOR).1,
                HealthText,
            ));

            // Mana
            panel.spawn((
                Text::new("MP: --/--"),
                text_style(TEXT_FONT_SIZE, MANA_COLOR).0,
                text_style(TEXT_FONT_SIZE, MANA_COLOR).1,
                ManaText,
            ));

            // Separator
            panel.spawn(Node {
                height: Val::Px(1.0),
                width: Val::Percent(100.0),
                margin: UiRect::vertical(Val::Px(5.0)),
                ..default()
            });

            // Turn info
            panel.spawn((
                Text::new("Turn: --"),
                text_style(TEXT_FONT_SIZE, TURN_COLOR).0,
                text_style(TEXT_FONT_SIZE, TURN_COLOR).1,
                TurnText,
            ));
        });
}

fn spawn_message_log_panel(parent: &mut ChildBuilder) {
    parent
        .spawn((
            Node {
                width: Val::Px(300.0),
                height: Val::Px(200.0),
                padding: UiRect::all(Val::Px(10.0)),
                margin: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(4.0),
                align_self: AlignSelf::FlexEnd,
                overflow: Overflow::clip(),
                ..default()
            },
            BackgroundColor(PANEL_BG),
            BorderColor(PANEL_BORDER),
            MessageLogPanel,
        ))
        .with_children(|panel| {
            // Title
            panel.spawn((
                Text::new("Messages"),
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).0,
                text_style(HEADER_FONT_SIZE, TEXT_COLOR).1,
            ));

            // Message entries will be spawned dynamically
            for i in 0..8 {
                panel.spawn((
                    Text::new(""),
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).0,
                    text_style(SMALL_FONT_SIZE, TEXT_COLOR).1,
                    MessageEntry { index: i },
                ));
            }
        });
}

/// Update the stats panel with current player info.
pub fn update_stats_panel(
    view_model: Option<Res<GameViewModel>>,
    mut health_text: Query<&mut Text, (With<HealthText>, Without<ManaText>, Without<TurnText>)>,
    mut mana_text: Query<&mut Text, (With<ManaText>, Without<HealthText>, Without<TurnText>)>,
    mut turn_text: Query<&mut Text, (With<TurnText>, Without<HealthText>, Without<ManaText>)>,
) {
    let Some(view_model) = view_model else {
        return;
    };

    let player = &view_model.0.player;
    let (hp_current, hp_max) = player.stats.hp();
    let (mp_current, mp_max) = player.stats.mp();

    if let Ok(mut text) = health_text.get_single_mut() {
        **text = format!("HP: {}/{}", hp_current, hp_max);
    }

    if let Ok(mut text) = mana_text.get_single_mut() {
        **text = format!("MP: {}/{}", mp_current, mp_max);
    }

    if let Ok(mut text) = turn_text.get_single_mut() {
        **text = format!("Turn: {}", view_model.0.turn.clock);
    }
}

/// Update the message log with recent messages.
pub fn update_message_log(
    message_log: Option<Res<GameMessageLog>>,
    mut message_entries: Query<(&MessageEntry, &mut Text)>,
) {
    let Some(message_log) = message_log else {
        return;
    };

    let messages: Vec<_> = message_log.0.iter().take(8).collect();

    for (entry, mut text) in message_entries.iter_mut() {
        if let Some(msg) = messages.get(entry.index) {
            **text = msg.text.clone();
        } else {
            **text = String::new();
        }
    }
}
