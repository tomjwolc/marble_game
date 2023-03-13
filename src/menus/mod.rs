use bevy::app::AppExit;

pub use super::*;

// mod main_menu;
// mod pause_menu;
// mod end_screen;
mod menu_controls;
mod pause_physics;
mod menu_scheduler;
mod stock_menu;

pub use {
    // main_menu::*,
    // pause_menu::*,
    // end_screen::*,
    menu_controls::*,
    pause_physics::*,
    menu_scheduler::*,
    stock_menu::*
};

#[derive(Component, Clone)]
pub struct MainMenuItem;

#[derive(Component, Clone)]
pub struct PauseMenuItem;

#[derive(Component, Clone)]
pub struct DeathScreenItem;

#[derive(Component, Clone)]
pub struct WinScreenItem;

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
                        menu_scheduler.set_menu_type(MenuType::None);
                        state.set(AppState::None);
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

        app
            .insert_resource(menu_scheduler)
            .add_system(transition_menu.run_if(state_changed::<AppState>()))
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