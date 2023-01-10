use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::assets::CoinAssets;
use crate::common::AnimTimer;


pub fn register_coin(_app: &mut App) {
}



#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Coin {
    pub value: u32
}

#[derive(Bundle)]
pub struct CoinBundle {
    pub sensor: Sensor,
    pub rigid_body: RigidBody,
    pub anim_timer: AnimTimer,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle
}

impl CoinBundle {
    pub fn new(pos: Vec2, assets: &CoinAssets) -> Self {
        let anim = &assets.anims["SPIN"];

        Self {
            sensor: Sensor,

            rigid_body: RigidBody::Dynamic,

            anim_timer: AnimTimer::from_seconds(anim.speed),

            sprite_sheet: SpriteSheetBundle {

                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },

                texture_atlas: anim.tex.clone(),

                transform: Transform::from_xyz(pos.x, pos.y, 100.0),
                ..default()
            }
        }
    }
}