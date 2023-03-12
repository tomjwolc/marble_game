macro_rules! color {
    ($color:literal) => {
        Color::rgb(($color >> 16) as f32 / 255.0, (($color >> 8) & 0xFF) as f32 / 255.0, ($color & 0xFF) as f32 / 255.0)
    }
}

macro_rules! add_spawning_system {
    ($app:ident, $func:ident) => {
        $app
        .add_system($func  .run_if(AppState::spawn_into).in_schedule(OnExit(AppState::MainMenu)))
        .add_system($func  .run_if(AppState::spawn_into).in_schedule(OnExit(AppState::None)))
    };
}

macro_rules! add_despawning_system {
    ($app:ident, $func:ident) => {
        $app
        .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::InGame)))
        .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::OverlayMenu)))
        .add_system($func  .run_if(AppState::despawn_into).in_schedule(OnExit(AppState::EndScreen)))
    };
}