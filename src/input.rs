use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum InputAction {
    RunLeft,
    RunRight,
    Crouch,
    Jump,
    Interact,

    Slash,
    Dash,
    Shoot,
}

impl InputAction {
    pub fn input_map() -> InputMap<Self> {
        use InputAction::*;

        use InputKind::*;
        use KeyCode as KC;

        InputMap::new([
            (Keyboard(KC::Left), RunLeft),
            (Keyboard(KC::Right), RunRight),
            (Keyboard(KC::Down), Crouch),
            (Keyboard(KC::Up), Jump),
            (Keyboard(KC::E), Interact),

            (Keyboard(KC::J), Slash),
            (Keyboard(KC::K), Dash),
            (Keyboard(KC::L), Shoot),
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
