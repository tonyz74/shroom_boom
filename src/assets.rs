use crate::state::GameState;

use bevy::prelude::*;

use std::collections::HashMap;
use crate::anim::Animation;
use crate::anim::map::AnimationMap;
use crate::ui::hud::PLAYER_HUD_DISPLAY_CHUNKS;

#[derive(Resource, Default, Debug)]
pub struct PlayerAssets {
    pub anims: AnimationMap,
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

        // SLASH
        let slash_sheet = asset_server.load("slash/slash longgg.png");
        let slash_atlas =
            TextureAtlas::from_grid(slash_sheet, Vec2::new(36.0, 24.0), 3, 1, None, None);
        let slash_handle = texture_atlases.add(slash_atlas);


        player_assets.anims = AnimationMap::new(HashMap::from([
            ("IDLE".to_string(), Animation::new("IDLE".to_string(), idle_handle, 0.2)),
            ("RUN".to_string(), Animation::new("RUN".to_string(), run_handle, 0.08)),
            ("CROUCH".to_string(), Animation::new("CROUCH".to_string(), crouch_handle, 0.4)),
            ("SLASH".to_string(), Animation::new("SLASH".to_string(), slash_handle, 0.05)),
        ]));

    }
}

#[derive(Resource, Default, Debug)]
pub struct FlowerEnemyAssets {
    pub map: AnimationMap,
}

impl FlowerEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<FlowerEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/enemies/Flower-Sheet.png");

        let mut anims = HashMap::new();

        // IDLE
        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let idle_handle = texture_atlases.add(idle_atlas);
        let idle_anim = Animation::new("IDLE".to_string(), idle_handle, 0.75);
        anims.insert(idle_anim.name.clone(), idle_anim);

        // RUN
        let move_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(2.0, 0.0) * SIZE));
        let move_handle = texture_atlases.add(move_atlas);
        let move_anim = Animation::new("MOVE".to_string(), move_handle, 0.1);
        anims.insert(move_anim.name.clone(), move_anim);

        // DETONATE
        let detonate_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(9.0, 0.0) * SIZE));
        let detonate_handle = texture_atlases.add(detonate_atlas);
        let detonate_anim = Animation::new("DETONATE".to_string(), detonate_handle, 0.1);
        anims.insert(detonate_anim.name.clone(), detonate_anim);

        // DEATH
        let death_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(16.0, 0.0) * SIZE));
        let death_handle = texture_atlases.add(death_atlas);
        let death_anim = Animation::new("DEATH".to_string(), death_handle, 0.1);
        anims.insert(death_anim.name.clone(), death_anim);


        assets.map = AnimationMap::new(anims);
    }
}

#[derive(Resource, Default, Debug)]
pub struct PumpkinEnemyAssets {
    pub map: AnimationMap,
    pub bullet: Animation
}

impl PumpkinEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<PumpkinEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/enemies/Pumpkin-Sheet.png");

        let mut anims = HashMap::new();

        // IDLE
        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let idle_handle = texture_atlases.add(idle_atlas);
        let idle_anim = Animation::new("IDLE".to_string(), idle_handle.clone(), 0.75);
        anims.insert(idle_anim.name.clone(), idle_anim);

        let shoot_wait_anim = Animation::new("SHOOT_WAIT".to_string(), idle_handle.clone(), 0.75);
        anims.insert(shoot_wait_anim.name.clone(), shoot_wait_anim);

        // MOVE
        let move_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(2.0, 0.0) * SIZE));
        let move_handle = texture_atlases.add(move_atlas);
        let move_anim = Animation::new("MOVE".to_string(), move_handle, 0.07);
        anims.insert(move_anim.name.clone(), move_anim);

        // SHOOT
        let shoot_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 6, 1, None, Some(Vec2::new(9.0, 0.0) * SIZE));
        let shoot_handle = texture_atlases.add(shoot_atlas);
        let shoot_anim = Animation::new("SHOOT".to_string(), shoot_handle, 0.075);
        anims.insert(shoot_anim.name.clone(), shoot_anim);

        // DEATH
        let death_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(14.0, 0.0) * SIZE));
        let death_handle = texture_atlases.add(death_atlas);
        let death_anim = Animation::new("DEATH".to_string(), death_handle, 0.1);
        anims.insert(death_anim.name.clone(), death_anim);

        assets.map = AnimationMap::new(anims);



        const BULLET_SIZE: Vec2 = Vec2::new(16., 16.);
        let seed_sheet = asset_server.load("art/enemies/PumpkinSeed.png");
        let seed_atlas = TextureAtlas::from_grid(seed_sheet.clone(), BULLET_SIZE, 1, 1, None, None);
        let seed_handle = texture_atlases.add(seed_atlas);
        let seed_anim = Animation::new("SEED".to_string(), seed_handle, 1.0);
        assets.bullet = seed_anim;
    }
}

