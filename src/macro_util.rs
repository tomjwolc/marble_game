macro_rules! color {
    ($color:literal) => {
        Color::rgb(($color >> 16) as f32 / 255.0, (($color >> 8) & 0xFF) as f32 / 255.0, ($color & 0xFF) as f32 / 255.0)
    }
}

macro_rules! add_spawning_system {
    ($app:ident, $func:ident) => {
        $app
        .add_system($func  .run_if(AppState::spawn_into).in_schedule(OnExit(AppState::MenuScreen)))
        .add_system($func  .run_if(AppState::spawn_into).in_schedule(OnExit(AppState::None)))
    };
}

macro_rules! add_despawning_system {
    ($app:ident, $func:ident) => {
        $app
        .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::InGame)))
        .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::OverlayMenu)))
    };
}

macro_rules! add_menu_enter_systems {
    ($menu_scheduler:ident, $($menu_type:expr => $system_name:ident),*) => {
        $(
            $menu_scheduler  .get_enter_schedule_mut( $menu_type ).add_system( $system_name );
        )*
    };
}

macro_rules! add_menu_exit_systems {
    ($menu_scheduler:ident, $($menu_type:expr => $system_name:ident),*) => {
        $(
            $menu_scheduler  .get_exit_schedule_mut( $menu_type ).add_system( $system_name );
        )*
    };
}

macro_rules! add_menu_update_systems {
    ($menu_scheduler:ident, $($menu_type:expr => ($( $system_name:ident ),*)),*) => {
        $(
            $menu_scheduler  .get_update_schedule_mut( $menu_type ) 
                $( .add_system( $system_name ) )*;
        )*
    };
}

// same as regular distributive_run_if, except doesn't require implementing Clone
macro_rules! distributive_run_if {
    ($condition:expr => $( $system:ident ),* ) => {
        ($(
            $system .run_if( $condition ),
        )*)
    };
}