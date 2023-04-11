use super::*;

pub fn menu_controls(
    state_res: Res<State<AppState>>,
    mut state: ResMut<NextState<AppState>>,
    mut menu_state: ResMut<NextState<MenuState>>,
    keys: Res<Input<KeyCode>>,
) {
    match (state_res.0, menu_state.0) {
        (AppState::InGame, _) => {
            if keys.just_pressed(KeyCode::Escape) {
                menu_state.set(MenuState::PauseMenu);
                state.set(AppState::OverlayMenu);
            }
        },
        _ => {}
    }
}