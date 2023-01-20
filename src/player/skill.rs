use bevy::prelude::*;
use crate::combat::Health;
use crate::player::abilities::dash::DashAbility;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::abilities::slash::SlashAbility;
use crate::player::ammo::Ammo;
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

    let _lvls = q.single();
}