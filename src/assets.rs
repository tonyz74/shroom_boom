use crate::{common::Anim, state::GameState};
use bevy::prelude::*;

use std::collections::HashMap;
use crate::ui::hud::PLAYER_HUD_DISPLAY_CHUNKS;

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

#[derive(Resource, Default, Debug)]
pub struct DandelionEnemyAssets {
    pub anims: HashMap<String, Anim>,
}

impl DandelionEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<DandelionEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/enemies/frown.png");

        // IDLE

        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);

        let idle_handle = texture_atlases.add(idle_atlas);

        assets.anims = HashMap::from([("IDLE".to_string(), Anim::new(idle_handle, 0.1))]);
    }
}

#[derive(Resource, Default, Debug)]
pub struct ExplosionAssets {
    pub anims: HashMap<String, Anim>
}

impl ExplosionAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<ExplosionAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/attacks/explosion.png");
        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);
        assets.anims = HashMap::from([("BOOM".to_string(), Anim::new(atlas_handle, 0.1))]);
    }
}



#[derive(Resource, Default, Debug)]
pub struct SporeAssets {
    pub anims: HashMap<String, Anim>
}

impl SporeAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<SporeAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/attacks/spore.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.anims = HashMap::from([("SPORE".to_string(), Anim::new(atlas_handle, 0.1))]);
    }
}


#[derive(Resource, Default, Debug)]
pub struct CoinAssets {
    pub anims: HashMap<String, Anim>
}

impl CoinAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<CoinAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("sprites/item/coin.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.anims = HashMap::from([("SPIN".to_string(), Anim::new(atlas_handle, 0.1))]);
    }
}

#[derive(Resource, Default, Debug)]
pub struct BossAssets {
    pub anims: HashMap<String, Anim>
}

impl BossAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<BossAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(128., 128.);
        let sheet = asset_server.load("sprites/enemies/boss/waiting.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.anims = HashMap::from([("WAIT".to_string(), Anim::new(atlas_handle, 0.1))]);
    }
}

#[derive(Resource, Default, Debug)]
pub struct IndicatorAssets {
    pub tr: Handle<Image>,
}

impl IndicatorAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut assets: ResMut<IndicatorAssets>,
    ) {
        let tr = asset_server.load("sprites/util/indicator_top_right.png");
        assets.tr = tr;
    }
}

#[derive(Resource, Default, Debug)]
pub struct UiAssets {
    pub health: Vec<Handle<Image>>,
    pub ammo: Vec<Handle<Image>>,
    pub coins: Handle<Image>,
    pub font: Handle<Font>,
    pub text_style: TextStyle
}

impl UiAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut assets: ResMut<UiAssets>,

    ) {
        let mut health_images = Vec::with_capacity(PLAYER_HUD_DISPLAY_CHUNKS + 1);
        for i in 0..=PLAYER_HUD_DISPLAY_CHUNKS {
            health_images.push(asset_server.load(format!("art/hud/health{}.png", i + 1)));
        }
        assets.health = health_images;

        let mut ammo_images = Vec::with_capacity(PLAYER_HUD_DISPLAY_CHUNKS + 1);
        for i in 0..=PLAYER_HUD_DISPLAY_CHUNKS {
            ammo_images.push(asset_server.load(format!("art/hud/ammo{}.png", i + 1)));
        }
        assets.ammo = ammo_images;

        assets.font = asset_server.load("fonts/FutilePro.ttf");
        assets.coins = asset_server.load("art/hud/coins.png");

        assets.text_style = TextStyle {
            font: assets.font.clone(),
            font_size: 24.0,
            color: Color::WHITE
        };
    }
}

#[derive(Resource, Default, Debug)]
pub struct ShopAssets {
    pub shopkeeper: Anim,
    pub tonics: Vec<Handle<Image>>,
    pub waters: Vec<Handle<Image>>,

    pub health_up: Handle<Image>,
    pub ammo_up: Handle<Image>,
    pub slash_up: Handle<Image>,
    pub dash_up: Handle<Image>,
    pub shoot_up: Handle<Image>
}

impl ShopAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<ShopAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(180., 148.);
        let sheet = asset_server.load("sprites/shop/shopkeeper.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 1, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.shopkeeper = Anim {
            tex: atlas_handle,
            speed: 0.2
        };

        assets.tonics = vec![
            asset_server.load("art/shop/OddTonic.png"),
            asset_server.load("art/shop/StrangeTonic.png"),
            asset_server.load("art/shop/SuspiciousTonic.png"),
        ];

        assets.waters = vec![
            asset_server.load("art/shop/CupOfWater.png"),
            asset_server.load("art/shop/BucketOfWater.png"),
            asset_server.load("art/shop/TankOfWater.png"),
        ];

        assets.health_up = asset_server.load("art/shop/HealthUp.png");
        assets.ammo_up = asset_server.load("art/shop/AmmoUp.png");
        assets.slash_up = asset_server.load("art/shop/SlashUp.png");
        assets.dash_up = asset_server.load("art/shop/DashUp.png");
        assets.shoot_up = asset_server.load("art/shop/ShootUp.png");
    }
}


pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerAssets>()
            .init_resource::<FlowerEnemyAssets>()
            .init_resource::<PumpkinEnemyAssets>()
            .init_resource::<DandelionEnemyAssets>()
            .init_resource::<ExplosionAssets>()
            .init_resource::<SporeAssets>()
            .init_resource::<CoinAssets>()
            .init_resource::<BossAssets>()
            .init_resource::<IndicatorAssets>()
            .init_resource::<UiAssets>()
            .init_resource::<ShopAssets>()

            .add_state(GameState::AssetLoading)
            .add_startup_system_set(
                SystemSet::new()
                    .label("assets")
                    .with_system(PlayerAssets::load)
                    .with_system(FlowerEnemyAssets::load)
                    .with_system(PumpkinEnemyAssets::load)
                    .with_system(DandelionEnemyAssets::load)
                    .with_system(ExplosionAssets::load)
                    .with_system(SporeAssets::load)
                    .with_system(CoinAssets::load)
                    .with_system(BossAssets::load)
                    .with_system(IndicatorAssets::load)
                    .with_system(UiAssets::load)
                    .with_system(ShopAssets::load)
            )

            .add_startup_system(enter_main_menu.after("assets"));
    }
}

fn enter_main_menu(mut state: ResMut<State<GameState>>) {
    state.overwrite_set(GameState::MainMenu).unwrap();
}
