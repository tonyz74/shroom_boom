use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, PartialEq)]
pub enum ShopItem {
    OddTonicItem,
    StrangeTonicItem,
    BizarreTonicItem,
    WaterCupItem,
    WaterBucketItem,
    WaterTankItem,

    HealthUpgrade,
    AmmoUpgrade,
    SlashUpgrade,
    DashUpgrade,
    ShootUpgrade
}


pub const SHOP_CATALOG_ALL: &[ShopItem] = &[
    ShopItem::OddTonicItem,
    ShopItem::StrangeTonicItem,
    ShopItem::BizarreTonicItem,
    ShopItem::WaterCupItem,
    ShopItem::WaterBucketItem,
    ShopItem::WaterTankItem,

    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];

pub const SHOP_CATALOG_ITEMS: &[ShopItem] = &[
    ShopItem::OddTonicItem,
    ShopItem::StrangeTonicItem,
    ShopItem::BizarreTonicItem,
    ShopItem::WaterCupItem,
    ShopItem::WaterBucketItem,
    ShopItem::WaterTankItem
];

pub const SHOP_CATALOG_UPGRADES: &[ShopItem] = &[
    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];