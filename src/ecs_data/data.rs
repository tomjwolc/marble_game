use bevy::{prelude::Color, ui::{UiRect, Val}};
pub use std::f32::consts::PI;

pub const DEBUG_MENUS: bool = false;

// Scale for physics
pub const SCALE: f32 = 10.0;

// Gravity
pub const GRAVITATIONAL_CONSTANT: f32 = 0.01;
pub const GRAVITY: f32 = 20.0 * SCALE;

// Camera orbit
pub const MAX_ANGLE: f32 = 0.6 * std::f32::consts::PI / 2.0;
pub const SENSITIVITY: f32 = 100.0;
pub const CAMERA_ORBIT_RADIUS: f32 = SCALE * 6.0;
pub const SURFACE_OFFSET: f32 = 0.1;

// Marble
pub const MARBLE_MASS: f32 = 1.0;
pub const MARBLE_RADIUS: f32 = SCALE * 0.3;
pub const MARBLE_COLOR: Color = color!(0xF49F0A);
pub const MARBLE_SPEED: f32 = SCALE * SCALE * 8.0;
pub const MAX_ANGLE_SPEED: f32 = 30.0;
pub const JUMP_IMPULSE: f32 = SCALE * 5.0;
pub const MARBLE_FRICTION: f32 = SCALE * 0.8;
pub const MARBLE_GRAVITY: f32 = SCALE * 1.0;
pub const MARBLE_RESTITUTION: f32 = 0.1;
pub const ANGULAR_DAMPING: f32 = 0.5;

// Warp
pub const WARP_FRICTION: f32 = 0.5;
pub const WARP_RESTITUTION: f32 = 0.1;

// sensor
pub const SENSOR_THICKNESS: f32 = 1.0;

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