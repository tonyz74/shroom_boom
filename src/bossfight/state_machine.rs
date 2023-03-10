use bevy::prelude::*;
use seldom_state::prelude::*;
use crate::bossfight::{Boss, BossStage};
use crate::bossfight::abilities::RestAbility;
use crate::combat::{DeathTrigger, HurtTrigger};
use crate::entity_states::*;


pub fn register_boss_state_machine(app: &mut App) {
    use TriggerPlugin as TP;

    app
        .add_plugin(TP::<SummonTrigger>::default())
        .add_plugin(TP::<EnragedTrigger>::default())
        .add_plugin(TP::<VulnerableTrigger>::default())

        .add_plugin(TP::<RestTrigger>::default())
        .add_plugin(TP::<PickNextMoveTrigger>::default())
        .add_plugin(TP::<TakeoffTrigger>::default())
        .add_plugin(TP::<ChargeLeftTrigger>::default())
        .add_plugin(TP::<ChargeRightTrigger>::default())
        .add_plugin(TP::<HoverTrigger>::default())
        .add_plugin(TP::<SlamTrigger>::default())
        .add_plugin(TP::<BoomTrigger>::default())
        .add_plugin(TP::<LeapTrigger>::default())
        .add_plugin(TP::<RelocateTrigger>::default());
}

pub fn boss_state_machine() -> StateMachine {
    StateMachine::new(Idle)
        .trans::<Hurt>(DoneTrigger::Success, Idle)
        .trans::<Idle>(HurtTrigger, Hurt)

        .trans::<Idle>(SummonTrigger, Summon)
        .trans::<Hurt>(SummonTrigger, Summon)

        .trans::<Summon>(VulnerableTrigger, BeginVulnerable)
        .trans::<BeginVulnerable>(AlwaysTrigger, Vulnerable)
        .trans::<Vulnerable>(SummonTrigger, Summon)

        .trans::<Vulnerable>(HurtTrigger, Hurt)
        .trans::<Idle>(VulnerableTrigger, Vulnerable)

        .trans::<Vulnerable>(EnragedTrigger, BeginEnraged)
        .trans::<BeginEnraged>(AlwaysTrigger, AbilityStartup)


        // Enraged attacks
        .trans::<Idle>(RestTrigger, Rest)
        .trans::<Rest>(HurtTrigger, Hurt)
        .trans::<Rest>(DoneTrigger::Success, PickNextMove)
        .trans::<Hurt>(PickNextMoveTrigger, PickNextMove)

        .trans::<Idle>(ChargeLeftTrigger, Charge)
        .trans::<Charge>(DoneTrigger::Success, PickNextMove)
        .trans::<Idle>(ChargeRightTrigger, Charge)
        .trans::<Charge>(DoneTrigger::Success, PickNextMove)

        .trans::<Idle>(HoverTrigger, Hover)
        .trans::<Hover>(DoneTrigger::Success, PickNextMove)

        .trans::<Idle>(RelocateTrigger, Relocate)
        .trans::<Relocate>(DoneTrigger::Success, PickNextMove)

        .trans::<Idle>(SlamTrigger, Slam)
        .trans::<Slam>(DoneTrigger::Success, PickNextMove)

        .trans::<Idle>(BoomTrigger, Boom)
        .trans::<Boom>(DoneTrigger::Success, PickNextMove)


        .trans::<Idle>(TakeoffTrigger, Takeoff)
        .trans::<Takeoff>(DoneTrigger::Success, PickNextMove)

        .trans::<Idle>(LeapTrigger, Leap)
        .trans::<Leap>(DoneTrigger::Success, PickNextMove)

        .trans::<PickNextMove>(AlwaysTrigger, AbilityStartup)
        .trans::<AbilityStartup>(AlwaysTrigger, Idle)



        .trans::<Fall>(DeathTrigger, Die::default())
        .trans::<Idle>(DeathTrigger, Die::default())
        .trans::<Hurt>(DeathTrigger, Die::default())
        .trans::<Rest>(DeathTrigger, Die::default())
        .trans::<Leap>(DeathTrigger, Die::default())
        .trans::<Boom>(DeathTrigger, Die::default())
        .trans::<Relocate>(DeathTrigger, Die::default())
        .trans::<Charge>(DeathTrigger, Die::default())
        .trans::<Hover>(DeathTrigger, Die::default())
        .trans::<Slam>(DeathTrigger, Die::default())

        .trans::<Die>(NotTrigger(AlwaysTrigger), Die::default())
}




#[derive(Component, Copy, Clone, Reflect)]
pub struct Summon;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Vulnerable;

#[derive(Component, Copy, Clone, Reflect)]
pub struct BeginEnraged;

#[derive(Component, Copy, Clone, Reflect)]
pub struct BeginVulnerable;

#[derive(Component, Copy, Clone, Reflect)]
pub struct PickNextMove;

#[derive(Component, Copy, Clone, Reflect)]
pub struct AbilityStartup;


#[derive(Component, Copy, Clone, Reflect)]
pub struct Rest;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Charge;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Hover;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Relocate;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Leap;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Slam;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Boom;

#[derive(Component, Copy, Clone, Reflect)]
pub struct Takeoff;



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


#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct RestTrigger;

impl Trigger for RestTrigger {
    type Param<'w, 's> = Query<'w, 's, (&'static Boss, &'static BossStage)>;

    fn trigger(&self, _: Entity, boss: &Self::Param<'_, '_>) -> bool {
        if boss.is_empty() {
            return false;
        }

        let (boss, stage) = boss.single();

        if stage != &BossStage::Enraged {
            return false;
        }

        match ATTACK_SEQUENCE[boss.move_index] {
            EnragedAttackMove::Rest(_) => true,
            _ => false
        }
    }
}

#[derive(Copy, Clone, Reflect, FromReflect)]
pub struct PickNextMoveTrigger;

impl Trigger for PickNextMoveTrigger {
    type Param<'w, 's> = Query<'w, 's, (&'static Boss, &'static BossStage, &'static RestAbility)>;

    fn trigger(&self, _: Entity, boss: &Self::Param<'_, '_>) -> bool {
        if boss.is_empty() {
            return false;
        }

        let (boss, stage, rest) = boss.single();

        if stage != &BossStage::Enraged {
            return false;
        }

        match ATTACK_SEQUENCE[boss.move_index] {
            EnragedAttackMove::Rest(_) => {
                rest.timer.finished()
            },
            _ => false
        }
    }
}

attack_trigger!(ChargeLeftTrigger, EnragedAttackMove::ChargeLeft);
attack_trigger!(ChargeRightTrigger, EnragedAttackMove::ChargeRight);
attack_trigger!(HoverTrigger, EnragedAttackMove::Hover);
attack_trigger!(RelocateTrigger, EnragedAttackMove::RelocateRight);
attack_trigger!(LeapTrigger, EnragedAttackMove::Leap);
attack_trigger!(BoomTrigger, EnragedAttackMove::Boom);
attack_trigger!(SlamTrigger, EnragedAttackMove::Slam);
attack_trigger!(TakeoffTrigger, EnragedAttackMove::Takeoff);
