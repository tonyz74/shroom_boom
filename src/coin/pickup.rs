use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::coin::coin::{Coin, CoinMovement};
use crate::coin::drops::CoinHolder;
use crate::state::GameState;


pub fn register_pickup(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(collect_coins)
    );
}


#[derive(Component, Copy, Clone, Debug, Default)]
pub struct CoinCollector;


fn collect_coins(
    mut coins: Query<(&GlobalTransform, &Collider, &mut Coin, &mut CoinMovement)>,
    mut collectors: Query<&mut CoinHolder, With<CoinCollector>>,
    rapier: Res<RapierContext>
) {

    for (tf, collider, mut coin, mut coin_mov) in coins.iter_mut() {
        let pos = Vec2::new(
            tf.translation().x,
            tf.translation().y
        );

        rapier.intersections_with_shape(
            pos,
            Rot::default(),
            collider,
            QueryFilter {
                flags: QueryFilterFlags::ONLY_KINEMATIC,
                ..default()
            },
            |collision| {
                if let Ok(mut coin_holder) = collectors.get_mut(collision) {
                    if coin.collected {
                        return true;
                    }

                    coin_holder.total_value += coin.value;
                    coin_mov.picked_up = true;
                    coin.collected = true;
                }

                true
            }
        );
    }

}