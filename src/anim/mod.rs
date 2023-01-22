use std::time::Duration;
use bevy::prelude::*;
use crate::state::GameState;


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
       app.add_event::<AnimationChangeEvent>().add_system_set(
           SystemSet::on_update(GameState::Gameplay)
               .with_system(animation_tick)
               .with_system(handle_animation_change_events)
       );
    }
}


#[derive(Debug, Default, Clone)]
pub struct Animation {
    pub tex: Handle<TextureAtlas>,
    pub speed: f32
}

impl Animation {
    pub fn new(handle: Handle<TextureAtlas>, speed: f32) -> Self {
        Animation {
            tex: handle,
            speed
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Animator {
    pub timer: Timer,
    pub total_frames: u32
}

impl Animator {
    pub fn new(anim: Animation) -> Animator {
        Self {
            timer: Timer::from_seconds(anim.speed, TimerMode::Repeating),
            total_frames: 0
        }
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::MAX, TimerMode::Once),
            total_frames: 0
        }
    }
}


pub fn animation_tick(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut q: Query<(&mut TextureAtlasSprite, &mut Animator, &Handle<TextureAtlas>)>,
) {
    for (mut spr, mut anim, handle) in q.iter_mut() {
        anim.timer.tick(time.delta());

        if anim.timer.just_finished() {
            let atlas = texture_atlases.get(handle).unwrap();
            spr.index = (spr.index + 1) % atlas.textures.len();
        }
    }
}


#[derive(Component, Debug, Clone, Resource)]
pub struct AnimationChangeEvent {
    pub e: Entity,
    pub new_anim: Animation,
}

pub fn handle_animation_change_events(
    mut animations: Query<(&mut TextureAtlasSprite, &mut Animator, &mut Handle<TextureAtlas>)>,
    mut events: EventReader<AnimationChangeEvent>
) {
    for event in events.iter() {
        if let Ok((mut spr, mut anim, mut texture_atlas)) = animations.get_mut(event.e) {
            anim.total_frames = 0;
            anim.timer.set_duration(Duration::from_secs_f32(event.new_anim.speed));
            anim.timer.reset();

            *texture_atlas = event.new_anim.tex.clone();
            spr.index = 0;
        }
    }
}