use bevy::gltf::Gltf;

use super::*;

pub fn setup_loading_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn((NodeBundle {
            style: Style {
                size: Size::width(Val::Percent(100.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            background_color: BACKGROUND_COLOR.into(),
            ..default()
        }, LoadingScreenItem)).with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Loading...",
                TextStyle {
                    font: asset_server.load(FONT_PATH),
                    font_size: 100.0,
                    color: TEXT_COLOR,
                },
            ));
        });
}

pub fn remove_loading_screen(
    mut commands: Commands,
    entities_query: Query<Entity, With<LoadingScreenItem>>
) {
    for entity in entities_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn check_for_load(
    level_stack: Res<LevelStack>,
    gltf_assets: Res<Assets<Gltf>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    mut next_state: ResMut<NextState<AppState>>
) {
    ignore_extract!(
        Some( gltf_handle ) = &level_stack.get_current_level().handle;
        Some( _ ) = gltf_assets.get(&gltf_handle)
    );

    if DEBUG_MENUS { println!("Loading Complete") }

    menu_scheduler.set_menu_type(MenuType::None);
    next_state.set(AppState::None);
}