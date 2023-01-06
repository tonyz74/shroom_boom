use std::collections::HashSet;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use seldom_state::prelude::StateMachine;
use crate::combat::{AttackStrength, CombatLayerMask, HitEvent, Immunity};
use crate::common::AnimTimer;



mod state {
    use bevy::prelude::*;
    use seldom_state::prelude::*;

    #[derive(Component, Copy, Clone, Default, Reflect, FromReflect)]
    pub struct Impact;

    #[derive(Component, Copy, Clone, Default, Reflect, FromReflect)]
    pub struct Travel;



    #[derive(Copy, Clone, Reflect, FromReflect)]
    pub struct CollidedTrigger;

    impl Trigger for CollidedTrigger {
        type Param<'w, 's> = Query<'w, 's, &'static super::ProjectileAttack>;

        fn trigger(&self, entity: Entity, projs: &Self::Param<'_, '_>) -> bool {
            if !projs.contains(entity) {
                return false;
            }

            projs.get(entity).unwrap().collided
        }
    }


    pub fn projectile_state_machine() -> StateMachine {
        StateMachine::new(Travel)
            .trans::<Travel>(CollidedTrigger, Impact)
            .trans::<Impact>(NotTrigger(AlwaysTrigger), Impact)
    }

    pub fn projectile_register_triggers(app: &mut App) {
        app.add_plugin(TriggerPlugin::<CollidedTrigger>::default());
    }
}

pub fn register_projectile_attacks(app: &mut App) {
    state::projectile_register_triggers(app);
}



#[derive(Component, Default, Clone, Debug)]
pub struct ProjectileAttack {
    pub vel: Vec2,
    pub speed: f32,
    pub collided: bool
}

#[derive(Bundle, Clone)]
pub struct ProjectileAttackBundle {
    pub anim_timer: AnimTimer,
    pub sprite_sheet: SpriteSheetBundle,
    pub collider: Collider,
    pub sensor: Sensor,
    pub attack: ProjectileAttack,
    pub strength: AttackStrength,
    pub combat_layer: CombatLayerMask,
    pub controller: KinematicCharacterController,
    pub state_machine: StateMachine
}

impl Default for ProjectileAttackBundle {
    fn default() -> Self {
       Self {
           anim_timer: Default::default(),
           sprite_sheet: Default::default(),
           collider: Default::default(),
           sensor: Default::default(),
           attack: Default::default(),
           strength: Default::default(),
           combat_layer: Default::default(),
           controller: Default::default(),
           state_machine: state::projectile_state_machine()
       }
    }
}

impl ProjectileAttackBundle {
    pub fn from_size(size: Vec2) -> Self {
        Self {
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            sensor: Sensor,
            controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.02),
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS | QueryFilterFlags::EXCLUDE_FIXED,
                ..default()
            },
            ..default()
        }
    }
}

pub fn move_projectile_attacks(
    mut proj: Query<(&ProjectileAttack, &mut KinematicCharacterController)>
) {
    for (proj, mut cc) in proj.iter_mut() {
        cc.translation = Some(proj.vel);
    }
}

pub fn projectile_hit_targets(
    immunities: Query<&Immunity>,
    transforms: Query<&GlobalTransform>,
    mut projectiles: Query<(
        Entity,
        &Collider,
        &CombatLayerMask,
        &AttackStrength,
        &mut ProjectileAttack
    )>,
    rapier: Res<RapierContext>,

    combat_layers: Query<&CombatLayerMask>,
    mut hit_events: EventWriter<HitEvent>,
) {
    for (entity, collider, proj_combat_layer, strength, mut proj) in projectiles.iter_mut() {
        let transform = transforms.get(entity).unwrap();
        let proj_pos = transform.translation();

        rapier.intersections_with_shape(
            Vect::new(proj_pos.x, proj_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_FIXED | QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
            |hit_entity| {
                proj.collided = true;
                true
            }
        );

        rapier.intersections_with_shape(
            Vect::new(proj_pos.x, proj_pos.y),
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |hit_entity| {
                if let Ok(layer) = combat_layers.get(hit_entity) {
                    if layer.is_ally_with(*proj_combat_layer) || immunities.contains(hit_entity) {
                        return true;
                    }

                    let hit_pos = transforms.get(hit_entity).unwrap().translation();
                    let mut dir = (hit_pos - proj_pos).normalize();

                    if (dir.x < 0.0 && proj.vel.x > 0.0) || (dir.x > 0.0 && proj.vel.x < 0.0) {
                        dir.x *= -1.0;
                    }

                    hit_events.send(HitEvent {
                        target: hit_entity,
                        damage: strength.power,
                        kb: Vec2::new(dir.x, dir.y)
                    });

                    proj.collided = true;
                }

                true
            }
        );
   }
}




pub fn remove_projectiles_on_impact(
    mut commands: Commands,
    impacted: Query<Entity, Added<state::Impact>>,
) {
    for entity in impacted.iter() {
        if let Some(mut cmd) = commands.get_entity(entity) {
            cmd.despawn();
        }
    }
}