use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easings::*;
use crate::assets::IndicatorAssets;
use crate::pathfind::Region;
use crate::state::GameState;
use crate::util::quat_rot2d_deg;

#[derive(Component, Copy, Clone)]
pub struct Indicator {
    pub region: Region,
    pub wait_time: f32,
    pub expand_time: f32,
    pub color: Color,
    pub corner_color: Color
}

#[derive(Component, Clone)]
pub struct IndicatorTimer {
    pub total_timer: Timer,
}

#[derive(Bundle)]
pub struct IndicatorBundle {
    pub indicator: Indicator,
    pub timer: IndicatorTimer,
    #[bundle]
    pub sprite: SpriteBundle,
}


#[derive(Copy, Clone, Debug, Component)]
pub enum IndicatorCornerPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight
}

#[derive(Copy, Clone, Debug, Component)]
pub struct IndicatorCorner {
    pub parent: Entity,
    pub pos: IndicatorCornerPosition
}

impl Indicator {
    pub fn spawn(
        assets: &IndicatorAssets,
        commands: &mut Commands,
        indicator: Self
    ) {
        let id = commands.spawn((
            IndicatorBundle {
                indicator,
                timer: IndicatorTimer {
                    total_timer: Timer::from_seconds(
                        indicator.wait_time + indicator.expand_time,
                        TimerMode::Once
                    ),
                },
                sprite: SpriteBundle {
                    transform: Transform::from_translation(indicator.region.tl.extend(100.0)),
                    ..default()
                }
            },

            Sprite {
                color: indicator.color,
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            }.ease_to(
                Sprite {
                    color: indicator.color,
                    custom_size: Some(indicator.region.extents()),
                    ..default()
                },
                EaseFunction::QuadraticOut,
                EasingType::Once {
                    duration: Duration::from_secs_f32(indicator.expand_time),
                }
            )
        )).id();


        commands.entity(id).with_children(|p| {
            let info = &[
                (0.0, IndicatorCornerPosition::TopLeft),
                (-90.0, IndicatorCornerPosition::TopRight),
                (180.0, IndicatorCornerPosition::BottomRight),
                (-270.0, IndicatorCornerPosition::BottomLeft),
            ];

            for (rot, pos) in info {
                p.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: indicator.corner_color,
                            custom_size: Some(Vec2::new(32.0, 32.0)),
                            ..default()
                        },
                        texture: assets.tr.clone(),
                        transform: Transform::from_rotation(quat_rot2d_deg(*rot)),
                        ..default()
                    },

                    IndicatorCorner {
                        pos: *pos,
                        parent: id
                    }
                ));
            }

        });
    }
}

pub fn register_indicators(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(update_corners)
            .with_system(update_indicators)
    );
}

fn update_indicators(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut IndicatorTimer)>,
) {
    for (entity, mut timer) in q.iter_mut() {
        timer.total_timer.tick(time.delta());

        if timer.total_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn update_corners(
    indicators: Query<&Sprite, With<Indicator>>,
    mut q: Query<(&mut Transform, &IndicatorCorner)>
) {
    for (mut tf, corner) in q.iter_mut() {
        let spr = indicators.get(corner.parent).unwrap();

        if spr.custom_size.is_none() {
            continue;
        }

        let half = (spr.custom_size.unwrap() / 2.0) - 12.0;

        let off = match corner.pos {
            IndicatorCornerPosition::TopLeft => Vec2::new(-half.x, half.y),
            IndicatorCornerPosition::TopRight => Vec2::new(half.x, half.y),
            IndicatorCornerPosition::BottomLeft => Vec2::new(-half.x, -half.y),
            IndicatorCornerPosition::BottomRight => Vec2::new(half.x, -half.y)
        };

        tf.translation = off.extend(101.0);

    }
}