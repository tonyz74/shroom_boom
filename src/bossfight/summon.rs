use bevy::prelude::*;
use crate::assets::FlowerEnemyAssets;
use crate::bossfight::Boss;
use crate::bossfight::state_machine::Summon;
use crate::combat::{ColliderAttack, Immunity};
use crate::enemies::flower::FlowerEnemyBundle;
use crate::level::consts::RENDERED_TILE_SIZE;
use crate::level::LevelInfo;
use crate::pathfind::Region;
use crate::state::GameState;



#[derive(Component, Debug, Clone, Copy)]
pub struct SummonedEnemy;

#[derive(Component, Debug, Clone, Copy)]
pub struct FinishedSummoning;


pub fn register_boss_summon(app: &mut App) {
    app.add_system_set(
        SystemSet::on_update(GameState::Gameplay)
            .with_system(enter_summon)
            .with_system(summon_enemies)
    );
}

fn enter_summon(
    mut collider_attacks: Query<&mut ColliderAttack>,
    mut q: Query<(&mut Immunity, &Children), (With<Boss>, Added<Summon>)>
) {
    for (mut immunity, children) in q.iter_mut() {
        immunity.is_immune = true;

        for child in children {
            if let Ok(mut atk) = collider_attacks.get_mut(*child) {
                atk.enabled = false;
            }
        }
    }
}

fn summon_enemies(
    mut commands: Commands,
    q: Query<Entity, (With<Boss>, Added<Summon>)>,

    flower_assets: Res<FlowerEnemyAssets>,
    lvl_info: Res<LevelInfo>
) {
    for e in q.iter() {
        let mut flower = FlowerEnemyBundle::from_assets(&flower_assets);
        flower.enemy.path.pathfinder.region = Region {
            tl: Vec2::new(0.0, lvl_info.grid_size.y * RENDERED_TILE_SIZE),
            br: Vec2::new(lvl_info.grid_size.x * RENDERED_TILE_SIZE, 0.0),
        };
        flower.enemy.sprite_sheet.transform.translation = Vec3::new(400.0, 200.0, 10.0);

        let id = FlowerEnemyBundle::spawn(&mut commands, flower);
        commands.entity(id).insert(SummonedEnemy);

        commands.entity(e).insert(FinishedSummoning);
    }
}