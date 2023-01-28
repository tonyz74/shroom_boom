use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component, PartialEq)]
pub enum ShopItem {
    OddTonicItem,
    StrangeTonicItem,
    SuspiciousTonicItem,
    CupOfWaterItem,
    BucketOfWaterItem,
    TankOfWaterItem,

    HealthUpgrade,
    AmmoUpgrade,
    SlashUpgrade,
    DashUpgrade,
    ShootUpgrade
}


pub const SHOP_CATALOG_ALL: &[ShopItem] = &[
    ShopItem::OddTonicItem,
    ShopItem::StrangeTonicItem,
    ShopItem::SuspiciousTonicItem,
    ShopItem::CupOfWaterItem,
    ShopItem::BucketOfWaterItem,
    ShopItem::TankOfWaterItem,

    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];

pub const SHOP_CATALOG_ITEMS: &[ShopItem] = &[
    ShopItem::OddTonicItem,
    ShopItem::StrangeTonicItem,
    ShopItem::SuspiciousTonicItem,
    ShopItem::CupOfWaterItem,
    ShopItem::BucketOfWaterItem,
    ShopItem::TankOfWaterItem
];

pub const SHOP_CATALOG_UPGRADES: &[ShopItem] = &[
    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];