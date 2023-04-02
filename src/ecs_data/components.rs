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

#[derive(Component, Default)]
pub struct NotGravityWell;

#[derive(Component)]
pub struct GravitySensorDirection(pub Vec3);

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

#[derive(Component, Default, Debug, Clone)]
pub struct SensorEvents {
    pub ongoing_events: HashSet<usize>
}

#[derive(Component)]
pub struct ObjectEvents {
    pub ongoing_events: Vec<HashSet<usize>>
}

impl ObjectEvents {
    pub fn new() -> Self {
        ObjectEvents { 
            ongoing_events: [(); NUM_SENSOR_CHANNELS].iter().map(|_| HashSet::new()).collect()
        }
    }

    pub fn get(&self, sensor_channel: SensorChannel) -> &HashSet<usize> {
        &self.ongoing_events[(sensor_channel.bits() as f32).log2() as usize]
    }

    pub fn get_mut(&mut self, sensor_channel: SensorChannel) -> &mut HashSet<usize> {
        &mut self.ongoing_events[(sensor_channel.bits() as f32).log2() as usize]
    }
}

#[derive(Component)]
pub struct SensorEventId(pub usize);

pub const NUM_SENSOR_CHANNELS: usize = 4;

#[bitmask(u8)]
#[derive(Component, Default)]
pub enum SensorChannel {
    Respawn,
    Warp,
    Activator,
    Gravity
} // when adding to this increment the size of the triggeredChannels array