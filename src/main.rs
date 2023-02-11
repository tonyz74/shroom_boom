use bevy::prelude::*;
use shroom_boom::entry::ShroomBoomPlugin;

fn main() {
    App::new()
        .add_plugin(ShroomBoomPlugin)
        .run();
}

