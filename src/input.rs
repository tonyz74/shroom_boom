use std::collections::HashMap;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;
use bevy_common_assets::toml::TomlAssetPlugin;
use crate::state::GameState;

#[derive(serde::Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "413be529-bfeb-41b3-9db0-4b8b380a2c46"]
struct Config {
    pub controls: HashMap<String, String>
}

#[derive(Resource, Clone)]
struct ConfigHandle(Handle<Config>);



#[derive(Resource, Copy, Clone, Debug)]
pub struct PlayerControls {
    pub move_left: KeyCode,
    pub move_right: KeyCode,

    pub crouch: KeyCode,
    pub jump: KeyCode,

    pub slash: KeyCode,
    pub shoot: KeyCode,
    pub dash: KeyCode,
}

impl Default for PlayerControls {
    fn default() -> Self {
       Self {
           move_left: KeyCode::A,
           move_right: KeyCode::D,
           crouch: KeyCode::S,
           jump: KeyCode::Space,
           slash: KeyCode::Left,
           shoot: KeyCode::Up,
           dash: KeyCode::Right
       }
    }
}


#[derive(Actionlike, PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub enum InputAction {
    RunLeft,
    RunRight,
    Crouch,
    Jump,

    Slash,
    Dash,
    Shoot,

    Interact,
}

impl InputAction {
    pub fn input_map(controls: &PlayerControls) -> InputMap<Self> {
        use InputAction::*;
        use InputKind::*;

        InputMap::new([
            (Keyboard(controls.move_left), RunLeft),
            (Keyboard(controls.move_right), RunRight),
            (Keyboard(controls.crouch), Crouch),
            (Keyboard(controls.jump), Jump),

            (Keyboard(controls.slash), Slash),
            (Keyboard(controls.shoot), Shoot),
            (Keyboard(controls.dash), Dash),

            (Keyboard(KeyCode::E), Interact),
        ])
    }

    pub fn input_manager_bundle(controls: &PlayerControls) -> InputManagerBundle<Self> {
        InputManagerBundle {
            action_state: ActionState::default(),
            input_map: Self::input_map(&controls)
        }
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(InputManagerPlugin::<InputAction>::default())
            .add_plugin(TomlAssetPlugin::<Config>::new(&["toml"]))
            .init_resource::<PlayerControls>()
            .add_startup_system(load_config)
            .add_system_set(
                SystemSet::on_update(GameState::MainMenu)
                    .with_system(update_key_bindings)
            );
    }
}



fn load_config(mut commands: Commands, asset_server: Res<AssetServer>) {
    let config = ConfigHandle(asset_server.load("config.toml"));
    commands.insert_resource(config);
}

fn update_key_bindings(
    mut ctrl: ResMut<PlayerControls>,
    cfg: Res<ConfigHandle>,
    configs: Res<Assets<Config>>
) {
    if let Some(cfg) = configs.get(&cfg.0) {
        ctrl.move_left = text_to_key_code(&cfg.controls["move_left"]);
        ctrl.move_right = text_to_key_code(&cfg.controls["move_right"]);
        ctrl.crouch = text_to_key_code(&cfg.controls["crouch"]);
        ctrl.jump = text_to_key_code(&cfg.controls["jump"]);
        ctrl.slash = text_to_key_code(&cfg.controls["slash"]);
        ctrl.shoot = text_to_key_code(&cfg.controls["shoot"]);
        ctrl.dash = text_to_key_code(&cfg.controls["dash"]);
    }
}


fn text_to_key_code(text: &str) -> KeyCode {
    use KeyCode::*;

    match text.to_uppercase().as_str() {
        "A" => A,
        "B" => B,
        "C" => C,
        "D" => D,
        "E" => E,
        "F" => F,
        "G" => G,
        "H" => H,
        "I" => I,
        "J" => J,
        "K" => K,
        "L" => L,
        "M" => M,
        "N" => N,
        "O" => O,
        "P" => P,
        "Q" => Q,
        "R" => R,
        "S" => S,
        "T" => T,
        "U" => U,
        "V" => V,
        "W" => W,
        "X" => X,
        "Y" => Y,
        "Z" => Z,

        "SPACE" => Space,

        "LEFT" => Left,
        "RIGHT" => Right,
        "UP" => Up,
        "DOWN" => Down,

        k => panic!("Unsupported key binding {:?}!", k)
    }
}

