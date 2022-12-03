use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum InputAction {
    RunLeft,
    RunRight,
    Jump,

    Slash,
    Dash,

    Teleport,
    Liquefy
}

impl InputAction {
    pub fn input_map() -> InputMap<Self> {
        use InputAction::*;

        use InputKind::*;
        use KeyCode as KC;

        InputMap::new([
            (Keyboard(KC::A), RunLeft),
            (Keyboard(KC::D), RunRight),
            (Keyboard(KC::Space), Jump),

            (Keyboard(KC::E), Slash),
            (Keyboard(KC::R), Dash),

            (Keyboard(KC::W), Teleport),
            (Keyboard(KC::S), Liquefy)
        ])
    }

    pub fn input_manager_bundle() -> InputManagerBundle<Self> {
        InputManagerBundle {
            action_state: ActionState::default(),
            input_map: Self::input_map()
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<InputAction>::default());
    }
}
