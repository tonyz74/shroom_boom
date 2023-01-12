pub mod drops;
pub mod pickup;
pub mod coin;
pub mod state_machine;

use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

pub struct CoinPlugin;

impl Plugin for CoinPlugin {
    fn build(&self, app: &mut App) {
        coin::register_coin(app);
        pickup::register_pickup(app);
        drops::register_drops(app);

        app.add_system(print_coin_holder_values);
    }
}

fn print_coin_holder_values(
    input: Res<Input<KeyCode>>,
    holders: Query<(Entity, &drops::CoinHolder), With<pickup::CoinCollector>>
) {
    if !input.just_pressed(KeyCode::I) {
        return;
    }

    for (ent, holder) in holders.iter() {
        screen_print!("{:?} has {:?} coins", ent, holder);
    }

}
