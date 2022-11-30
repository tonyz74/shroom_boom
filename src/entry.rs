use bevy::prelude::*;

use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use seldom_state::prelude::*;

use bevy_egui::EguiPlugin;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::{
    assets::AssetLoaderPlugin,
    player::PlayerPlugin,
    input::InputPlugin
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

            .add_startup_system(add_camera)
            .add_startup_system(add_ground)

            .init_resource::<SlashResource>()

            .add_startup_system(add_slash)

            .add_system(draw_slash);
    }
}

pub fn add_camera(mut commands: Commands, mut config: ResMut<RapierConfiguration>) {
    config.gravity = Vect::new(0.0, -500.0);

    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource, Default)]
pub struct SlashResource {
    pub slashes: Vec<Handle<Image>>,
    pub i: usize
}

use crate::common::AnimTimer;

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
        })

    );
}

pub fn add_slash(mut commands: Commands,
                 mut sr: ResMut<SlashResource>,
                 asset_server: Res<AssetServer>) {

    sr.slashes = vec![
        asset_server.load("slash/01.png"),
        asset_server.load("slash/02.png"),
        asset_server.load("slash/03.png"),
        asset_server.load("slash/04.png"),
        asset_server.load("slash/05.png"),
        asset_server.load("slash/06.png"),
        asset_server.load("slash/blank.png")
    ];

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(16., 9.)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::new(8.0, 8.0, 1.0)),
            texture: sr.slashes[0].clone(),
            ..default()
        },


        AnimTimer::from_seconds(0.04)
    ));
}

pub fn draw_slash(time: Res<Time>,
                  mut sr: ResMut<SlashResource>,
                  mut q: Query<(&mut Handle<Image>, &mut AnimTimer)>
) {
    if q.is_empty() { return; }


    let (mut tex, mut timer) = q.single_mut();

    timer.tick(time.delta());

    if timer.just_finished() {
        if sr.i < sr.slashes.len() {
            *tex = sr.slashes[sr.i].clone();
        }

        sr.i = (sr.i + 1) % 20;
    }
}


