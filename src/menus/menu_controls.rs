use bevy::app::AppExit;

use super::*;

pub fn menu_controls(
    state_res: Res<State<AppState>>,
    mut state: ResMut<NextState<AppState>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    match (state_res.0, menu_scheduler.get_menu_type()) {
        (AppState::MenuScreen, MenuType::MainMenu) => {
            if keys.just_pressed(KeyCode::Escape) {
                exit.send(AppExit)
            } else if keys.just_pressed(KeyCode::Return) {
                menu_scheduler.set_menu_type(MenuType::None);
                state.set(AppState::InGame);
            }
        },
        (AppState::OverlayMenu, MenuType::PauseMenu) => {
            if keys.just_pressed(KeyCode::Escape) {
                menu_scheduler.set_menu_type(MenuType::None);
                state.set(AppState::InGame);
            } else if keys.just_pressed(KeyCode::Return) {
                menu_scheduler.set_menu_type(MenuType::None);
                state.set(AppState::InGame);
            } else if keys.just_pressed(KeyCode::R) {
                menu_scheduler.set_menu_type(MenuType::None);
                state.set(AppState::None);
            } else if keys.just_pressed(KeyCode::Q) {
                menu_scheduler.set_menu_type(MenuType::MainMenu);
                state.set(AppState::MenuScreen);
            }
        },
        (AppState::InGame, _) => {
            if keys.just_pressed(KeyCode::Escape) {
                menu_scheduler.set_menu_type(MenuType::PauseMenu);
                state.set(AppState::OverlayMenu);
            }
        },
        (AppState::OverlayMenu, MenuType::DeathScreen) => {
            if keys.just_pressed(KeyCode::Escape) {
                menu_scheduler.set_menu_type(MenuType::MainMenu);
                state.set(AppState::MenuScreen);
            } else if keys.just_pressed(KeyCode::R) {
                state.set(AppState::None);
            }
        },
        _ => {}
    }
}