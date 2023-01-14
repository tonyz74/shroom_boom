use bevy::prelude::*;

mod rest;
pub use rest::RestAbility;

pub fn register_boss_abilities(app: &mut App) {
    rest::register_rest_ability(app);
}