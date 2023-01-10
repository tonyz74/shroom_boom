use bevy::prelude::*;
use rand::prelude::*;
use crate::state::GameState;
use crate::entity_states::Die;


#[derive(Copy, Clone, Debug, Component, Default)]
pub struct CoinDrops {
    pub amount: i32
}

pub fn register_drops(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(drop_coins_on_death)
    );
}


fn drop_coins_on_death(
    dead: Query<&CoinDrops, Added<Die>>
) {
    for drop in dead.iter() {
        println!("dropping {:?}", drop);
    }
}

