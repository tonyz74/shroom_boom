use bevy::prelude::*;

mod melee;
mod projectile;

mod components;
mod events;
mod hurt;
mod collision;

pub use melee::*;
pub use projectile::*;
pub use components::*;
pub use events::*;
pub use hurt::*;
pub use collision::*;

use crate::assets::FlowerEnemyAssets;
use crate::combat::collision::register_collider_attacks;
use crate::common::AnimTimer;
use crate::entity_states::*;
use crate::player::Player;

use crate::state::GameState;
pub struct AttackPlugin;


impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {

        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(resolve_melee_attacks)
                    .with_system(move_projectile_attacks)
                    .with_system(projectile_hit_targets)
                    .with_system(remove_projectiles_on_impact)

                    .with_system(temp_shoot)
                    .with_system(handle_hits)
            )

            .add_event::<CombatEvent>();

        register_projectile_attacks(app);
        register_hurt_ability(app);
        register_collider_attacks(app);
    }
}

fn handle_hits(
    immune: Query<&Immunity>,
    mut q: Query<(Entity, &mut HurtAbility, &mut Health), Without<Hurt>>,
    mut hit_events: EventReader<CombatEvent>
) {
    for hit in hit_events.iter() {
        if let Ok((entity, mut hurt, mut health)) = q.get_mut(hit.target) {

            if immune.contains(entity) {
                hurt.hit_event = None;
                continue;
            }

            health.hp -= hit.damage;
            hurt.hit_event = Some(*hit);
        }
    }
}

pub fn temp_shoot(
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    input: Res<Input<KeyCode>>,
    assets: Res<FlowerEnemyAssets>
) {
    if input.just_pressed(KeyCode::Return) {
        let player_pos = player.single().translation();
        
        commands.spawn(ProjectileAttackBundle {
            anim_timer: AnimTimer::from_seconds(0.4),

            sprite_sheet: SpriteSheetBundle {

                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(16., 16.)),
                    ..default()
                },

                texture_atlas: assets.anims["IDLE"].tex.clone(),

                transform: Transform::from_xyz(player_pos.x, player_pos.y, 0.0),

                ..default()
            },

            attack: ProjectileAttack {
                vel: Vec2::new(12.0, 0.0),
                speed: 12.0,
                ..default()
            },

            strength: AttackStrength::new(2),
            combat_layer: CombatLayerMask::PLAYER,

            ..ProjectileAttackBundle::from_size(Vec2::new(16., 16.))
        });
    }
}