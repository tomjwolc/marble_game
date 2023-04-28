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
    ($app:ident, $func:ident if $runif:expr) => {
        $app
            .add_system($func  .run_if(AppState::spawn_into.and_then($runif)).in_schedule(OnExit(AppState::MenuScreen)))
            .add_system($func  .run_if(AppState::spawn_into.and_then($runif)).in_schedule(OnExit(AppState::None)))
    };
}

macro_rules! add_despawning_system {
    ($app:ident, $func:ident) => {
        $app
            .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::InGame)))
            .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::OverlayMenu)))
    };
    ($app:ident, $func:ident if $runif:expr) => {
        $app
            .add_system($func  .run_if(AppState::despawn_into.and_then($runif)).in_schedule(OnExit(AppState::InGame)))
            .add_system($func  .run_if(AppState::despawn_into.and_then($runif)).in_schedule(OnExit(AppState::OverlayMenu)))
    };
}

macro_rules! add_to_all_exit_systems {
    ($app:ident, $func:expr) => {
        $app
            .add_system($func  .in_schedule(OnExit(MenuState::MainMenu)))
            .add_system($func  .in_schedule(OnExit(MenuState::Loading)))
            .add_system($func  .in_schedule(OnExit(MenuState::PauseMenu)))
            .add_system($func  .in_schedule(OnExit(MenuState::DeathScreen)))
            .add_system($func  .in_schedule(OnExit(MenuState::WinScreen)))
            .add_system($func  .in_schedule(OnExit(MenuState::Credits)))
    }
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
                println!(
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
macro_rules! extract {
    ($expr:stmt ; $(
        $let_pat:pat = $let_expr:expr
    );*) => {
        $(
            let $let_pat = $let_expr else { 
                $expr
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

macro_rules! pretty_debug {
    ($enum_name:ident :: $variant:ident ( $($field_name:ident),* )) => {
        {
            format!("{}(\n    {}\n  )", stringify!( $variant ), vec![
                $( format!("{}: {:?}", stringify!($field_name).bold(), $field_name), )*
            ].join(",\n    "))
        }
    };
    ($enum_name:ident :: $variant:ident { $($field_name:ident),* }) => {
        {
            format!("{}(\n    {}\n  )", stringify!( $variant ), vec![
                $( format!("{}: {:?}", stringify!($field_name).bold(), $field_name), )*
            ].join(",\n    "))
        }
    }
}

macro_rules! define_marker_components {
    ($($unit_struct:ident),*) => {
        $(
            #[derive(Component)]
            pub struct $unit_struct;
            
        )*
    }
}