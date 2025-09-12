use bevy::math::Vec2;

pub(super) const MAX_MOVE: f32 = RUN_FORWARD;
pub const MAX_MOMENTUM: f32 = 30.0;
pub(super) const FRICTION: f32 = 0.90625;
pub const STOP_SPEED: Vec2 = Vec2::splat(0.0625);

pub(super) const MOVE_FORWARD: f32 = 0.78125;
pub(super) const RUN_FORWARD: f32 = 1.5625;

pub(super) const STRAFE: f32 = 0.75;
pub(super) const FAST_STRAFE: f32 = 1.25;

pub(super) const SLOW_TURN_TICS: u32 = 6;
pub(super) const SLOW_TURN: f32 = TURN / 2.0;

/// Equal to 2π * (640/2<sup>16</sup>).
pub(super) const TURN: f32 = 0.06135923;
pub(super) const FAST_TURN: f32 = 2.0 * TURN;
