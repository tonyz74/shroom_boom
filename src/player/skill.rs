use bevy::prelude::*;
use crate::combat::Health;
use crate::player::abilities::dash::DashAbility;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::abilities::slash::SlashAbility;
use crate::player::ammo::Ammo;
use crate::player::consts::{AMMO_LEVELS, HEALTH_LEVELS};
use crate::player::Player;

#[derive(Copy, Clone, Default, Debug, Component, PartialEq)]
pub struct PlayerSkillLevels {
    pub dash_lvl: u8,
    pub slash_lvl: u8,
    pub shoot_lvl: u8,
    pub ammo_lvl: u8,
    pub health_lvl: u8,
}

pub fn upgrade_player_from_skills(
    q: Query<&PlayerSkillLevels, Changed<PlayerSkillLevels>>,
    mut stats: Query<(
        &mut Health,
        &mut Ammo,
        &mut ShootAbility,
        &mut SlashAbility,
        &mut DashAbility
    ), With<Player>>
) {
    if q.is_empty() || stats.is_empty() {
        return;
    }

    let lvls = q.single();
    let (mut health, mut ammo, mut shoot, mut slash, mut dash) = stats.single_mut();

    health.max_hp = HEALTH_LEVELS[lvls.health_lvl as usize];
    ammo.max_rounds = AMMO_LEVELS[lvls.ammo_lvl as usize] as u32;
}