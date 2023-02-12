use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use crate::assets::UiAssets;
use crate::input::PlayerControls;
use crate::interact::Interact;
use crate::level::{coord, LevelInfo, util};
use crate::state::GameState;

#[derive(Component, Copy, Clone, Default)]
pub struct HelpTextSpawnpointMarker;

#[derive(LdtkEntity, Bundle, Default)]
pub struct HelpTextSpawnpointBundle {
    marker: HelpTextSpawnpointMarker,
    #[from_entity_instance]
    inst: EntityInstance
}

pub fn register_tutorial_text(app: &mut App) {
    app.register_ldtk_entity::<HelpTextSpawnpointBundle>("HelpText")
        .add_system_set(
            SystemSet::on_update(GameState::LevelTransition)
                .with_system(spawn_tutorial_text)
        );
}

fn spawn_tutorial_text(
    mut commands: Commands,
    ui_assets: Res<UiAssets>,
    q: Query<&EntityInstance, Added<HelpTextSpawnpointMarker>>,
    lvl_info: Res<LevelInfo>,
    ctrl: Res<PlayerControls>
) {
    for inst in q.iter() {
        let pos_vec2 = coord::grid_coord_to_translation(
            inst.grid,
            lvl_info.grid_size.as_ivec2()
        );

        let mut content = util::val_expect_string(&inst.field_instances[0].value).unwrap();

        let replacements = &[
            ("MoveLeft", ctrl.move_left),
            ("MoveRight", ctrl.move_right),
            ("Jump", ctrl.jump),
            ("Crouch", ctrl.crouch),
            ("Slash", ctrl.slash),
            ("Shoot", ctrl.shoot),
            ("Dash", ctrl.dash),
        ];

        for (pattern, new) in replacements {
            content = content.replace(&format!("${{Controls.{}}}", pattern), &format!("{:?}", new));
        }

        commands.spawn(HelpTextBundle::new(&ui_assets, pos_vec2, content));
    }
}


#[derive(Component, Copy, Clone)]
pub struct HelpText;

#[derive(Bundle)]
pub struct HelpTextBundle {
    help: HelpText,
    interact: Interact,
    #[bundle]
    sprite: SpriteBundle
}

impl HelpTextBundle {
    fn new(ui_assets: &UiAssets, pos: Vec2, text: String) -> Self {
        let mut info_style = ui_assets.text_style.clone();
        info_style.font_size = 12.0;

        Self {
            help: HelpText,
            interact: Interact {
                content: Text::from_section(text, info_style),
                max_dist: 160.0,
                ..default()
            },
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                    ..default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 900.0),
                ..default()
            }
        }
    }
}