use crate::{common::Anim, state::GameState};
use bevy::prelude::*;

use std::collections::HashMap;

#[derive(Resource, Default, Debug)]
pub struct PlayerAssets {
    pub anims: HashMap<String, Anim>,
    pub slash_anim: Anim,
}

impl PlayerAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut player_assets: ResMut<PlayerAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(24., 24.);
        let sheet = asset_server.load("dino/sheets/yellow.png");

        // IDLE

        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 4, 1, None, None);

        let idle_handle = texture_atlases.add(idle_atlas);

        // RUN

        let run_atlas = TextureAtlas::from_grid(
            sheet.clone(),
            SIZE,
            6,
            1,
            None,
            Some(Vec2::new(3., 0.) * SIZE),
        );

        let run_handle = texture_atlases.add(run_atlas);

        // CROUCH

        let crouch_atlas = TextureAtlas::from_grid(
            sheet.clone(),
            SIZE,
            1,
            1,
            None,
            Some(Vec2::new(17., 0.) * SIZE),
        );

        let crouch_handle = texture_atlases.add(crouch_atlas);

        player_assets.anims = HashMap::from([
            ("IDLE".to_string(), Anim::new(idle_handle, 0.2)),
            ("RUN".to_string(), Anim::new(run_handle, 0.08)),
            ("CROUCH".to_string(), Anim::new(crouch_handle, 0.4)),
        ]);

        // SLASH
        let slash_sheet = asset_server.load("slash/slash longgg.png");
        let slash_atlas =
            TextureAtlas::from_grid(slash_sheet, Vec2::new(36.0, 24.0), 3, 1, None, None);

        let slash_handle = texture_atlases.add(slash_atlas);

        player_assets.slash_anim = Anim::new(slash_handle, 0.05);
    }
}

#[derive(Resource, Default, Debug)]
pub struct FlowerEnemyAssets {
    pub anims: HashMap<String, Anim>,
}

impl FlowerEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<FlowerEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/enemies/smile.png");

        // IDLE

        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);

        let idle_handle = texture_atlases.add(idle_atlas);

        assets.anims = HashMap::from([("IDLE".to_string(), Anim::new(idle_handle, 0.1))]);
    }
}

#[derive(Resource, Default, Debug)]
pub struct PumpkinEnemyAssets {
    pub anims: HashMap<String, Anim>,
}

impl PumpkinEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<PumpkinEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/enemies/frown.png");

        // IDLE

        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);

        let idle_handle = texture_atlases.add(idle_atlas);

        assets.anims = HashMap::from([("IDLE".to_string(), Anim::new(idle_handle, 0.1))]);
    }
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAssets>()
            .init_resource::<FlowerEnemyAssets>()
            .init_resource::<PumpkinEnemyAssets>()
            .add_state(GameState::AssetLoading)
            .add_startup_system_set(
                SystemSet::new()
                    .label("assets")
                    .with_system(PlayerAssets::load)
                    .with_system(FlowerEnemyAssets::load)
                    .with_system(PumpkinEnemyAssets::load)
            )
            .add_startup_system(enter_level_transition.after("assets"));
    }
}

fn enter_level_transition(mut state: ResMut<State<GameState>>) {
    state.overwrite_set(GameState::LevelTransition).unwrap();
}
