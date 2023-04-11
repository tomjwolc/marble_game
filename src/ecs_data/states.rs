use super::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum AppState {
    #[default]
    MenuScreen,
    OverlayMenu,
    InGame,
    None
}

impl AppState {
    pub fn in_menu(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::MenuScreen | Self::OverlayMenu => true,
            _ => false
        }
    }

    pub fn in_game(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::InGame => true,
            _ => false
        }
    }

    pub fn spawn_into(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::InGame | Self::OverlayMenu => true,
            _ => false
        }
    }

    pub fn despawn_into(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::InGame | Self::OverlayMenu => false,
            _ => true
        }
    }
}

// !! If you add to MenuState add the new menu to this macro aswell (except for none) !!
// The macro is used to unload the menus
#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, Copy, States)]
pub enum MenuState {
    #[default]
    MainMenu,
    Loading,
    PauseMenu,
    DeathScreen,
    WinScreen,
    Credits,
    None
}