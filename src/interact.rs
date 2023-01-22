use std::time::Duration;
use bevy::prelude::*;
use bevy_easings::*;
use bevy::math::Vec3Swizzles;
use leafwing_input_manager::prelude::ActionState;
use seldom_state::prelude::StateMachine;
use crate::assets::UiAssets;
use crate::input::InputAction;
use crate::player::Player;
use crate::state::GameState;
use crate::util;

pub const TEXT_POPUP_SPEED: f32 = 0.2;



pub struct InteractPlugin;

impl Plugin for InteractPlugin {
    fn build(&self, app: &mut App) {
       app.add_system_set(
           SystemSet::new()
               .with_system(interact_update)
               .with_system(interact_spawn_text)
               .with_system(despawn_text)
               .with_system(interact_with)
               .with_system(update_interact_timer)
       );
    }
}




#[derive(Component, Clone, Debug)]
pub struct Interact {
    pub content: Text,
    pub text: Option<Entity>,
    pub max_dist: f32,
    pub within: bool,
    pub text_offset: Vec2,
    pub interacted: Timer
}

impl Default for Interact {
    fn default() -> Self {
        let mut interacted = Timer::from_seconds(0.1, TimerMode::Once);
        util::timer_tick_to_finish(&mut interacted);

        Self {
            content: Text::default(),
            text: None,
            max_dist: 360.0,
            within: false,
            text_offset: Vec2::default(),
            interacted
        }
    }
}

impl Interact {
    pub fn interacted_with(&self) -> bool {
        !self.interacted.finished()
    }
}

#[derive(Component, Clone, Default)]
pub struct InteractText {
    pub despawn_countdown: Option<Timer>
}

#[derive(Clone, Bundle)]
pub struct InteractTextBundle {
    #[bundle]
    text: Text2dBundle,
    interact_text: InteractText,
}

pub fn interact_update(
    p: Query<&GlobalTransform, With<Player>>,
    mut q: Query<(&GlobalTransform, &mut Interact)>
) {
    if p.is_empty() {
        return;
    }

    let p_pos = p.single().translation().xy();

    for (tf, mut interact) in q.iter_mut() {
        let i_pos = tf.translation().xy();
        interact.within = p_pos.distance(i_pos) <= interact.max_dist;
    }
}

pub fn interact_spawn_text(
    mut commands: Commands,
    mut q: Query<(Entity, &mut Interact)>,
    mut r: Query<&mut InteractText>,
) {
    for (entity, mut interact) in q.iter_mut() {
        let transform = Transform::from_translation(interact.text_offset.extend(10.0));
        let visible = transform.with_scale(Vec3::splat(1.0));
        let not_visible = transform.with_scale(Vec3::splat(0.0));

        if interact.within && interact.text.is_none() {
            let id = commands.spawn((
                InteractTextBundle {
                    text: Text2dBundle {
                        text: interact.content.clone()
                            .with_alignment(TextAlignment::CENTER),
                        transform: not_visible,
                        ..default()
                    },

                    interact_text: InteractText {
                        despawn_countdown: None
                    }
                },

                not_visible.ease_to(
                    visible,
                    EaseFunction::CircularInOut,
                    EasingType::Once {
                        duration: Duration::from_secs_f32(TEXT_POPUP_SPEED),
                    }
                )
            )).id();

            interact.text = Some(id);
            commands.entity(entity).push_children(&[id].as_slice());

        } else if !interact.within && interact.text.is_some() {

            commands.entity(interact.text.unwrap()).insert(
                visible.ease_to(
                    not_visible,
                    EaseFunction::CircularInOut,
                    EasingType::Once {
                        duration: Duration::from_secs_f32(TEXT_POPUP_SPEED),
                    }
                ),
            );

            let text = &mut r.get_mut(interact.text.unwrap()).unwrap();
            text.despawn_countdown = Some(Timer::from_seconds(TEXT_POPUP_SPEED, TimerMode::Once));
            interact.text = None;
        }
    }
}






fn despawn_text(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut InteractText)>
) {
    for (e, mut interact) in q.iter_mut() {
        if let Some(timer) = &mut interact.despawn_countdown {
            timer.tick(time.delta());

            if timer.just_finished() {
                commands.entity(e).despawn();
            }
        }
    }
}


fn interact_with(
    p: Query<&ActionState<InputAction>, With<Player>>,
    mut q: Query<&mut Interact>
) {
    if p.is_empty() {
        return;
    }

    let input = p.single();

    for mut interact in q.iter_mut() {
        if interact.within && input.just_pressed(InputAction::Interact) {
            // Only allow for one interaction at a time
            interact.interacted.reset();
            return;
        }
    }
}

fn update_interact_timer(
    time: Res<Time>,
    mut q: Query<&mut Interact>
) {
    for mut interact in q.iter_mut() {
        interact.interacted.tick(time.delta());
    }
}