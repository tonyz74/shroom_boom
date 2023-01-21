use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::*;
use kayak_ui::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_easings::EasingsPlugin;
use kayak_ui::widgets::KayakWidgets;

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
    bossfight::BossPlugin,
    ui::GameUiPlugin,
    shop::ShopPlugin,
    interact::InteractPlugin
};

pub struct ShadePlugin;

impl Plugin for ShadePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
            )

            .add_plugin(KayakContextPlugin)
            .add_plugin(KayakWidgets)

            // physics
            .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            // .add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(EguiPlugin)
            // .add_plugin(WorldInspectorPlugin::new())
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
            .add_plugin(GameUiPlugin)
            .add_plugin(EffectsPlugin)
            .add_plugin(ShopPlugin)
            .add_plugin(InteractPlugin)

            .add_startup_system(setup_rapier);
    }
}

pub fn setup_rapier(config: ResMut<RapierConfiguration>) {
    let _ = config;
}