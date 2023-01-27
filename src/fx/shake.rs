use bevy::prelude::*;
use bevy_easings::Lerp;
use rand::prelude::*;
use crate::camera::{camera_track_player, GameCamera};
use crate::state::GameState;

pub const SHAKE_DECAY_RATE: f32 = 5.0;

#[derive(Resource, Component, Debug, Copy, Clone, Default)]
pub struct ScreenShakeManager {
    pub intensity: f32
}

#[derive(Resource, Component, Debug, Copy, Clone)]
pub struct ScreenShakeEvent {
    pub intensity: f32
}

impl ScreenShakeEvent {
    pub const TINY: Self = Self {
        intensity: 5.0
    };

    pub const SMALL: Self = Self {
        intensity: 10.0
    };

    pub const MEDIUM: Self = Self {
        intensity: 20.0
    };

    pub const LARGE: Self = Self {
        intensity: 30.0
    };

    pub const HUGE: Self = Self {
        intensity: 50.0
    };
}

pub fn register_screen_shake(app: &mut App) {
    app
        .init_resource::<ScreenShakeManager>()
        .add_event::<ScreenShakeEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(handle_events)
                .with_system(shake_update.after(camera_track_player))
        );
}

fn handle_events(
    mut mgr: ResMut<ScreenShakeManager>,
    mut ev: EventReader<ScreenShakeEvent>
) {
    for shake in ev.iter() {
        mgr.intensity += shake.intensity;
    }
}

fn shake_update(
    time: Res<Time>,
    mut mgr: ResMut<ScreenShakeManager>,
    mut cam: Query<&mut Transform, With<GameCamera>>
) {
    if cam.is_empty() {
        return;
    }

    let mut rng = thread_rng();
    let mut tf = cam.single_mut();

    mgr.intensity = mgr.intensity.lerp(&0.0_f32, &(SHAKE_DECAY_RATE * time.delta().as_secs_f32()));

    if -mgr.intensity >= mgr.intensity {
        tf.translation.x += rng.gen_range(-mgr.intensity..mgr.intensity);
        tf.translation.y += rng.gen_range(-mgr.intensity..mgr.intensity);
    }
}
