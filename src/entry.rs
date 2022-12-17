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
    level::LevelLoaderPlugin
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

            // custom shapes
            .add_plugin(ShapePlugin)

            // state machine
            .add_plugin(StateMachinePlugin)

            // subsystems
            .add_plugin(AssetLoaderPlugin)
            .add_plugin(InputPlugin)

            // gameplay
            .add_plugin(PlayerPlugin)
            .add_plugin(LevelLoaderPlugin)

            .add_system(track_player)

            .add_startup_system(add_camera);
            // .add_startup_system(add_ground);
    }
}

pub fn add_camera(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vect::new(0.0, -500.0);
    commands.spawn(Camera2dBundle::default());
}

pub fn track_player(
    p: Query<&GlobalTransform, With<crate::player::Player>>,
    mut q: Query<&mut Transform, With<Camera2d>>
) {
    if p.is_empty() || q.is_empty() {
        return;
    }

    let pos = p.single().translation();
    let mut cam_pos = q.single_mut();

    let fac_x = 20.0;
    let fac_y = 20.0;

    cam_pos.translation.x += (pos.x - cam_pos.translation.x) / fac_x;
    cam_pos.translation.y += (pos.y - cam_pos.translation.y) / fac_y;
}

pub fn add_ground(mut commands: Commands) {
    commands.spawn((
        Restitution::coefficient(0.8),
        Friction::coefficient(0.0),
        Collider::cuboid(500.0, 50.0),

        RigidBody::Fixed,

        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 0.4, 0.4),
                custom_size: Some(Vec2::new(1000.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            ..default()
        }
    ));

    commands.spawn((
        Restitution::coefficient(0.8),
        Friction::coefficient(0.0),
        Collider::ball(50.0),

        TransformBundle::from(Transform::from_xyz(600.0, 0.0, 0.0)),

        RigidBody::Fixed,
    ));

    commands.spawn((
        Restitution::coefficient(0.8),
        Friction::coefficient(0.0),
        Collider::ball(50.0),

        TransformBundle::from(Transform::from_xyz(800.0, -100.0, 0.0)),

        RigidBody::Fixed,
    ));
}
