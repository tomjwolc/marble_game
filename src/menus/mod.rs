use bevy::app::AppExit;

pub use super::*;

mod menu_controls;
mod pause_physics;
mod stock_menu;
mod loading_menu;

pub use {
    menu_controls::*,
    pause_physics::*,
    stock_menu::*,
    loading_menu::*
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // Main Menu
        StockMenu::new(
            "The Amazing Marble Game",
            vec![
                Button::new(
                    "Start",
                    vec![KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        //     Switch to the loading screen, which will then redirect to InGame if
                        // the level has been loaded
                        menu_state.set(MenuState::Loading);
                        state.set(AppState::MenuScreen);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |mut exit: EventWriter<AppExit>| {
                        exit.send(AppExit);
                    })
                )
            ]
        ).add_to_app(MenuState::MainMenu, app);

        // Pause menu
        StockMenu::new_overlay(
            "Pause",
            vec![
                Button::new(
                    "Restart",
                    vec![KeyCode::R],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::None);
                        state.set(AppState::None);
                    })
                ), Button::new(
                    "Resume",
                    vec![KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::None);
                        state.set(AppState::InGame);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ), Button::new_key_press_only(
                    vec![KeyCode::W],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::WinScreen);
                        state.set(AppState::OverlayMenu);
                    })
                ),
            ]
        ).add_to_app(MenuState::PauseMenu, app);

        // Death menu
        StockMenu::new_overlay(
            "You Died!!",
            vec![
                Button::new(
                    "Replay",
                    vec![KeyCode::R, KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::None);
                        state.set(AppState::None);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ),
            ]
        ).add_to_app(MenuState::DeathScreen, app);

        // Win menu
        StockMenu::new_overlay(
            "-- Complete --",
            vec![
                // This is the what causes the current world to unload
                Button::new(
                    "Continue",
                    vec![KeyCode::R, KeyCode::Return],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::Loading);
                        state.set(AppState::MenuScreen);
                    })
                ), Button::new(
                    "Quit",
                    vec![KeyCode::Q, KeyCode::Escape],
                    pass_schedule!(move |
                        mut menu_state: ResMut<NextState<MenuState>>,
                        mut state: ResMut<NextState<AppState>>
                    | {
                        menu_state.set(MenuState::MainMenu);
                        state.set(AppState::MenuScreen);
                    })
                ),
            ]
        ).add_to_app(MenuState::WinScreen, app);

        add_to_all_exit_systems!(app, unload_menu);

        app
            .add_system(
                button_hover_event.run_if(AppState::in_menu)
            )

            .add_systems((
                load_glb_asset,
                setup_loading_screen
            ).in_schedule(OnEnter(MenuState::Loading)))

            .add_system(menu_controls)
            .add_system(go_back_to_game.in_schedule(OnEnter(AppState::None)))

            .add_system(pause_physics.in_schedule(OnExit(AppState::InGame)))
            .add_system(unpause_physics.in_schedule(OnEnter(AppState::InGame)))
            .add_system(try_load_glb_data.in_set(OnUpdate(MenuState::Loading)))
        ;
    }
}

pub fn unload_menu(
    mut commands: Commands,
    menu_entity_query: Query<Entity, With<MenuEntity>>
) {
    for menu_entity in menu_entity_query.into_iter() {
        commands.entity(menu_entity).despawn_recursive();
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