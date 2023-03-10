use bevy::prelude::*;
use crate::entity_states::*;
use crate::player::consts::AMMO_LEVELS;
use crate::player::Player;
use crate::state::GameState;

#[derive(Component, Copy, Clone, Debug)]
pub struct Ammo {
    pub rounds_left: u32,
    pub max_rounds: u32
}

impl Default for Ammo {
    fn default() -> Self {
        Self {
            rounds_left: AMMO_LEVELS[0] as u32,
            max_rounds: AMMO_LEVELS[0] as u32
        }
    }
}

pub fn register_ammo(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(lower_ammo_on_shoot)
    );
}

fn lower_ammo_on_shoot(
    mut player: Query<&mut Ammo, (Added<Shoot>, With<Player>)>
) {
    if player.is_empty() {
        return;
    }

    let mut ammo = player.single_mut();
    ammo.rounds_left -= 1;
}