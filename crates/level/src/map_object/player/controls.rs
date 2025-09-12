use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

const KEY_UP: KeyCode = KeyCode::ArrowUp;
const KEY_DOWN: KeyCode = KeyCode::ArrowDown;
const KEY_LEFT: KeyCode = KeyCode::ArrowLeft;
const KEY_RIGHT: KeyCode = KeyCode::ArrowRight;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    MoveForward,
    MoveBackward,
    TurnLeft,
    TurnRight,
    RunForward,
    RunBackward,
    FastLeftTurn,
    FastRightTurn,
    StrafeLeft,
    StrafeRight,
    FastStrafeLeft,
    FastStrafeRight,
}

impl PlayerAction {
    pub fn default_map() -> InputMap<Self> {
        use KeyCode::*;
        use ModifierKey::*;
        use PlayerAction::*;

        InputMap::default()
            .with_multiple([(TurnLeft, KEY_LEFT), (TurnRight, KEY_RIGHT)])
            .with_multiple([
                (FastLeftTurn, ButtonlikeChord::modified(Shift, KEY_LEFT)),
                (FastRightTurn, ButtonlikeChord::modified(Shift, KEY_RIGHT)),
            ])
            .with_one_to_many(
                MoveForward,
                [
                    ButtonlikeChord::from_single(KeyW),
                    ButtonlikeChord::from_single(KEY_UP),
                ],
            )
            .with_one_to_many(
                MoveBackward,
                [
                    ButtonlikeChord::from_single(KeyS),
                    ButtonlikeChord::from_single(KEY_DOWN),
                ],
            )
            .with_one_to_many(
                RunForward,
                [
                    ButtonlikeChord::modified(Shift, KeyW),
                    ButtonlikeChord::modified(Shift, KEY_UP),
                ],
            )
            .with_one_to_many(
                RunBackward,
                [
                    ButtonlikeChord::modified(Shift, KeyS),
                    ButtonlikeChord::modified(Shift, KEY_DOWN),
                ],
            )
            .with_one_to_many(
                StrafeLeft,
                [
                    ButtonlikeChord::from_single(KeyA),
                    ButtonlikeChord::from_single(Comma),
                    ButtonlikeChord::modified(Alt, KEY_LEFT),
                ],
            )
            .with_one_to_many(
                StrafeRight,
                [
                    ButtonlikeChord::from_single(KeyD),
                    ButtonlikeChord::from_single(Period),
                    ButtonlikeChord::modified(Alt, KEY_RIGHT),
                ],
            )
            .with_one_to_many(
                FastStrafeLeft,
                [
                    ButtonlikeChord::modified(Shift, KeyA),
                    ButtonlikeChord::modified(Shift, Comma),
                    ButtonlikeChord::from_single(KEY_LEFT).with_multiple([Alt, Shift]),
                ],
            )
            .with_one_to_many(
                FastStrafeRight,
                [
                    ButtonlikeChord::modified(Shift, KeyD),
                    ButtonlikeChord::modified(Shift, Period),
                    ButtonlikeChord::new([Alt, Shift]).with(KEY_RIGHT),
                ],
            )
    }
}
