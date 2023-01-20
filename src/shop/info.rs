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
    pub fn for_item(assets: &ShopAssets, order: ShopItem) -> Self {
        match order {
            // ITEMS

            ShopItem::CupOfWaterItem => Self {
                cost: 5,
                name: "Cup of Water",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::JugOfWaterItem => Self {
                cost: 10,
                name: "Jug of Water",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::BucketOfWaterItem => Self {
                cost: 20,
                name: "Bucket of Water",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::OddPopsicleItem => Self {
                cost: 5,
                name: "Odd Popsicle",
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

            ShopItem::MaxHealthUpgrade => Self {
                cost: 15,
                name: "Max Health",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::MaxAmmoUpgrade => Self {
                cost: 15,
                name: "Max Ammo",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::SlashUpgrade => Self {
                cost: 15,
                name: "Slash",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::DashUpgrade => Self {
                cost: 15,
                name: "Dash",
                icon: assets.tonics[0].clone(),
            },

            ShopItem::ShootUpgrade => Self {
                cost: 15,
                name: "Shoot",
                icon: assets.tonics[0].clone(),
            },
        }
    }
}