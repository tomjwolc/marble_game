use bevy::app::AppExit;

use super::*;

pub fn menu_controls(
    state_res: Res<State<AppState>>,
    mut state: ResMut<NextState<AppState>>,
    keys: Res<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>
) {
    match state_res.0 {
        AppState::MainMenu => {
            if keys.just_pressed(KeyCode::Escape) {
                exit.send(AppExit)
            } else if keys.just_pressed(KeyCode::Return){
                state.set(AppState::InGame);
            }
        },
        AppState::OverlayMenu => {
            if keys.just_pressed(KeyCode::Escape) {
                state.set(AppState::InGame);
            } else if keys.just_pressed(KeyCode::Return){
                state.set(AppState::InGame);
            } else if keys.just_pressed(KeyCode::R) {
                state.set(AppState::None);
            } else if keys.just_pressed(KeyCode::Q) {
                state.set(AppState::MainMenu);
            }
        },
        AppState::InGame => {
            if keys.just_pressed(KeyCode::Escape) {
                state.set(AppState::OverlayMenu);
            }
        },
        AppState::EndScreen => {
            if keys.just_pressed(KeyCode::Escape) {
                state.set(AppState::MainMenu);
            } else if keys.just_pressed(KeyCode::R) {
                state.set(AppState::None);
            }
        },
        _ => {}
    }
}