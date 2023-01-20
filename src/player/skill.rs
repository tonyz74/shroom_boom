use bevy::prelude::*;

#[derive(Copy, Clone, Default, Debug, Component, PartialEq)]
pub struct PlayerSkillLevels {
    pub dash_lvl: u8,
    pub slash_lvl: u8,
    pub shoot_lvl: u8,
    pub ammo_lvl: u8,
    pub health_lvl: u8,
}