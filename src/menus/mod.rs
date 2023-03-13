pub use super::*;

mod main_menu;
pub use main_menu::*;

mod overlay_menu;
pub use overlay_menu::*;

mod end_screen;
pub use end_screen::*;

mod menu_controls;
pub use menu_controls::*;

mod pause_physics;
pub use pause_physics::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(setup_overlay.in_schedule(OnEnter(AppState::OverlayMenu)))
            .add_system(setup_end_screen.in_schedule(OnEnter(AppState::EndScreen)))
            
            .add_system(close_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            .add_system(close_overlay.in_schedule(OnExit(AppState::OverlayMenu)))
            .add_system(close_end_screen.in_schedule(OnExit(AppState::EndScreen)))

            .add_systems(( // Main menu systems
                start_button_events,
                quit_button_events
            ).distributive_run_if(AppState::in_main_menu))

            .add_systems(( // Overlay menu systems
                restart_button_events,
                resume_button_events,
                quit_overlay_button_events
            ).distributive_run_if(AppState::in_overlay_menu))

            .add_systems(( // End screen systems
                replay_button_events,
                quit_end_button_events
            ).distributive_run_if(AppState::in_end_screen))


            .add_system(
                button_hover_event.run_if(AppState::in_menu)
            )

            .add_system(menu_controls)
            .add_system(go_back_to_game.in_schedule(OnEnter(AppState::None)))

            .add_system(pause_physics.in_schedule(OnExit(AppState::InGame)))
            .add_system(unpause_physics.in_schedule(OnEnter(AppState::InGame)))
        ;
    }
}

#[derive(Component, Clone, Copy)]
pub struct HoverEvent {
    color: Color, 
    hover_color: Color
}

pub fn button_hover_event(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &HoverEvent)>
) {
    for (
        interaction, 
        mut background_color,
        HoverEvent { color, hover_color }
    ) in interaction_query.iter_mut() {
        if let Interaction::Hovered = interaction {
            *background_color = (*hover_color).into();
        } if let Interaction::None = interaction {
            *background_color = (*color).into();
        }
    }
}

pub fn go_back_to_game(mut state: ResMut<NextState<AppState>>) {
    state.set(AppState::InGame);
}