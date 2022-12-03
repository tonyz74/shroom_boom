use bevy::prelude::*;
use crate::{
    common::Anim,
    state::GameState
};

use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct PlayerAssets {
    pub anims: HashMap<String, Anim>,
    pub slash_anim: Anim
}

impl PlayerAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut player_assets: ResMut<PlayerAssets>
    ) {

        const SIZE: Vec2 = Vec2::new(24., 24.);
        let sheet = asset_server.load("dino/sheets/yellow.png");

        // IDLE

        let idle_atlas = TextureAtlas::from_grid(
            sheet.clone(),
            SIZE,
            4, 1,
            None,
            None
        );

        let idle_handle = texture_atlases.add(idle_atlas);

        // RUN

        let run_atlas = TextureAtlas::from_grid(
            sheet.clone(),
            SIZE,
            6, 1,
            None,
            Some(Vec2::new(3., 0.) * SIZE),
        );

        let run_handle = texture_atlases.add(run_atlas);


        player_assets.anims = HashMap::from([
            ("IDLE".to_string(), Anim::new(idle_handle, 0.2)),
            ("RUN".to_string(), Anim::new(run_handle, 0.08))
        ]);


        // SLASH
        let slash_sheet = asset_server.load("slash/slash longgg.png");
        let slash_atlas = TextureAtlas::from_grid(
            slash_sheet,
            Vec2::new(36.0, 24.0),
            3, 1,
            None,
            None
        );

        let slash_handle = texture_atlases.add(slash_atlas);

        player_assets.slash_anim = Anim::new(slash_handle, 0.05);
    }
}


pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAssets>()
            .add_state(GameState::AssetLoading)

            .add_startup_system_set(
                SystemSet::new()
                    .label("assets")
                    .with_system(PlayerAssets::load)
            )

            .add_startup_system(enter_gameplay.after("assets"));
    }
}

fn enter_gameplay(mut state: ResMut<State<GameState>>) {
    state.overwrite_set(GameState::Gameplay).unwrap();
}
