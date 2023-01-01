use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use seldom_state::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::{
    assets::AssetLoaderPlugin,
    player::PlayerPlugin,
    input::InputPlugin,
    level::LevelLoaderPlugin,
    enemies::EnemyPlugin,
    camera::CameraPlugin,
    pathfind::PathfindingPlugin,
    attack::AttackPlugin
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
            .add_plugin(RapierDebugRenderPlugin::default())

            // egui
            .add_plugin(EguiPlugin)
            .add_plugin(WorldInspectorPlugin::new())

            .add_plugin(bevy_debug_text_overlay::OverlayPlugin::default())

            .insert_resource(ClearColor(Color::rgb(0.015, 0.015, 0.1)))

            // custom shapes
            .add_plugin(ShapePlugin)

            // state machine
            .add_plugin(StateMachinePlugin)

            // subsystems
            .add_plugin(CameraPlugin)
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(InputPlugin)

            // gameplay
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(LevelLoaderPlugin)
            .add_plugin(PathfindingPlugin)
            .add_plugin(AttackPlugin)

            .add_startup_system(setup_rapier);
    }
}

pub fn setup_rapier(config: ResMut<RapierConfiguration>) {
    let _ = config;
}