use bevy::{prelude::{Color, Component}, ui::{UiRect, Val}};
pub use std::f32::consts::PI;

pub const MAX_ANGLE: f32 = 0.5 * std::f32::consts::PI / 2.0;
pub const SENSITIVITY: f32 = 100.0;
pub const CAMERA_ORBIT_RADIUS: f32 = 4.0;

pub const MARBLE_RADIUS: f32 = 0.3;
pub const MARBLE_COLOR: Color = BUTTON_COLOR;

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

#[derive(Component)]
pub struct Player;