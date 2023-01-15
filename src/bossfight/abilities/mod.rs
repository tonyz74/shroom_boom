use bevy::prelude::*;

mod rest;
mod boom;

pub use rest::RestAbility;
pub use boom::BoomAbility;

pub fn register_boss_abilities(app: &mut App) {
    rest::register_rest_ability(app);
    boom::register_boom_ability(app);
}