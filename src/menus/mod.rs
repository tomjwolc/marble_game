pub use super::*;

mod main_menu;
mod pause_menu;
mod end_screen;
mod menu_controls;
mod pause_physics;
mod menu_scheduler;

pub use {
    main_menu::*,
    pause_menu::*,
    end_screen::*,
    menu_controls::*,
    pause_physics::*,
    menu_scheduler::*
};

pub fn debug_state_and_menu_type(
    state: Res<State<AppState>>,
    menu_scheduler: Res<MenuScheduler>,
) {
    // println!("{:?}, {:?}", state, menu_scheduler.get_menu_type());
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        let mut menu_scheduler = MenuScheduler::new();


        add_menu_enter_systems!(menu_scheduler,
            MenuType::MainMenu => setup_main_menu,
            MenuType::PauseMenu => setup_pause_menu,
            MenuType::DeathScreen => setup_end_screen
        );

        add_menu_exit_systems!(menu_scheduler,
            MenuType::MainMenu => close_main_menu,
            MenuType::PauseMenu => close_pause_menu,
            MenuType::DeathScreen => close_end_screen
        );

        add_menu_update_systems!(menu_scheduler,
            MenuType::MainMenu => (
                start_button_events,
                quit_button_events
            ),
            MenuType::PauseMenu => (
                restart_button_events,
                resume_button_events,
                quit_pause_menu_button_events
            ),
            MenuType::DeathScreen => (
                replay_button_events,
                quit_end_button_events
            )
        );

        app
            .insert_resource(menu_scheduler)
            .add_system(debug_state_and_menu_type)
            .add_system(transition_menu.run_if(state_changed::<AppState>()))
            // .add_system(setup_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // .add_system(setup_overlay.in_schedule(OnEnter(AppState::OverlayMenu)))
            // .add_system(setup_end_screen.in_schedule(OnEnter(AppState::EndScreen)))
            
            // .add_system(close_main_menu.in_schedule(OnExit(AppState::MainMenu)))
            // .add_system(close_overlay.in_schedule(OnExit(AppState::OverlayMenu)))
            // .add_system(close_end_screen.in_schedule(OnExit(AppState::EndScreen)))

            .add_systems(distributive_run_if!(
                can_update_menu(MenuType::MainMenu) => 
                start_button_events,
                quit_button_events
            ))

            .add_systems(distributive_run_if!(
                can_update_menu(MenuType::PauseMenu) => 
                restart_button_events,
                resume_button_events,
                quit_pause_menu_button_events
            ))

            .add_systems(distributive_run_if!(
                can_update_menu(MenuType::DeathScreen) => 
                replay_button_events,
                quit_end_button_events
            ))

            // .add_systems(( // Main menu systems
            //     start_button_events,
            //     quit_button_events
            // ).distributive_run_if(can_update_menu(MenuType::MainMenu)))

            // .add_systems(( // Overlay menu systems
            //     restart_button_events,
            //     resume_button_events,
            //     quit_overlay_button_events
            // ).distributive_run_if(AppState::in_overlay_menu))

            // .add_systems(( // End screen systems
            //     replay_button_events,
            //     quit_end_button_events
            // ).distributive_run_if(AppState::in_end_screen))


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