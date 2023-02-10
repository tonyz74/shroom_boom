use bevy::prelude::*;
use rand::prelude::*;
use crate::assets::CoinAssets;
use crate::coin::coin::{Coin, CoinBundle, CoinMovement};
use crate::state::GameState;
use crate::entity_states::Die;
use crate::player::Player;


#[derive(Copy, Clone, Debug, Component, Default)]
pub struct CoinHolder {
    pub total_value: i32
}

pub fn register_drops(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(drop_coins_on_death)
    );
}

fn spawn_random_coin<R: Rng + ?Sized>(
    rng: &mut R,
    commands: &mut Commands,
    worth: i32,
    pos: Vec2,
    assets: &CoinAssets
) {
    let vel = Vec2::new(
        rng.gen_range(-2.0..2.0),
        rng.gen_range(1.0..2.0)
    );

    commands.spawn(CoinBundle {
        coin: Coin { value: worth, collected: false },
        coin_movement: CoinMovement { vel, ..default() },
        ..CoinBundle::new(pos, assets)
    });
}

fn drop_coins_on_death(
    mut commands: Commands,
    dead: Query<(&CoinHolder, &GlobalTransform), (Added<Die>, Without<Player>)>,
    assets: Res<CoinAssets>
) {
    for (drop, transform) in dead.iter() {
        let pos = Vec2::new(
            transform.translation().x,
            transform.translation().y
        );

        let mut rng = thread_rng();

        const COIN_SPLIT: usize = 5;

        let value_split = drop.total_value / (COIN_SPLIT as i32);
        let remaining = drop.total_value % (COIN_SPLIT as i32);

        for _ in 0..value_split {
            spawn_random_coin(&mut rng, &mut commands, COIN_SPLIT as i32, pos, &assets);
        }

        if remaining != 0 {
            spawn_random_coin(&mut rng, &mut commands, remaining, pos, &assets);
        }

        println!("dropping {:?}", drop);
    }
}