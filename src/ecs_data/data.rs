use bevy::{prelude::Color, ui::{UiRect, Val}};
use bevy_rapier3d::prelude::{Friction, CoefficientCombineRule, Restitution};
pub use std::f32::consts::PI;
use phf::phf_map;

pub const DEBUG_MENUS: bool = false;
pub const DEBUG_GLTF_LOAD: bool = false;
pub const DEBUG_SENSORS: bool = false;

// Scale for physics
pub const SCALE: f32 = 10.0;

// Gravity
pub const GRAVITATIONAL_CONSTANT: f32 = 0.01;
pub const GRAVITY: f32 = 20.0 * SCALE;

// Defaults for all objects
pub const NUM_STACKS: usize = 50;
pub const NUM_SECTORS: usize = 50;
pub const FRICTION_COMBINE_RULE: CoefficientCombineRule = CoefficientCombineRule::Multiply;
pub const RESTITUTION_COMBINE_RULE: CoefficientCombineRule = CoefficientCombineRule::Average;

// Camera orbit
pub const MAX_ANGLE: f32 = 0.6 * std::f32::consts::PI / 2.0;
pub const SENSITIVITY: f32 = 100.0;
pub const CAMERA_ORBIT_RADIUS: f32 = SCALE * 6.0;
pub const SURFACE_OFFSET: f32 = 0.1;

// keypress que
pub const  KEY_QUEUE_LIFESPAN: usize = 10;

// Marble
pub const MARBLE_MASS: f32 = 1.0;
pub const MARBLE_RADIUS: f32 = SCALE * 0.3;
pub const MARBLE_COLOR: Color = color!(0xF49F0A);
pub const MARBLE_SPEED: f32 = SCALE * SCALE * 8.0;
pub const MAX_ANGLE_SPEED: f32 = 30.0;
pub const JUMP_IMPULSE: f32 = SCALE * 5.0;
pub const MARBLE_FRICTION: f32 = 0.8;
pub const MARBLE_GRAVITY: f32 = SCALE * 1.0;
pub const MARBLE_RESTITUTION: f32 = 0.1;
pub const ANGULAR_DAMPING: f32 = 0.5;

// Marble jump sensor
pub const JUMP_SENSOR_OFFSET: f32 = 0.1 * SCALE;
pub const STEEPEST_JUMP_ANGLE: f32 = 45.0; // degrees

// Warp
pub const WARP_FRICTION: f32 = 0.5;
pub const WARP_RESTITUTION: f32 = 0.1;
pub const WARP_SENSOR_HEIGHT: f32 = 0.3 * SCALE;

// Button
pub const BUTTON_RADIUS: f32 = 2.2938 / 2.0;
pub const BUTTON_HEIGHT: f32 = 0.152148;
pub const BUTTON_SENSOR_HEIGHT: f32 = 0.02 * SCALE;
pub const BUTTON_COMPRESS_DEPTH: f32 = 0.2 * SCALE;

// sensor
pub const SENSOR_THICKNESS: f32 = 1.0;

// movable
pub const MOVABLE_FRICTION: Friction = Friction { coefficient: 0.5, combine_rule: FRICTION_COMBINE_RULE };
pub const MOVABLE_RESTITUTION: Restitution = Restitution { coefficient: 0.5, combine_rule: RESTITUTION_COMBINE_RULE };


// In game
pub const AMBIENT_COLOR: Color = color!(0xF5FBEF);
pub const AMBIENT_BRIGHTNESS: f32 = 0.7;

// Menus
pub const OVERLAY_TRANSPARANCY: f32 = 0.5;
pub const BACKGROUND_COLOR: Color = color!(0x1C0B19);
pub const FONT_PATH: &'static str = "fonts/RobotoCondensed-Regular.ttf";
pub const BUTTON_COLOR: Color = color!(0xF5FBEF);
pub const BUTTON_HOVER_COLOR: Color = color!(0xB5BBAF);
pub const TEXT_COLOR: Color = BUTTON_COLOR;
pub const BUTTON_TEXT_COLOR: Color = BACKGROUND_COLOR;
pub const BUTTON_PADDING: UiRect = UiRect {
    top: Val::Px(10.0), bottom: Val::Px(10.0),
    left: Val::Px(20.0), right: Val::Px(20.0)
};

#[derive(Clone, Copy, Debug)]
pub struct MaterialProperties {
    pub restitution: Restitution,
    pub friction: Friction
}

pub const DEFAULT_MATERIAL_PROPERTIES: MaterialProperties = MaterialProperties { 
    restitution: Restitution { coefficient: 0.1, combine_rule: RESTITUTION_COMBINE_RULE },
    friction:    Friction    { coefficient: 2.0, combine_rule: FRICTION_COMBINE_RULE    },
};

pub static MATERIAL_PROPERTIES: phf::Map<&'static str, MaterialProperties> = phf_map! {
    "default_fixed" => DEFAULT_MATERIAL_PROPERTIES,
    "default_movable" => MaterialProperties {
        restitution: Restitution { coefficient: 0.05,  combine_rule: RESTITUTION_COMBINE_RULE    },
        friction:    Friction    { coefficient: 0.3, combine_rule: FRICTION_COMBINE_RULE },
    },
    "ice" => MaterialProperties { 
        restitution: Restitution { coefficient: 0.05,  combine_rule: RESTITUTION_COMBINE_RULE    },
        friction:    Friction    { coefficient: 0.05, combine_rule: FRICTION_COMBINE_RULE },
    }
};

impl MaterialProperties {
    pub fn from(string: Option<String>, is_dynamic: bool) -> Self {
        *match (string, is_dynamic) {
            (Some(material_key), _) if MATERIAL_PROPERTIES.get(material_key.as_str())
            .is_some() => MATERIAL_PROPERTIES
                .get(material_key.as_str())
                .unwrap(),
            (_, true) => MATERIAL_PROPERTIES
                .get("default_movable")
                .unwrap(),
            (_, false) => MATERIAL_PROPERTIES
                .get("default_fixed")
                .unwrap()
        }
    }
}