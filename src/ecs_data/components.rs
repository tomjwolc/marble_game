use super::*;
use bitmask_enum::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

// Applied to all objects that can be jumped off of
#[derive(Component)]
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

// Applied to any physics objects that need to stop during pause screen
#[derive(Default, Component)]
pub struct Pausable {
    pub velocity: Velocity,
    pub prev_rigid_body: Option<RigidBody>
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

#[derive(Component)]
pub struct NotGravityWell;

// For doors, warps, etc.
#[derive(Component, Clone, Debug)]
pub struct Activatable {
    pub requirements: Vec<usize>,
    pub is_active: bool
}

// For buttons, levers, timers, etc.
#[derive(Component, Clone, Debug)]
pub struct Activator {
    pub id: usize,
    pub is_active: bool
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
    Button
}

#[derive(Component)]
pub struct RespawnSensor;