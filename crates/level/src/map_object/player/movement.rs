use super::controls::PlayerAction;
use crate::prelude::{Camera, MapObject, Player};
use bevy::prelude::*;
pub use constants::*;
use game_state::conditions::in_level_state;
use leafwing_input_manager::prelude::*;
use std::f32::consts;

#[allow(unused)]
pub mod constants;

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(
                FixedUpdate,
                (
                    update_side_momentum,
                    update_forward_momentum,
                    update_angle,
                    try_move_player,
                    apply_friction,
                )
                    .chain()
                    .run_if(in_level_state()),
            )
            .add_systems(FixedPostUpdate, update_camera.run_if(in_level_state()));
    }
}

fn update_angle(
    mut turn_held: Local<u32>,
    action_query: Query<&ActionState<PlayerAction>>,
    mut query: Query<&mut MapObject, With<Player>>,
) {
    let action_state = action_query.single();

    update_turn_held(&mut turn_held, action_state);
    let slow_turn = *turn_held < SLOW_TURN_TICS;

    let mut angle_turn = 0.0;

    if slow_turn {
        if action_state.pressed(&PlayerAction::TurnLeft)
            || action_state.pressed(&PlayerAction::FastLeftTurn)
        {
            angle_turn += SLOW_TURN;
        }
        if action_state.pressed(&PlayerAction::TurnRight)
            || action_state.pressed(&PlayerAction::FastRightTurn)
        {
            angle_turn -= SLOW_TURN;
        }
    } else {
        if action_state.pressed(&PlayerAction::FastLeftTurn) {
            angle_turn += FAST_TURN;
        } else if action_state.pressed(&PlayerAction::TurnLeft) {
            angle_turn += TURN;
        }
        if action_state.pressed(&PlayerAction::FastRightTurn) {
            angle_turn -= FAST_TURN;
        } else if action_state.pressed(&PlayerAction::TurnRight) {
            angle_turn -= TURN;
        }
    }

    if angle_turn != 0.0 {
        let mut player = query.single_mut();
        player.dir = Rot2::radians(angle_turn) * player.dir;
        player.dir = player.dir.fast_renormalize();
    }
}

fn update_turn_held(turn_held: &mut u32, action_state: &ActionState<PlayerAction>) {
    if action_state.pressed(&PlayerAction::TurnLeft)
        || action_state.pressed(&PlayerAction::FastLeftTurn)
        || action_state.pressed(&PlayerAction::TurnRight)
        || action_state.pressed(&PlayerAction::FastRightTurn)
    {
        *turn_held += 1;
    } else {
        *turn_held = 0;
    }
}

fn update_forward_momentum(
    action_query: Query<&ActionState<PlayerAction>>,
    mut query: Query<&mut MapObject, With<Player>>,
) {
    let mut player = query.single_mut();
    let action_state = action_query.single();

    let mut forward = 0.0;
    if action_state.pressed(&PlayerAction::RunForward) {
        forward += RUN_FORWARD;
    } else if action_state.pressed(&PlayerAction::MoveForward) {
        forward += MOVE_FORWARD;
    }
    if action_state.pressed(&PlayerAction::RunBackward) {
        forward -= RUN_FORWARD;
    } else if action_state.pressed(&PlayerAction::MoveBackward) {
        forward -= MOVE_FORWARD;
    }
    forward = forward.clamp(-MAX_MOVE, MAX_MOVE);

    // Make sure the Bevy change system works properly by
    // avoiding calling thrust if movement is zero
    if forward != 0.0 {
        let thrust = forward * player.dir;
        player.velocity += thrust;
    }
}

fn update_side_momentum(
    action_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut query: Query<&mut MapObject, With<Player>>,
) {
    let mut player = query.single_mut();
    let action_state = action_query.single();
    let mut side = 0.0;

    if action_state.pressed(&PlayerAction::FastStrafeLeft) {
        side -= FAST_STRAFE;
    } else if action_state.pressed(&PlayerAction::StrafeLeft) {
        side -= STRAFE;
    }
    if action_state.pressed(&PlayerAction::FastStrafeRight) {
        side += FAST_STRAFE;
    } else if action_state.pressed(&PlayerAction::StrafeRight) {
        side += STRAFE;
    }
    side = side.clamp(-MAX_MOVE, MAX_MOVE);

    // Make sure the Bevy change system works properly by
    // avoiding calling thrust if movement is zero
    if side != 0.0 {
        // Rotate 90º clockwise to move side way.
        let dir = Rot2::radians(-consts::FRAC_PI_2) * player.dir;
        let thrust = side * dir;
        player.velocity += thrust;
    }
}

fn try_move_player(mut query: Query<&mut MapObject, With<Player>>) {
    let mut player = query.single_mut();

    if player.velocity == Vec2::ZERO {
        return;
    }
    let mut xmove = player.velocity.x;
    let mut ymove = player.velocity.y;

    loop {
        let ptryx: f32;
        let ptryy: f32;

        if xmove > MAX_MOMENTUM / 2.0 || ymove > MAX_MOMENTUM / 2.0 {
            ptryx = player.pos.x + xmove / 2.0;
            ptryy = player.pos.y + ymove / 2.0;
            xmove /= 2.0;
            ymove /= 2.0;
        } else {
            ptryx = player.pos.x + xmove;
            ptryy = player.pos.y + ymove;
            xmove = 0.0;
            ymove = 0.0;
        }

        player.pos.x = ptryx;
        player.pos.y = ptryy;

        if xmove == 0.0 || ymove == 0.0 {
            break;
        }
    }
}

fn apply_friction(mut query: Query<&mut MapObject, With<Player>>) {
    let mut player = query.single_mut();

    // No need to apply friction if not moving.
    if player.velocity == Vec2::ZERO {
        return;
    }
    // If momentum is within stop range and there is no
    // user input changing momentum (BUG???), then zero it out.
    if player.is_speed_low() && !player.is_changed() {
        player.velocity = Vec2::ZERO;
        return;
    }
    player.velocity *= FRICTION;
}

fn update_camera(
    mut camera_query: Query<&mut Camera, With<Player>>,
    player_query: Query<&MapObject, (Changed<MapObject>, With<Player>)>,
) {
    let Ok(player) = player_query.get_single() else {
        return;
    };
    let mut camera = camera_query.single_mut();
    camera.update(*player);
}
