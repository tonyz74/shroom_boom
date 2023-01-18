use std::time::Duration;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_easings::*;
use crate::assets::IndicatorAssets;
use crate::level::consts::RENDERED_TILE_SIZE;
use crate::pathfind::Region;
use crate::state::GameState;
use crate::util::quat_rot2d_deg;

#[derive(Component, Copy, Clone, Default)]
pub struct Indicator {
    pub region: Region,
    pub wait_time: f32,
    pub expand_time: f32,
    pub color: Color,
    pub corner_color: Color,
    pub z: f32
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

impl Indicator {
    pub fn spawn(
        assets: &IndicatorAssets,
        commands: &mut Commands,
        indicator: Indicator,
    ) {
        {
            let extents = indicator.region.extents();
            if extents.x < RENDERED_TILE_SIZE || extents.y < RENDERED_TILE_SIZE {
                panic!("Indicator is too small! ({:?})", extents);
            }
        }

        let half = indicator.region.extents() / 2.0;
        let center = indicator.region.tl + Vec2::new(half.x, -half.y);
        let ease_func = EaseFunction::ExponentialInOut;
        let ease_type = EasingType::PingPong {
            duration: Duration::from_secs_f32(indicator.expand_time),
            pause: Some(Duration::from_secs_f32(indicator.wait_time))
        };

        let id = commands.spawn((
            IndicatorBundle {
                indicator,
                timer: IndicatorTimer {
                    total_timer: Timer::from_seconds(
                        indicator.wait_time + indicator.expand_time * 2.0,
                        TimerMode::Once
                    ),
                },
                sprite: SpriteBundle {
                    transform: Transform::from_translation(center.extend(indicator.z)),
                    ..default()
                }
            },

            Sprite {
                color: indicator.color,
                custom_size: Some(Vec2::splat(RENDERED_TILE_SIZE)),
                ..default()
            }.ease_to(
                Sprite {
                    color: indicator.color,
                    custom_size: Some(indicator.region.extents()),
                    ..default()
                },
                ease_func,
                ease_type
            )
        )).id();


        commands.entity(id).with_children(|p| {
            let corner_pos = half - 12.0;

            let info = &[
                (0.0, Vec2::new(-corner_pos.x, corner_pos.y)),
                (-90.0, Vec2::new(corner_pos.x, corner_pos.y)),
                (180.0, Vec2::new(corner_pos.x, -corner_pos.y)),
                (-270.0, Vec2::new(-corner_pos.x, -corner_pos.y)),
            ];

            for (rot, off) in info {
                let transform = Transform::from_rotation(quat_rot2d_deg(*rot));

                p.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: indicator.corner_color,
                            custom_size: Some(Vec2::splat(RENDERED_TILE_SIZE)),
                            ..default()
                        },
                        texture: assets.tr.clone(),
                        ..default()
                    },

                    transform.ease_to(
                        transform.with_translation(off.extend(0.0)),
                        ease_func,
                        ease_type
                    )
                ));
            }

        });
    }
}

pub fn register_indicators(app: &mut App) {
    app
        .add_event::<Indicator>()
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(update_indicators)
                .with_system(handle_indicator_events)
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



fn handle_indicator_events(
    assets: Res<IndicatorAssets>,
    mut commands: Commands,
    mut reader: EventReader<Indicator>
) {
    for ev in reader.iter() {
        Indicator::spawn(&assets, &mut commands, *ev);
    }
}

impl Indicator {
    pub const EXPLOSION: Self = Indicator {
        color: Color::rgba(1.0, 0.2, 0.2, 0.6),
        corner_color: Color::rgba(1.0, 0.5, 0.5, 1.0),

        region: Region { tl: Vec2::ZERO, br: Vec2::ZERO },
        wait_time: 0.0,
        expand_time: 0.0,
        z: 10.0
    };

    pub const ATTACK: Self = Indicator {
        color: Color::rgba(1.0, 0.2, 0.2, 0.6),
        corner_color: Color::rgba(1.0, 0.5, 0.5, 1.0),

        region: Region { tl: Vec2::ZERO, br: Vec2::ZERO },
        wait_time: 0.0,
        expand_time: 0.0,
        z: 0.0
    };

    pub const SPAWNER: Self = Indicator {
        color: Color::rgba(0.2, 0.2, 1.0, 0.6),
        corner_color: Color::rgba(0.3, 0.3, 1.0, 1.0),

        region: Region { tl: Vec2::ZERO, br: Vec2::ZERO },
        wait_time: 0.0,
        expand_time: 0.0,
        z: 10.0
    };
}