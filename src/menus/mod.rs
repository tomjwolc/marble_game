use bevy::app::AppExit;

pub use super::*;

mod menu_controls;
mod pause_physics;
mod menu_scheduler;
mod stock_menu;
mod loading_menu;
mod menu_components;

pub use {
    menu_controls::*,
    pause_physics::*,
    menu_scheduler::*,
    stock_menu::*,
    loading_menu::*,
    menu_components::*
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        let mut menu_scheduler = MenuScheduler::new();

        // Main Menu
        StockMenu::new(
            "The Amazing Marble Game",
            vec![
                Button::new(
                    "Start",
                    vec![KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        //     Switch to the loading screen, which will then redirect to InGame if
                        // the level has been loaded
                        menu_scheduler.set_menu_type(MenuType::Loading);
                        state.set(AppState::MenuScreen);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |mut exit: EventWriter<AppExit>| {
                        exit.send(AppExit);
                    })
                )
            ],
            MainMenuItem
        ).add_to_app(&mut menu_scheduler, MenuType::MainMenu, app);

        // Pause menu
        StockMenu::new_overlay(
            "Pause",
            vec![
                Button::new(
                    "Restart",
                    vec![KeyCode::R],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::None);
                        state.set(AppState::None);
                    })
                ), Button::new(
                    "Resume",
                    vec![KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::None);
                        state.set(AppState::InGame);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ), Button::new_key_press_only(
                    vec![KeyCode::W],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::WinScreen);
                        state.set(AppState::OverlayMenu);
                    })
                ),
            ],
            PauseMenuItem
        ).add_to_app(&mut menu_scheduler, MenuType::PauseMenu, app);

        // Death menu
        StockMenu::new_overlay(
            "You Died!!",
            vec![
                Button::new(
                    "Replay",
                    vec![KeyCode::R, KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::None);
                        state.set(AppState::None);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ),
            ],
            DeathScreenItem
        ).add_to_app(&mut menu_scheduler, MenuType::DeathScreen, app);

        // Win menu
        StockMenu::new_overlay(
            "You Won!!",
            vec![
                Button::new(
                    "Replay",
                    vec![KeyCode::R, KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::None);
                        state.set(AppState::None);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_scheduler: ResMut<MenuScheduler>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_scheduler.set_menu_type(MenuType::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ),
            ],
            WinScreenItem
        ).add_to_app(&mut menu_scheduler, MenuType::WinScreen, app);

        // Loading menu
        menu_scheduler.get_enter_schedule_mut(MenuType::Loading)
            .add_system(load_glb_asset)
            .add_system(setup_loading_screen);

        menu_scheduler.get_exit_schedule_mut(MenuType::Loading)
            .add_system(remove_loading_screen);

        app
            .insert_resource(menu_scheduler)
            .insert_resource(PrevMenuType(MenuType::None))
            .add_system(transition_menu.run_if(menu_type_changed))
            .add_system(
                button_hover_event.run_if(AppState::in_menu)
            )

            .add_system(menu_controls)
            .add_system(go_back_to_game.in_schedule(OnEnter(AppState::None)))

            .add_system(pause_physics.in_schedule(OnExit(AppState::InGame)))
            .add_system(unpause_physics.in_schedule(OnEnter(AppState::InGame)))
            .add_system(check_for_load.run_if(can_update_menu(MenuType::Loading)))
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