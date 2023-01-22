use bevy::prelude::*;
use crate::assets::ShopAssets;
use crate::shop::stock::ShopItem;

#[derive(Clone, Component)]
pub struct ShopItemInfo {
    pub cost: i32,
    pub name: &'static str,
    pub icon: Handle<Image>,
}

impl ShopItemInfo {
    pub fn for_item(assets: &ShopAssets, order: ShopItem, lvl: Option<u8>) -> Self {
        match order {
            // ITEMS

            ShopItem::CupOfWaterItem => Self {
                cost: 5,
                name: "Cup of Water",
                icon: assets.waters[0].clone(),
            },

            ShopItem::BucketOfWaterItem => Self {
                cost: 10,
                name: "Bucket of Water",
                icon: assets.waters[1].clone(),
            },

            ShopItem::TankOfWaterItem => Self {
                cost: 20,
                name: "Tank of Water",
                icon: assets.waters[2].clone(),
            },

            ShopItem::OddTonicItem => Self {
                cost: 5,
                name: "Odd Tonic",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::StrangeTonicItem => Self {
                cost: 10,
                name: "Strange Tonic",
                icon: assets.tonics[1].clone(),
            },

            ShopItem::SuspiciousTonicItem => Self {
                cost: 20,
                name: "Suspicious Tonic",
                icon: assets.tonics[2].clone(),
            },

            // UPGRADES

            ShopItem::HealthUpgrade => Self {
                cost: cost_for_upgrading(lvl.unwrap()),
                name: "Health",
                icon: assets.health_up.clone(),
            },

            ShopItem::AmmoUpgrade => Self {
                cost: cost_for_upgrading(lvl.unwrap()),
                name: "Ammo",
                icon: assets.ammo_up.clone(),
            },

            ShopItem::SlashUpgrade => Self {
                cost: cost_for_upgrading(lvl.unwrap()),
                name: "Slash",
                icon: assets.slash_up.clone(),
            },

            ShopItem::DashUpgrade => Self {
                cost: cost_for_upgrading(lvl.unwrap()),
                name: "Dash",
                icon: assets.dash_up.clone(),
            },

            ShopItem::ShootUpgrade => Self {
                cost: cost_for_upgrading(lvl.unwrap()),
                name: "Shoot",
                icon: assets.shoot_up.clone(),
            },
        }
    }
}

fn cost_for_upgrading(lvl: u8) -> i32 {
    (lvl as i32 + 1) * 5
}