use bevy::prelude::*;
use seldom_state::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::bossfight::{Boss, BossStage};
use crate::combat::{DeathTrigger, HurtTrigger};
use crate::entity_states::*;


pub fn register_boss_state_machine(app: &mut App) {
    use TriggerPlugin as TP;

    app
        .add_plugin(TP::<SummonTrigger>::default())
        .add_plugin(TP::<GroundedTrigger>::default())
        .add_plugin(TP::<EnragedTrigger>::default())
        .add_plugin(TP::<VulnerableTrigger>::default())

        .add_plugin(TP::<RestTrigger>::default())
        .add_plugin(TP::<ChargeLeftTrigger>::default())
        .add_plugin(TP::<ChargeRightTrigger>::default())
        .add_plugin(TP::<HoverTrigger>::default())
        .add_plugin(TP::<SlamTrigger>::default())
        .add_plugin(TP::<BoomTrigger>::default())
        .add_plugin(TP::<RelocateTrigger>::default());
}

pub fn boss_state_machine() -> StateMachine {
    StateMachine::new(Fall)
        .trans::<Fall>(GroundedTrigger, Idle)
        .trans::<Hurt>(DoneTrigger::Success, Fall)

        .trans::<Idle>(HurtTrigger, Hurt)
        .trans::<Fall>(HurtTrigger, Hurt)

        .trans::<Idle>(SummonTrigger, Summon)
        .trans::<Summon>(VulnerableTrigger, Vulnerable)
        .trans::<Vulnerable>(SummonTrigger, Summon)

        .trans::<Vulnerable>(HurtTrigger, Hurt)
        .trans::<Idle>(VulnerableTrigger, Vulnerable)

        .trans::<Vulnerable>(EnragedTrigger, BeginEnraged)
        .trans::<BeginEnraged>(AlwaysTrigger, Fall)


        // Enraged attacks
        .trans::<Idle>(RestTrigger, Rest)
        .trans::<Rest>(DoneTrigger::Success, Idle)
        .trans::<Idle>(ChargeLeftTrigger, Charge)
        .trans::<Charge>(DoneTrigger::Success, Idle)
        .trans::<Idle>(ChargeRightTrigger, Charge)
        .trans::<Charge>(DoneTrigger::Success, Idle)
        .trans::<Idle>(HoverTrigger, Hover)
        .trans::<Hover>(DoneTrigger::Success, Idle)
        .trans::<Idle>(RelocateTrigger, Relocate)
        .trans::<Relocate>(DoneTrigger::Success, Idle)
        .trans::<Idle>(SlamTrigger, Slam)
        .trans::<Slam>(DoneTrigger::Success, Idle)
        .trans::<Idle>(BoomTrigger, Boom)
        .trans::<Boom>(DoneTrigger::Success, Idle)




        .trans::<Fall>(DeathTrigger, Die::default())
        .trans::<Idle>(DeathTrigger, Die::default())
        .trans::<Hurt>(DeathTrigger, Die::default())

        .trans::<Die>(NotTrigger(AlwaysTrigger), Die::default())
}




#[derive(Component, Copy, Clone, Reflect)]
pub struct Summon;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Vulnerable;

#[derive(Component, Copy, Clone, Reflect)]
pub struct BeginEnraged;



#[derive(Component, Copy, Clone, Reflect)]
pub struct Rest;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Charge;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Hover;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Relocate;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Slam;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Boom;






#[derive(Copy, Clone, Debug, Reflect, FromReflect)]
pub struct GroundedTrigger;

impl Trigger for GroundedTrigger {
    type Param<'w, 's> = Query<'w, 's,
        &'static KinematicCharacterControllerOutput,
        With<Boss>
    >;

    fn trigger(&self, entity: Entity, outs: &Self::Param<'_, '_>) -> bool {
        if !outs.contains(entity) {
            return false;
        }

        let cc_out = outs.get(entity).unwrap();
        cc_out.grounded
    }
}

#[derive(Copy, Clone, Debug, Reflect, FromReflect)]
pub struct SummonTrigger;

impl Trigger for SummonTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static BossStage>;

    fn trigger(&self, entity: Entity, bosses: &Self::Param<'_, '_>) -> bool {
        if !bosses.contains(entity) {
            return false;
        }

        let stage = *bosses.get(entity).unwrap();
        stage.is_summon_stage()
    }
}

#[derive(Copy, Clone, Debug, Reflect, FromReflect)]
pub struct VulnerableTrigger;

impl Trigger for VulnerableTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static BossStage>;

    fn trigger(&self, entity: Entity, bosses: &Self::Param<'_, '_>) -> bool {
        if !bosses.contains(entity) {
            return false;
        }

        let stage = *bosses.get(entity).unwrap();
        stage.is_vulnerable_stage()
    }
}


#[derive(Copy, Clone, Debug, Reflect, FromReflect)]
pub struct EnragedTrigger;

impl Trigger for EnragedTrigger {
    type Param<'w, 's> = Query<'w, 's, &'static BossStage>;

    fn trigger(&self, entity: Entity, bosses: &Self::Param<'_, '_>) -> bool {
        if !bosses.contains(entity) {
            return false;
        }

        let stage = *bosses.get(entity).unwrap();
        let ok = stage == BossStage::Enraged;

        if ok {
            println!("ok");
        }

        ok
    }
}





macro_rules! attack_trigger {
    ($trig_name: ident, $action: expr) => {
        #[derive(Copy, Clone, Reflect, FromReflect)]
        pub struct $trig_name;

        impl Trigger for $trig_name {
            type Param<'w, 's> = Query<'w, 's, (&'static Boss, &'static BossStage)>;

            fn trigger(&self, _: Entity, boss: &Self::Param<'_, '_>) -> bool {
                if boss.is_empty() {
                    return false;
                }

                let (boss, stage) = boss.single();
                stage == &BossStage::Enraged && ATTACK_SEQUENCE[boss.move_index] == $action
            }
        }
    }
}


use crate::bossfight::enraged::{EnragedAttackMove, ATTACK_SEQUENCE};

attack_trigger!(RestTrigger, EnragedAttackMove::Rest);
attack_trigger!(ChargeLeftTrigger, EnragedAttackMove::ChargeLeft);
attack_trigger!(ChargeRightTrigger, EnragedAttackMove::ChargeRight);
attack_trigger!(HoverTrigger, EnragedAttackMove::Hover);
attack_trigger!(RelocateTrigger, EnragedAttackMove::RelocateRight);
attack_trigger!(SlamTrigger, EnragedAttackMove::Slam);
attack_trigger!(BoomTrigger, EnragedAttackMove::Boom);