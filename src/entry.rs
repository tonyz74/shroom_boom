use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use seldom_state::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_easings::EasingsPlugin;

use crate::{
    assets::AssetLoaderPlugin,
    player::PlayerPlugin,
    input::InputPlugin,
    level::LevelPlugin,
    enemies::EnemyPlugin,
    camera::CameraPlugin,
    pathfind::PathfindingPlugin,
    combat::AttackPlugin,
    fx::EffectsPlugin,
    coin::CoinPlugin,
    bossfight::BossPlugin
};

pub struct ShadePlugin;

impl Plugin for ShadePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
            )

            // physics
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(EguiPlugin)
            .add_plugin(WorldInspectorPlugin::new())
            .add_plugin(EasingsPlugin)

            .add_plugin(bevy_debug_text_overlay::OverlayPlugin::default())

            .insert_resource(ClearColor(Color::rgb(0.015, 0.015, 0.1)))

            // state machine
            .add_plugin(StateMachinePlugin)

            // subsystems
            .add_plugin(CameraPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(InputPlugin)

            // gameplay
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(PathfindingPlugin)
            .add_plugin(AttackPlugin)
            .add_plugin(CoinPlugin)
            .add_plugin(BossPlugin)

            // polish
            .add_plugin(EffectsPlugin)

            .add_startup_system(setup_rapier);
    }
}

pub fn setup_rapier(config: ResMut<RapierConfiguration>) {
    let _ = config;
}