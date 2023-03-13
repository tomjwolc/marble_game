pub use super::*;

#[derive(Resource)]
pub struct MenuScheduler {
    menu_type: MenuType,
    prev_menu_type: MenuType,
    enter_schedules: Vec<Schedule>,
    exit_schedules: Vec<Schedule>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MenuType {
    MainMenu = 0,     // OutGame
    LevelSelect = 1,  // OutGame
    PauseMenu = 2,    // PauseGame
    DeathScreen = 3,  // PauseGame
    WinScreen = 4,    // PauseGame
    Credits = 5,      // OutGame
    None = 6          // For prev_menu_type initialization
}

pub const NUM_MENU_TYPES: usize = 7;

impl MenuScheduler {
    pub fn new() -> Self {
        Self {
            menu_type: MenuType::MainMenu,
            prev_menu_type: MenuType::None,
            enter_schedules: vec![0; NUM_MENU_TYPES].iter().map(|_| Schedule::new()).collect(),
            exit_schedules: vec![0; NUM_MENU_TYPES].iter().map(|_| Schedule::new()).collect()
        }
    }

    pub fn set_menu_type(&mut self, menu_type: MenuType) {
        self.prev_menu_type = self.menu_type;
        self.menu_type = menu_type;
    }

    pub fn get_menu_type(&self) -> MenuType {
        self.menu_type
    }

    pub fn get_enter_schedule_mut(&mut self, menu_type: MenuType) -> &mut Schedule {
        &mut self.enter_schedules[menu_type as usize]
    }

    pub fn get_exit_schedule_mut(&mut self, menu_type: MenuType) -> &mut Schedule {
        &mut self.exit_schedules[menu_type as usize]
    }

    pub fn transition_menu(&mut self, world: &mut World) {
        if DEBUG_MENUS {
            println!("Closing: {:?}\nOpening: {:?}\n\n", self.prev_menu_type, self.menu_type);
        }
        self.get_exit_schedule_mut(self.prev_menu_type).run(world);
        self.get_enter_schedule_mut(self.menu_type).run(world);
    }
}

pub fn transition_menu(world: &mut World) {
    let Some(mut menu_scheduler) = world.remove_resource::<MenuScheduler>() else { return };
    menu_scheduler.transition_menu(world);
    world.insert_resource(menu_scheduler);
}

pub fn can_update_menu(menu_type: MenuType) -> impl FnMut(Res<State<AppState>>, Res<MenuScheduler>) -> bool {
    Box::new(move |state: Res<State<AppState>>, menu_scheduler: Res<MenuScheduler>| {
        AppState::in_menu(state) && menu_scheduler.get_menu_type() == menu_type
    })
}
