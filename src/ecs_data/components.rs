use std::time::Duration;
use bitmask_enum::bitmask;
use super::*;

define_marker_components!(
    InGameEntity,

    Player,
    CanJumpSensor,

    Jumpy,
    NotGravityWell,
    AddTimerForReload,

    MenuEntity
);

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
pub struct GravitySensorDirection(pub Vec3);

// For doors, warps, etc.
#[derive(Component, Clone, Debug)]
pub struct Activatable {
    pub requirements: Vec<usize>,
    pub is_active: bool
}

impl Activatable {
    pub fn new(requirements: Vec<usize>) -> Self {
        Self { requirements, is_active: false }
    }
}

impl Default for Activatable {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

#[derive(Component)]
pub struct ButtonActivatable {
    pub initial_position: Vec3
}

// For buttons, levers, timers, etc.
#[derive(Component, Clone, Debug)]
pub struct Activator {
    pub id: usize,
    pub is_active: bool,
    pub variant: ActivatorVariant
}

impl Activator {
    pub fn new(id: usize, variant: ActivatorVariant) -> Self {
        Self { id, variant, is_active: false }
    }
}

#[derive(Clone, Debug)]
pub enum ActivatorVariant {
    Button {
        initial_position: Vec3
    },
    Timer {
        duration: f32,
        seconds_left: f32
    },
    Warp
}

// Describes where the warp will take you
#[derive(Component, Clone, Debug)]
pub enum WarpTo {
    File(String),
    Out
}

#[derive(Component, Default, Debug, Clone)]
pub struct SensorEvents(pub HashSet<(Entity, Entity)>, pub HashSet<(Entity, Entity)>);

impl SensorEvents {
    pub fn new() -> Self {
        SensorEvents(HashSet::new(), HashSet::new())
    }
}

#[derive(Component, Debug)]
pub struct ObjectEvents(pub Vec<HashSet<(Entity, Entity)>>, pub Vec<HashSet<(Entity, Entity)>>);

impl ObjectEvents {
    pub fn new() -> Self {
        ObjectEvents( 
            [(); NUM_SENSOR_CHANNELS].iter().map(|_| HashSet::new()).collect(),
            [(); NUM_SENSOR_CHANNELS].iter().map(|_| HashSet::new()).collect()
        )
    }

    pub fn get(&self, sensor_channel: SensorChannel) -> &HashSet<(Entity, Entity)> {
        &self.0[(sensor_channel.bits() as f32).log2() as usize]
    }

    pub fn get_mut(&mut self, sensor_channel: SensorChannel) -> &mut HashSet<(Entity, Entity)> {
        &mut self.0[(sensor_channel.bits() as f32).log2() as usize]
    }
}

#[derive(Component)]
pub struct SensorEventId(pub usize);

// Num variants in SensorChannel (exclude None)
pub const NUM_SENSOR_CHANNELS: usize = 5;

#[bitmask(u8)]
#[derive(Component, Default)]
pub enum SensorChannel {
    Respawn,
    Warp,
    Activator,
    Gravity,
    CanJump,
    None
}

impl std::fmt::Display for SensorChannel { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self.bits() {
            1  => "Respawn",
            2  => "Warp",
            4  => "Activator",
            8  => "Gravity",
            16 => "CanJump",
            _  => "Combination"
        })
    }
}

#[derive(Component)]
pub struct SoftUnloadedData {
    pub id: usize,
    pub collider_option: Option<Collider>,
    pub rigid_body_option: Option<RigidBody>,
    pub give_lifetime: Option<Duration>
}