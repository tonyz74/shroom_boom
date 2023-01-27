use bevy::prelude::*;
use crate::coin::drops::CoinHolder;
use crate::combat::Health;
use crate::player::ammo::Ammo;
use crate::player::Player;
use crate::player::skill::PlayerSkillLevels;
use crate::shop::{ShopPurchaseEvent};
use crate::shop::stock::ShopItem;

pub fn shop_apply_purchases(
    mut coins: Query<&mut CoinHolder, With<Player>>,
    mut events: EventReader<ShopPurchaseEvent>,
    mut stats: Query<(
        &mut Health,
        &mut Ammo,
        &mut PlayerSkillLevels
    ), With<Player>>,
) {
    if coins.is_empty() || stats.is_empty() {
        return;
    }

    let mut allowance = coins.single_mut();
    let (mut health, mut ammo, mut skills) = stats.single_mut();

    for buy in events.iter() {
        if buy.cost > allowance.total_value {
            continue;
        }

        allowance.total_value -= buy.cost;
        
        use ShopItem as Item;

        match buy.order {
            // Items
            Item::OddTonicItem | Item::StrangeTonicItem | Item::SuspiciousTonicItem => {
                let hp = match buy.order {
                    Item::OddTonicItem => 10,
                    Item::StrangeTonicItem => 15,
                    Item::SuspiciousTonicItem => 25,
                    _ => panic!()
                };

                health.hp = (health.hp + hp).clamp(0, health.max_hp);
            }
            Item::CupOfWaterItem | Item::BucketOfWaterItem | Item::TankOfWaterItem => {
                let rounds = match buy.order {
                    Item::CupOfWaterItem => 10,
                    Item::BucketOfWaterItem => 15,
                    Item::TankOfWaterItem => 25,
                    _ => panic!()
                };

                ammo.rounds_left = (ammo.rounds_left + rounds).clamp(0, ammo.max_rounds);
            }

            // UPGRADES
            Item::ShootUpgrade => {
                skills.shoot_lvl += 1;
            },
            Item::AmmoUpgrade => {
                skills.ammo_lvl += 1;
            },
            Item::DashUpgrade => {
                skills.dash_lvl += 1;
            },
            Item::SlashUpgrade => {
                skills.slash_lvl += 1;
            },
            Item::HealthUpgrade => {
                skills.health_lvl += 1
            }
        };
    }
}