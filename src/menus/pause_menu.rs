use super::*;

#[derive(Component)]
pub struct PauseMenuEntity;

#[derive(Component)]
pub struct PauseMenuRestartButton;

#[derive(Component)]
pub struct PauseMenuResumeButton;

#[derive(Component)]
pub struct PauseMenuQuitButton;

pub fn setup_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        }, PauseMenuEntity))
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
                .spawn((button.clone(), hover_event, PauseMenuRestartButton))
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
                .spawn((button.clone(), hover_event, PauseMenuResumeButton))
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
                .spawn((button, hover_event, PauseMenuQuitButton))
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

pub fn close_pause_menu(
    entities_query: Query<Entity, With<PauseMenuEntity>>,
    mut commands: Commands
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn restart_button_events(
    mut interaction_query: Query<&Interaction, With<PauseMenuRestartButton>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        menu_scheduler.set_menu_type(MenuType::None);
        state.set(AppState::None);
    }
}

pub fn resume_button_events(
    mut interaction_query: Query<&Interaction, With<PauseMenuResumeButton>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        menu_scheduler.set_menu_type(MenuType::None);
        state.set(AppState::InGame);
    }
}

pub fn quit_pause_menu_button_events(
    mut interaction_query: Query<&Interaction, With<PauseMenuQuitButton>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut state: ResMut<NextState<AppState>>
) {
    if let Ok(Interaction::Clicked) = interaction_query.get_single_mut() {
        menu_scheduler.set_menu_type(MenuType::MainMenu);
        state.set(AppState::MenuScreen);
    }
}