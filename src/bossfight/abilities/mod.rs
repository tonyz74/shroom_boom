use bevy::prelude::*;

mod rest;
mod boom;
mod relocate;
mod charge;
mod leap;

pub use rest::RestAbility;
pub use boom::BoomAbility;
pub use relocate::RelocateAbility;
pub use charge::ChargeAbility;
pub use leap::LeapAbility;

pub fn register_boss_abilities(app: &mut App) {
    rest::register_rest_ability(app);
    boom::register_boom_ability(app);
    charge::register_boom_ability(app);
    relocate::register_relocate_ability(app);
    leap::register_leap_ability(app);
}