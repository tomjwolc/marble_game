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
    ($menu_scheduler:ident: $($menu_type:expr => $system:expr),*) => {
        $(
            $menu_scheduler  .get_enter_schedule_mut( $menu_type ).add_system( $system );
        )*
    };
}

macro_rules! add_menu_exit_systems {
    ($menu_scheduler:ident: $($menu_type:expr => $system:expr),*) => {
        $(
            $menu_scheduler  .get_exit_schedule_mut( $menu_type ).add_system( $system );
        )*
    };
}

#[allow(unused_macros)]
// same as regular distributive_run_if, except doesn't require implementing Clone
macro_rules! distributive_run_if {
    ($condition:expr => $( $system:ident ),* ) => {
        ($(
            $system .run_if( $condition ),
        )*)
    };
}

#[allow(unused_macros)]
macro_rules! on_key_press {
    ($($key:ident),*) => {
        move |keys: Res<Input<KeyCode>>| {
           ( $(keys.just_pressed( KeyCode:: $key ) || )* false) 
        }
    };
}

macro_rules! pass_schedule {
    ($system:expr) => {
        { let mut schedule = Schedule::new(); schedule.add_system( $system ); schedule }
    };
}

macro_rules! panic_extract {
    ($fn_name:ident : $(
        $let_pat:pat = $let_expr:expr
    );*) => {
        $(
            let $let_pat = $let_expr else { 
                panic!(
                    "Could not exectute {} = {} in {}!", 
                    stringify!( $let_pat ),
                    stringify!( $let_expr ), 
                    stringify!( $fn_name )
                ); 
            };
        )*
    }
}

#[allow(unused_macros)]
macro_rules! log_extract {
    ($fn_name:ident : $(
        $let_pat:pat = $let_expr:expr
    );*) => {
        $(
            let $let_pat = $let_expr else { 
                panic!(
                    "Could not exectute {} = {} in {}!", 
                    stringify!( $let_pat ),
                    stringify!( $let_expr ), 
                    stringify!( $fn_name )
                ); 
                return;
            };
        )*
    }
}

#[allow(unused_macros)]
macro_rules! ignore_extract {
    ($(
        $let_pat:pat = $let_expr:expr
    );*) => {
        $(
            let $let_pat = $let_expr else { 
                return;
            };
        )*
    }
}

#[allow(unused_macros)]
macro_rules! bounded {
    (($a:expr) < ($b:expr) < ($c:expr)) => {
        ($a < $b) && ($b < $c)
    };
}