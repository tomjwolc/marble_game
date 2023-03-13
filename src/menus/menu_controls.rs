use super::*;

pub fn menu_controls(
    state_res: Res<State<AppState>>,
    mut state: ResMut<NextState<AppState>>,
    mut menu_scheduler: ResMut<MenuScheduler>,
    keys: Res<Input<KeyCode>>,
) {
    match (state_res.0, menu_scheduler.get_menu_type()) {
        (AppState::InGame, _) => {
            if keys.just_pressed(KeyCode::Escape) {
                menu_scheduler.set_menu_type(MenuType::PauseMenu);
                state.set(AppState::OverlayMenu);
            }
        },
        _ => {}
    }
}