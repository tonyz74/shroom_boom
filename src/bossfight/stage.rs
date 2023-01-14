use bevy::prelude::*;

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
            BossStage::Waiting => 200,

            BossStage::SummonEasy => 150,
            BossStage::VulnerableEasy => 150,

            BossStage::SummonMedium => 100,
            BossStage::VulnerableMedium => 100,

            BossStage::SummonHard => 50,
            BossStage::VulnerableHard => 50,

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
