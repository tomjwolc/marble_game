use super::*;
use bitmask_enum::*;

#[derive(Component, Default)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

#[derive(Component, Default)]
pub struct InGameEntity;

// Applied to all objects that can be jumped off of
#[derive(Component, Default)]
pub struct Jumpy;

#[derive(Component, Debug)]
pub struct CameraDir {
    pub horizontal_direction: Vec3,
    pub pitch: f32
}

impl Default for CameraDir {
    fn default() -> Self {
        Self { horizontal_direction: Vec3::Z, pitch: 0.0 }
    }
}

// --------------------------- Gravity
#[derive(Component, Clone)]
pub struct Gravity(pub Vec3, pub GravityType);

#[derive(Clone, Eq, PartialEq)]
pub enum GravityType {
    Constant,       // Constant gravity
    Planets,        // Gravitation pull toward masses
    AntiPlanets     // Gravitation pull away from masses
}

#[derive(Component)]
pub struct GravityChangeSensor(pub Vec3);

#[derive(Component, Default)]
pub struct NotGravityWell;

// For doors, warps, etc.
#[derive(Component, Clone, Debug)]
pub struct Activatable {
    pub requirements: Vec<usize>,
    pub is_active: bool
}

// For buttons, levers, timers, etc.
#[derive(Component, Clone, Debug)]
pub struct Activator(pub usize);

#[derive(Component)]
pub enum ActivatorType {
    Button {
        initial_position: Vec3
    },
    Timer {
        duration: f32,
        seconds_left: f32
    }
}

// Describes where the warp will take you
#[derive(Component, Clone, Debug)]
pub enum WarpTo {
    File(String),
    Out
}

#[bitmask(u8)]
#[derive(Component)]
pub enum SensorChannel {
    Respawn,
    Warp,
    Activator
}