#[derive(Resource, Default, Debug)]
pub struct DandelionEnemyAssets {
    pub map: AnimationMap,
}

impl DandelionEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<DandelionEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/enemies/Dandelion-Sheet.png");
        
        let mut anims = HashMap::new();

        // IDLE
        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let idle_handle = texture_atlases.add(idle_atlas);
        let idle_anim = Animation::new("IDLE".to_string(), idle_handle.clone(), 0.75);
        anims.insert(idle_anim.name.clone(), idle_anim);
        
        // MOVE
        let move_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 9, 1, None, Some(Vec2::new(2.0, 0.0) * SIZE));
        let move_handle = texture_atlases.add(move_atlas);
        let move_anim = Animation::new("MOVE".to_string(), move_handle.clone(), 0.1);
        anims.insert(move_anim.name.clone(), move_anim);
        
        // DEATH 
        let death_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(11.0, 0.0) * SIZE));
        let death_handle = texture_atlases.add(death_atlas);
        let death_anim = Animation::new("DEATH".to_string(), death_handle.clone(), 0.1);
        anims.insert(death_anim.name.clone(), death_anim);

        assets.map = AnimationMap::new(anims);
    }
}



#[derive(Resource, Default, Debug)]
pub struct TumbleweedEnemyAssets {
    pub map: AnimationMap,
}

impl TumbleweedEnemyAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<TumbleweedEnemyAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/enemies/Tumbleweed-Sheet.png");

        let mut anims = HashMap::new();

        // IDLE
        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let idle_handle = texture_atlases.add(idle_atlas);
        let idle_anim = Animation::new("IDLE".to_string(), idle_handle.clone(), 0.75);
        anims.insert(idle_anim.name.clone(), idle_anim);

        // MOVE
        let move_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 4, 1, None, Some(Vec2::new(2.0, 0.0) * SIZE));
        let move_handle = texture_atlases.add(move_atlas);
        let move_anim = Animation::new("MOVE".to_string(), move_handle.clone(), 0.1);
        anims.insert(move_anim.name.clone(), move_anim);

        // DEATH
        let death_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(6.0, 0.0) * SIZE));
        let death_handle = texture_atlases.add(death_atlas);
        let death_anim = Animation::new("DEATH".to_string(), death_handle.clone(), 0.1);
        anims.insert(death_anim.name.clone(), death_anim);

        assets.map = AnimationMap::new(anims);
    }
}

#[derive(Resource, Default, Debug)]
pub struct ExplosionAssets {
    pub anims: HashMap<String, Animation>
}

impl ExplosionAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<ExplosionAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/misc/Explosion-Sheet.png");
        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, None);

        let atlas_handle = texture_atlases.add(atlas);
        assets.anims = HashMap::from([("BOOM".to_string(), Animation::new("BOOM".to_string(), atlas_handle, 0.08))]);
    }
}



#[derive(Resource, Default, Debug)]
pub struct SporeAssets {
    pub anims: HashMap<String, Animation>
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

        assets.anims = HashMap::from([("SPORE".to_string(), Animation::new("SPORE".to_string(), atlas_handle, 0.1))]);
    }
}


#[derive(Resource, Default, Debug)]
pub struct CoinAssets {
    pub spin: Animation
}

impl CoinAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<CoinAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(16., 16.);
        let sheet = asset_server.load("art/misc/Coin-Sheet.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 10, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.spin = Animation::new("SPIN".to_string(), atlas_handle, 0.08);
    }
}

#[derive(Resource, Default, Debug)]
pub struct BossAssets {
    pub anims: AnimationMap
}

impl BossAssets {
    pub fn load(
        asset_server: Res<AssetServer>,
        mut texture_atlases: ResMut<Assets<TextureAtlas>>,
        mut assets: ResMut<BossAssets>,
    ) {
        const SIZE: Vec2 = Vec2::new(64., 64.);
        let sheet = asset_server.load("art/enemies/Shroom-Sheet.png");
        let mut anims = HashMap::new();

        // IDLE
        let idle_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let idle_atlas_handle = texture_atlases.add(idle_atlas);
        let idle_anim = Animation::new("IDLE".to_string(), idle_atlas_handle, 0.75);
        anims.insert(idle_anim.name.to_string(), idle_anim.clone());

        // BOOM
        let boom_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 13, 1, None, Some(Vec2::new(26.0, 0.0) * SIZE));
        let boom_atlas_handle = texture_atlases.add(boom_atlas);
        let mut boom_anim = Animation::new("BOOM".to_string(), boom_atlas_handle, 0.1);
        boom_anim.repeating = false;
        anims.insert(boom_anim.name.to_string(), boom_anim.clone());

