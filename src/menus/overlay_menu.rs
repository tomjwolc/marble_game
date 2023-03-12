use super::*;

#[derive(Component)]
pub struct OverlayEntity;

#[derive(Component)]
pub struct OverlayRestartButton;

#[derive(Component)]
pub struct OverlayResumeButton;

#[derive(Component)]
pub struct OverlayQuitButton;

pub fn setup_overlay(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        }, OverlayEntity))
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
                        "Pause",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 100.0,
                            color: TEXT_COLOR,
                        },
                    ));
                });
            
            // Spawns start button
            parent
                .spawn((button.clone(), hover_event, OverlayRestartButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Restart",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            color: BUTTON_TEXT_COLOR,
                        },
                    ));
                });
            
            // Spawns resume button
            parent
                .spawn((button.clone(), hover_event, OverlayResumeButton))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Resume",
                        TextStyle {
                            font: asset_server.load(FONT_PATH),
                            font_size: 40.0,
                            color: BUTTON_TEXT_COLOR,
                        },
                    ));
                });

            // Spawns quit button
            parent
                .spawn((button, hover_event, OverlayQuitButton))
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

pub fn close_overlay(
    entities_query: Query<Entity, With<OverlayEntity>>,
    mut commands: Commands
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn restart_button_events(
    mut interaction_query: Query<&Interaction, With<OverlayRestartButton>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        state.set(AppState::None);
    }
}

pub fn resume_button_events(
    mut interaction_query: Query<&Interaction, With<OverlayResumeButton>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        state.set(AppState::InGame);
    }
}

pub fn quit_overlay_button_events(
    mut interaction_query: Query<&Interaction, With<OverlayQuitButton>>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        state.set(AppState::MainMenu);
    }
}