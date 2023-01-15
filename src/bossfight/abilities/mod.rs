use bevy::prelude::*;

mod rest;
mod boom;
mod relocate;

pub use rest::RestAbility;
pub use boom::BoomAbility;
pub use relocate::RelocateAbility;

pub fn register_boss_abilities(app: &mut App) {
    rest::register_rest_ability(app);
    boom::register_boom_ability(app);
    relocate::register_relocate_ability(app);
}