use bevy::prelude::*;
use shroom_boom::entry::ShadePlugin;

fn main() {
    App::new()
        .add_plugin(ShadePlugin)
        .run();
}

