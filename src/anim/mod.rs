use std::time::Duration;
use bevy::prelude::*;
use crate::state::GameState;
use crate::util::Facing;


pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
       app.add_event::<AnimationChangeEvent>().add_system_set(
           SystemSet::on_update(GameState::Gameplay)
               .with_system(animation_tick)
               .with_system(handle_animation_change_events)
               .with_system(flip_sprite_on_direction)
       );
    }
}




#[derive(Debug, Default, Clone)]
pub struct Animation {
    pub tex: Handle<TextureAtlas>,
    pub speed: f32,
    pub facing_flipped: bool
}

impl Animation {
    pub fn new(handle: Handle<TextureAtlas>, speed: f32) -> Self {
        Self::new_flipped(handle, speed, false)
    }

    pub fn new_flipped(handle: Handle<TextureAtlas>, speed: f32, flipped: bool) -> Self {
        Animation {
            tex: handle,
            speed,
            facing_flipped: flipped
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct Animator {
    pub timer: Timer,
    pub total_frames: u32,
    pub anim: Animation,
}

impl Animator {
    pub fn new(anim: Animation) -> Animator {
        Self {
            timer: Timer::from_seconds(anim.speed, TimerMode::Repeating),
            anim,
            ..default()
        }
    }
}

impl Default for Animator {
    fn default() -> Self {
        Self {
            anim: Animation::default(),
            timer: Timer::new(Duration::MAX, TimerMode::Once),
            total_frames: 0,
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
            if anim.anim.tex == event.new_anim.tex {
                continue;
            }

            anim.total_frames = 0;
            anim.timer.set_duration(Duration::from_secs_f32(event.new_anim.speed));
            anim.timer.reset();

            *texture_atlas = event.new_anim.tex.clone();
            spr.index = 0;

            anim.anim = event.new_anim.clone();
        }
    }
}

fn flip_sprite_on_direction(mut q: Query<(&mut TextureAtlasSprite, &Animator, &Facing)>) {
    for (mut sprite, anim, facing) in q.iter_mut() {
        match facing {
            Facing::Left => {
                sprite.flip_x = !anim.anim.facing_flipped
            },

            Facing::Right => {
                sprite.flip_x = anim.anim.facing_flipped
            }
        };

    }
}