pub mod drops;
pub mod pickup;
pub mod coin;

use bevy::prelude::*;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        coin::register_coin(app);
        pickup::register_pickup(app);
        drops::register_drops(app);
    }
}
