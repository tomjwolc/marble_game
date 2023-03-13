use super::*;

#[derive(Default, Debug, Hash, PartialEq, Eq, Clone, States)]
pub enum AppState {
    #[default]
    MainMenu,
    OverlayMenu,
    EndScreen,
    InGame,
    None
}

impl AppState {
    pub fn in_main_menu(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::MainMenu => true,
            _ => false
        }
    }

    pub fn in_menu(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::MainMenu | Self::OverlayMenu | Self::EndScreen => true,
            _ => false
        }
    }

    pub fn in_overlay_menu(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::OverlayMenu => true,
            _ => false
        }
    }

    pub fn in_end_screen(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::EndScreen => true,
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
            Self::InGame | Self::OverlayMenu | Self::EndScreen => true,
            _ => false
        }
    }

    pub fn despawn_into(state: Res<State<AppState>>) -> bool {
        match state.0 {
            Self::InGame | Self::OverlayMenu | Self::EndScreen => false,
            _ => true
        }
    }
}