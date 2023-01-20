use std::time::Duration;
use bevy::prelude::*;
use crate::combat::Health;
use crate::player::abilities::dash::DashAbility;
use crate::player::abilities::shoot::ShootAbility;
use crate::player::abilities::slash::SlashAbility;
use crate::player::ammo::Ammo;
use crate::player::consts::{AMMO_LEVELS, DASH_LEVELS, HEALTH_LEVELS, SHOOT_LEVELS, SLASH_LEVELS};
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

    let health_lvl = lvls.health_lvl as usize;
    health.max_hp = HEALTH_LEVELS[health_lvl];

    let ammo_lvl = lvls.ammo_lvl as usize;
    ammo.max_rounds = AMMO_LEVELS[ammo_lvl] as u32;

    let dash_lvl = lvls.dash_lvl as usize;
    dash.cd.set_duration(Duration::from_secs_f32(DASH_LEVELS[dash_lvl].0));
    dash.speed = DASH_LEVELS[dash_lvl].1;
    dash.damage = DASH_LEVELS[dash_lvl].2;

    let slash_lvl = lvls.slash_lvl as usize;
    slash.cd.set_duration(Duration::from_secs_f32(SLASH_LEVELS[slash_lvl].0));
    slash.damage = SLASH_LEVELS[slash_lvl].1;

    let shoot_lvl = lvls.shoot_lvl as usize;
    shoot.cd.set_duration(Duration::from_secs_f32(SHOOT_LEVELS[shoot_lvl].0));
    shoot.proj_speed = SHOOT_LEVELS[shoot_lvl].1;
    shoot.damage = SHOOT_LEVELS[shoot_lvl].2;
}