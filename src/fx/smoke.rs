use bevy::prelude::*;
use crate::anim::Animator;
use crate::anim::map::AnimationMap;
use crate::assets::IndicatorAssets;
use crate::state::GameState;


pub struct SmokeEvent {
    pub pos: Vec2
}

pub fn register_smoke(app: &mut App) {
    app.add_event::<SmokeEvent>().add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(spawn_smoke)
            .with_system(despawn_smoke)
    );
}


#[derive(Component, Copy, Clone)]
pub struct Smoke;

#[derive(Bundle)]
pub struct SmokeBundle {
    pub anim: Animator,
    pub anim_map: AnimationMap,
    pub smoke: Smoke,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle
}


fn spawn_smoke(
    mut commands: Commands,
    mut ev: EventReader<SmokeEvent>,
    assets: Res<IndicatorAssets>
) {
    for smoke in ev.iter() {
        commands.spawn(SmokeBundle {
            anim: Animator::new(assets.smoke_map["SMOKE"].clone()),

            anim_map: assets.smoke_map.clone(),

            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(48.0, 96.0)),
                    ..default()
                },
                texture_atlas: assets.smoke_map["SMOKE"].tex.clone(),
                transform: Transform::from_xyz(smoke.pos.x, smoke.pos.y + 20.0, 10.0),
                ..default()
            },

            smoke: Smoke
        });
    }
}

fn despawn_smoke(
    mut commands: Commands,
    q: Query<(Entity, &Animator), With<Smoke>>
) {
    for (ent, animator) in q.iter() {
        if animator.total_looped == 1 {
            commands.entity(ent).despawn();
        }
    }
}