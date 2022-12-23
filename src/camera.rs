use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

use crate::{
    state::GameState,
    player::Player,
    level::consts::SCALE_FACTOR
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera);

        app.add_system_set(
            SystemSet::new()
                .with_system(camera_track_player)
        );

        app.add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(camera_reconfigure_to_fit_level)
        );
    }
}

#[derive(Component)]
pub struct GameCamera {
    pub border_min: Vec2,
    pub border_max: Vec2
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle::default(),

        GameCamera {
            border_min: Vec2::new(0.0, 0.0),
            border_max: Vec2::new(0.0, 0.0)
        }
    ));
}

pub fn camera_track_player(
    p: Query<&GlobalTransform, With<Player>>,
    mut q: Query<(&mut Transform, &GameCamera)>
) {
    if p.is_empty() || q.is_empty() {
        return;
    }

    let pos = p.single().translation();
    let (mut cam_pos, cam) = q.single_mut();

    let _ = cam;
    let fac_x = 20.0;
    let fac_y = 20.0;

    cam_pos.translation.x += (pos.x - cam_pos.translation.x) / fac_x;
    cam_pos.translation.y += (pos.y - cam_pos.translation.y) / fac_y;

    cam_pos.translation.x = cam_pos.translation.x.clamp(cam.border_min.x, cam.border_max.x);
    cam_pos.translation.y = cam_pos.translation.y.clamp(cam.border_min.y, cam.border_max.y);

    cam_pos.translation.z = 999.0;
}

fn camera_reconfigure_to_fit_level(
    mut camera: Query<&mut GameCamera>,

    levels: Query<&Handle<LdtkAsset>>,
    level_sel: Res<LevelSelection>,

    windows: Res<Windows>,
    assets: Res<Assets<LdtkAsset>>,
) {
    if levels.is_empty() || assets.is_empty() {
        return;
    }

    for mut camera in camera.iter_mut() {
        let win = windows.get_primary().unwrap();
        let level = assets.get(levels.single()).unwrap();

        if let Some(lvl) = level.get_level(&level_sel) {
            let half_extents = Vec2::new(
                win.width() / 2.0,
                win.height() / 2.0
            );

            camera.border_min = half_extents;
            camera.border_max = Vec2::new(
                lvl.px_wid as f32 * SCALE_FACTOR,
                lvl.px_hei as f32 * SCALE_FACTOR
            ) - half_extents;

            if camera.border_min.x > camera.border_max.x
                || camera.border_min.y > camera.border_max.y {
                camera.border_max = camera.border_max + half_extents;
            }
        }
    }
}