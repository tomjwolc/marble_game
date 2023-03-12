use super::*;

#[derive(Component)]
pub struct EndScreenEntity;

#[derive(Component)]
pub struct EndReplayButton;

#[derive(Component)]
pub struct EndQuitButton;

pub fn setup_end_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let hover_event = HoverEvent { 
        color: BUTTON_COLOR, 
        hover_color: BUTTON_HOVER_COLOR 
    };

    let button = ButtonBundle {
        style: Style {
            size: Size::new(Val::Auto, Val::Auto),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,

            padding: BUTTON_PADDING,
            margin: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        background_color: hover_event.color.into(),
        ..default()
    };

    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BACKGROUND_COLOR.with_a(0.7).into(),
            ..default()
        }, EndScreenEntity))
        .with_children(|parent| {
            // Spawns Title banner
            parent
                .spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Auto, Val::Auto),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,

                        padding: BUTTON_PADDING,
                        margin: UiRect::bottom(Val::Px(100.0)),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Win/Lose",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 100.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
            
            // Spawns replay button
            parent
                .spawn((button.clone(), hover_event, EndReplayButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Replay",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            color: BUTTON_TEXT_COLOR,
                        },
                    ));
                });
            
            // Spawns quit button
            parent
                .spawn((button.clone(), hover_event, EndQuitButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            color: BUTTON_TEXT_COLOR,
                        },
                    ));
                });
        });
}

pub fn close_end_screen(
    entities_query: Query<Entity, With<EndScreenEntity>>,
    mut commands: Commands
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn replay_button_events(
    mut interaction_query: Query<&Interaction, With<EndReplayButton>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        state.set(AppState::None);
    }
}

pub fn quit_end_button_events(
    mut interaction_query: Query<&Interaction, With<EndQuitButton>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        state.set(AppState::MainMenu);
    }
}