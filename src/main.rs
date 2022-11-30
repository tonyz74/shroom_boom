use bevy::prelude::*;
use shade::entry::ShadePlugin;

fn main() {
    App::new()
        .add_plugin(ShadePlugin)
        .run();
}

