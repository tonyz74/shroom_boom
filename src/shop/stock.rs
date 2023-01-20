use bevy::prelude::*;

#[derive(Copy, Clone, Debug, Component)]
pub enum ShopItem {
    OddPopsicleItem,
    StrangeTonicItem,
    SuspiciousTonicItem,
    CupOfWaterItem,
    JugOfWaterItem,
    BucketOfWaterItem,

    HealthUpgrade,
    AmmoUpgrade,
    SlashUpgrade,
    DashUpgrade,
    ShootUpgrade
}


pub const SHOP_CATALOG_ALL: &[ShopItem] = &[
    ShopItem::OddPopsicleItem,
    ShopItem::StrangeTonicItem,
    ShopItem::SuspiciousTonicItem,
    ShopItem::CupOfWaterItem,
    ShopItem::JugOfWaterItem,
    ShopItem::BucketOfWaterItem,

    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];

pub const SHOP_CATALOG_ITEMS: &[ShopItem] = &[
    ShopItem::OddPopsicleItem,
    ShopItem::StrangeTonicItem,
    ShopItem::SuspiciousTonicItem,
    ShopItem::CupOfWaterItem,
    ShopItem::JugOfWaterItem,
    ShopItem::BucketOfWaterItem
];

pub const SHOP_CATALOG_UPGRADES: &[ShopItem] = &[
    ShopItem::HealthUpgrade,
    ShopItem::AmmoUpgrade,
    ShopItem::SlashUpgrade,
    ShopItem::DashUpgrade,
    ShopItem::ShootUpgrade
];