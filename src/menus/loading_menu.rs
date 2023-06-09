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
        }, MenuEntity)).with_children(|parent| {
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