        // RETRACT
        let retract_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 8, 1, None, Some(Vec2::new(2.0, 0.0) * SIZE));
        let retract_atlas_handle = texture_atlases.add(retract_atlas);
        let mut retract_anim = Animation::new("RETRACT".to_string(), retract_atlas_handle, 0.08);
        retract_anim.repeating = false;
        anims.insert(retract_anim.name.to_string(), retract_anim.clone());

        // EXTEND
        let extend_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 5, 1, None, Some(Vec2::new(10.0, 0.0) * SIZE));
        let extend_atlas_handle = texture_atlases.add(extend_atlas);
        let mut extend_anim = Animation::new("EXTEND".to_string(), extend_atlas_handle, 0.06);
        extend_anim.repeating = false;
        anims.insert(extend_anim.name.to_string(), extend_anim.clone());

        // SLAM
        let slam_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 6, 1, None, Some(Vec2::new(15.0, 0.0) * SIZE));
        let slam_atlas_handle = texture_atlases.add(slam_atlas);
        let mut slam_anim = Animation::new("SLAM".to_string(), slam_atlas_handle, 0.04);
        slam_anim.repeating = false;
        anims.insert(slam_anim.name.to_string(), slam_anim.clone());

        // FLY
        let fly_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 5, 1, None, Some(Vec2::new(21.0, 0.0) * SIZE));
        let fly_atlas_handle = texture_atlases.add(fly_atlas);
        let mut fly_anim = Animation::new("FLY".to_string(), fly_atlas_handle, 0.1);
        fly_anim.repeating = false;
        anims.insert(fly_anim.name.to_string(), fly_anim.clone()); 
        
        // LEAP
        let leap_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 4, 1, None, Some(Vec2::new(21.0, 0.0) * SIZE));
        let leap_atlas_handle = texture_atlases.add(leap_atlas);
        let mut leap_anim = Animation::new("LEAP".to_string(), leap_atlas_handle, 0.1);
        leap_anim.repeating = false;
        anims.insert(leap_anim.name.to_string(), leap_anim.clone());

        // DEATH
        let death_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 7, 1, None, Some(Vec2::new(39.0, 0.0) * SIZE));
        let death_atlas_handle = texture_atlases.add(death_atlas);
        let death_anim = Animation::new("DEATH".to_string(), death_atlas_handle, 0.1);
        anims.insert(death_anim.name.to_string(), death_anim.clone());


        // SUMMON
        let summon_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 6, 1, None, Some(Vec2::new(46.0, 0.0) * SIZE));
        let summon_atlas_handle = texture_atlases.add(summon_atlas);
        let mut summon_anim = Animation::new("SUMMON".to_string(), summon_atlas_handle, 0.1);
        summon_anim.repeating = false; 
        anims.insert(summon_anim.name.to_string(), summon_anim.clone());

        // VULNERABLE
        let vulnerable_atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 3, 1, None, Some(Vec2::new(52.0, 0.0) * SIZE));
        let vulnerable_atlas_handle = texture_atlases.add(vulnerable_atlas);
        let mut vulnerable_anim = Animation::new("VULNERABLE".to_string(), vulnerable_atlas_handle, 0.2);
        vulnerable_anim.repeating = false;
        anims.insert(vulnerable_anim.name.to_string(), vulnerable_anim.clone());

        assets.anims = AnimationMap::new(anims);
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
    pub shopkeeper: Animation,
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
        const SIZE: Vec2 = Vec2::new(32., 32.);
        let sheet = asset_server.load("art/shop/Shopkeeper-Sheet.png");

        let atlas = TextureAtlas::from_grid(sheet.clone(), SIZE, 2, 1, None, None);
        let atlas_handle = texture_atlases.add(atlas);

        assets.shopkeeper = Animation {
            tex: atlas_handle,
            speed: 0.75,
            ..default()
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
            .init_resource::<TumbleweedEnemyAssets>()
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
                    .with_system(TumbleweedEnemyAssets::load)
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
