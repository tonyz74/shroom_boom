use bevy::prelude::*;

pub mod spore;
pub mod indicator;
pub mod shake;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        spore::register_spore_particles(app);
        indicator::register_indicators(app);
        shake::register_screen_shake(app);
    }
}