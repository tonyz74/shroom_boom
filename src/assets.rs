use bevy::prelude::*;
use crate::state::GameState;

use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct PlayerAssets {
    pub sprite_sheets: HashMap<String, (Handle<TextureAtlas>, f32)>
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
            None);

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


        player_assets.sprite_sheets = HashMap::from([
            ("IDLE".to_string(), (idle_handle, 0.2)),
            ("RUN".to_string(), (run_handle, 0.08))
        ]);
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
