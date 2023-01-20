use bevy::prelude::*;
use crate::common::AnimTimer;
use crate::interact::Interact;
use crate::state::GameState;

#[derive(Clone, Copy, Debug)]
pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        let _ = app;
    }
}

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Shop;

#[derive(Clone, Bundle, Default)]
pub struct ShopBundle {
    pub shop: Shop,
    pub interact: Interact,
    pub anim_timer: AnimTimer,
    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}