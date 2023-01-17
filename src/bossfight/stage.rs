use bevy::prelude::*;
use crate::bossfight::consts::{BOSS_EASY_HEALTH_THRESHOLD, BOSS_HARD_HEALTH_THRESHOLD, BOSS_HEALTH, BOSS_MEDIUM_HEALTH_THRESHOLD};

#[derive(Component, Copy, Clone, Reflect, PartialEq, Eq, Debug)]
pub enum BossStage {
    Waiting,
    SummonEasy,
    VulnerableEasy,
    SummonMedium,
    VulnerableMedium,
    SummonHard,
    VulnerableHard,
    Enraged,
}

impl Default for BossStage {
    fn default() -> Self {
        Self::Waiting
    }
}

impl BossStage {
    pub fn advance(&mut self) {
        *self = self.next();
    }

    pub fn next(self) -> Self {
        match self {
            BossStage::Waiting => BossStage::SummonEasy,
            BossStage::SummonEasy => BossStage::VulnerableEasy,
            BossStage::VulnerableEasy => BossStage::SummonMedium,
            BossStage::SummonMedium => BossStage::VulnerableMedium,
            BossStage::VulnerableMedium => BossStage::SummonHard,
            BossStage::SummonHard => BossStage::VulnerableHard,
            BossStage::VulnerableHard => BossStage::Enraged,
            BossStage::Enraged => panic!("No stage to advance")
        }
    }

    pub fn is_summon_stage(&self) -> bool {
        match self {
            BossStage::SummonEasy |
            BossStage::SummonMedium |
            BossStage::SummonHard => true,

            _ => false
        }
    }

    pub fn is_vulnerable_stage(&self) -> bool {
        match self {
            BossStage::VulnerableEasy |
            BossStage::VulnerableMedium |
            BossStage::VulnerableHard => true,

            _ => false
        }
    }

    pub const fn health_threshold(&self) -> i32 {
        match self {
            BossStage::Waiting => BOSS_HEALTH,

            BossStage::SummonEasy => BOSS_EASY_HEALTH_THRESHOLD,
            BossStage::VulnerableEasy => BOSS_EASY_HEALTH_THRESHOLD,

            BossStage::SummonMedium => BOSS_MEDIUM_HEALTH_THRESHOLD,
            BossStage::VulnerableMedium => BOSS_MEDIUM_HEALTH_THRESHOLD,

            BossStage::SummonHard => BOSS_HARD_HEALTH_THRESHOLD,
            BossStage::VulnerableHard => BOSS_HARD_HEALTH_THRESHOLD,

            BossStage::Enraged => 0
        }
    }

    pub fn from_health(health: i32) -> Self {
        let choices = [
            BossStage::Waiting,
            BossStage::SummonEasy,
            BossStage::VulnerableEasy,
            BossStage::SummonMedium,
            BossStage::VulnerableMedium,
            BossStage::SummonHard,
            BossStage::VulnerableHard,
            BossStage::Enraged
        ];

        for choice in choices {
            if health > choice.health_threshold() {
                return choice;
            }
        }

        panic!("Health does not match any state!");
    }
}
