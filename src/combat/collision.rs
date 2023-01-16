use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::combat::{AttackStrength, CombatEvent, CombatLayerMask, KnockbackModifier};
use crate::combat::knockbacks::collider_attack_knockback;
use crate::state::GameState;

#[derive(Copy, Clone, Debug, Component)]
pub struct ColliderAttack {
    pub enabled: bool
}

impl Default for ColliderAttack {
    fn default() -> Self {
        Self { enabled: true }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct ColliderAttackBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub rigid_body: RigidBody,
    pub attack: ColliderAttack,
    pub strength: AttackStrength,
    pub knockback: KnockbackModifier,
    pub combat_layer: CombatLayerMask,

    #[bundle]
    pub transform: TransformBundle,
}

impl ColliderAttackBundle {
    pub fn from_size(size: Vec2) -> Self {
        Self {
            collider: Collider::cuboid(size.x / 2.0, size.y / 2.0),
            sensor: Sensor,
            rigid_body: RigidBody::Fixed,
            attack: Default::default(),
            strength: Default::default(),
            knockback: Default::default(),
            combat_layer: Default::default(),
            transform: Default::default()
        }
    }
}

pub fn register_collider_attacks(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(collider_attack_update)
    );
}

fn collider_attack_update(
    transforms: Query<&GlobalTransform>,
    combat_layers_query: Query<&CombatLayerMask>,
    attacks: Query<(
        &GlobalTransform,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &ColliderAttack,
        &KnockbackModifier
    )>,
    rapier: Res<RapierContext>,
    mut hit_events: EventWriter<CombatEvent>
) {
   for (transform, collider, combat_layer, atk, collider_atk, kb_amp) in attacks.iter() {
       if !collider_atk.enabled {
           continue;
       }

       let self_pos = transform.translation();

       rapier.intersections_with_shape(
           Vect::new(self_pos.x, self_pos.y),
           Rot::default(),
           collider,
           QueryFilter {
               flags: QueryFilterFlags::ONLY_KINEMATIC,
               ..default()
           },
           |hit| {
               if let Ok(hit_combat_layer) = combat_layers_query.get(hit) {
                   // Skip if they're friendly (friendly fire is not enabled)
                   if hit_combat_layer.is_ally_with(*combat_layer) {
                       return true;
                   }

                   let hit_pos = transforms.get(hit).unwrap().translation();
                   let dir = (hit_pos - self_pos).normalize();

                   // Only accept hits that have occurred between enemies
                   hit_events.send(CombatEvent {
                       target: hit,
                       damage: atk.power,
                       kb: (kb_amp.mod_fn)(
                           collider_attack_knockback(Vec2::new(dir.x, dir.y))
                       )
                   });
               }

               true
           }
       );
   }
}