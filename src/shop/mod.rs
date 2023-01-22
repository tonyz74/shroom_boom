pub mod stock;
pub mod info;
pub mod purchase;

use bevy::prelude::*;
use crate::assets::{ShopAssets, UiAssets};
use crate::interact::Interact;
use crate::shop::stock::ShopItem;
use crate::state::GameState;
use crate::anim::AnimationPlayer;

#[derive(Clone, Copy, Debug)]
pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShopPurchaseEvent>();

        app.add_system_set(
            SystemSet::new()
                .with_system(shop_open_menu)
                .with_system(purchase::shop_apply_purchases)
        );
    }
}




#[derive(Resource, Copy, Clone, Debug, Component)]
pub struct ShopPurchaseEvent {
    pub cost: i32,
    pub order: ShopItem
}

#[derive(Component, Clone, Debug, Default)]
pub struct Shop {
    pub orders: Vec<ShopItem>
}

#[derive(Clone, Bundle, Default)]
pub struct ShopBundle {
    pub shop: Shop,
    pub interact: Interact,
    pub anim: AnimationPlayer,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl ShopBundle {
    pub fn new(assets: &ShopAssets, ui_assets: &UiAssets, pos: Vec2) -> Self {
        ShopBundle {
            sprite_sheet: SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    custom_size: Some(Vec2::new(180.0, 148.0)),
                    ..default()
                },
                texture_atlas: assets.shopkeeper.tex.clone(),
                transform: Transform::from_translation(pos.extend(0.0)),
                ..default()
            },
            anim: AnimationPlayer::new(assets.shopkeeper.clone()),
            interact: Interact {
                content: Text::from_section("See stock [E]", ui_assets.text_style.clone()),
                max_dist: 256.0,
                text_offset: Vec2::new(0.0, 96.0),
                ..default()
            },
            ..default()
        }
    }
}


fn shop_open_menu(
    mut state: ResMut<State<GameState>>,
    q: Query<&Interact, With<Shop>>
) {
    for interact in q.iter() {
        if interact.interacted_with() && state.current() != &GameState::ShopMenu {
            state.push(GameState::ShopMenu).unwrap();
        }
    }
}