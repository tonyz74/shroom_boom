use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::coin::coin::Coin;
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
    mut commands: Commands,
    coins: Query<(Entity, &GlobalTransform, &Collider, &Coin)>,
    mut collectors: Query<(&GlobalTransform, &mut CoinHolder), With<CoinCollector>>,
    rapier: Res<RapierContext>
) {

    for (entity, tf, collider, coin) in coins.iter() {
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
                if let Ok((tf, mut coin_holder)) = collectors.get_mut(collision) {
                    let collision_pos = Vec2::new(
                        tf.translation().x,
                        tf.translation().y
                    );

                    let _ = collision_pos;

                    coin_holder.total_value += coin.value;
                    commands.entity(entity).despawn();
                }

                true
            }
        );
    }

}