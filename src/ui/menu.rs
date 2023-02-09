use bevy::prelude::*;
use bevy_ecs_ldtk::ldtk::Level;
use bevy_ecs_ldtk::{LevelSelection, LevelSet};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::{CoinAssets, UiAssets};
use crate::bossfight::Boss;
use crate::coin::coin::Coin;
use crate::combat::{ExplosionAttack, ProjectileAttack};
use crate::enemies::Enemy;
use crate::fx::indicator::Indicator;
use crate::fx::smoke::Smoke;
use crate::level::transition::LevelTransition;
use crate::player::Player;
use crate::shop::Shop;

use crate::state::GameState;
use crate::ui::bossbar::BossBar;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::hud::Hud;
use crate::ui::style::{background_style, button_style};

#[derive(Component, Clone, PartialEq, Default)]
pub struct MainMenuProps {
}

impl Widget for MainMenuProps {
}

#[derive(Bundle)]
pub struct MainMenuBundle {
    pub props: MainMenuProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for MainMenuBundle {
    fn default() -> Self {
        Self {
            props: MainMenuProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: MainMenuProps::default().get_name(),
        }
    }
}


pub fn register_menu_ui_systems(app: &mut App) {
    app.add_system_set(
        SystemSet::on_enter(GameState::MainMenu)
            .with_system(menu_state_setup)
    ).add_system_set(
        SystemSet::new().with_system(goto_menu)
    ).add_system_set(
        SystemSet::on_exit(GameState::MainMenu).with_system(stop_attempting)
    ).init_resource::<GotoMenuEvent>();
}

pub fn menu_state_setup(
    mut commands: Commands,
    q: Query<Entity, Or<(
        With<Player>,
        With<Coin>,
        With<LevelSet>,
        With<Enemy>,
        With<Boss>,
        With<Collider>,
        With<RigidBody>,
        With<Shop>,
        With<Indicator>,
        With<Smoke>,
        With<ExplosionAttack>,
        With<ProjectileAttack>,
        With<BossBar>
    )>>,

    mut hud: ResMut<Hud>,
    mut trans: ResMut<LevelTransition>,
    mut sel: ResMut<LevelSelection>,
) {
    for e in q.iter() {
        if let Some(cmd) = commands.get_entity(e) {
            cmd.despawn_recursive();
        }
    }

    if let Some(cmd) = commands.get_entity(hud.entity) {
        if hud.entity != Entity::from_raw(0) {
            println!("despawning");
            cmd.despawn_recursive();
            hud.entity = Entity::from_raw(0);
        }
    }

    trans.next = 0;
    *sel = LevelSelection::Index(0);
}


#[derive(Clone, Copy, Resource, Debug, Default)]
pub struct GotoMenuEvent {
    pub attempt: bool
}

fn stop_attempting(mut goto: ResMut<GotoMenuEvent>) {
    goto.attempt = false;
}

pub fn goto_menu(mut state: ResMut<State<GameState>>, goto: Res<GotoMenuEvent>) {
     if goto.attempt {
        if let Err(e) = state.set(GameState::MainMenu) {
            // error!("State transition: {:?}", e);
            let _ = e;
        }
    }
}


pub fn register_menu_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<MainMenuProps, EmptyState>();

    widget_context.add_widget_system(
        MainMenuProps::default().get_name(),
        widget_update::<MainMenuProps, EmptyState>,
        main_menu_render,
    );
}

pub fn main_menu_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    assets: Res<UiAssets>
) -> bool {
    let button_styles = button_style();

    let title_styles = KStyle {
        top: StyleProp::Value(Units::Pixels(0.0)),
        bottom: StyleProp::Value(Units::Percentage(16.0)),
        font_size: StyleProp::Value(40.0),
        ..default()
    };

    let image_styles = KStyle {
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),

        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),

        width: StyleProp::Value(Units::Pixels(320.0)),
        height: StyleProp::Value(Units::Pixels(384.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(20.0)),

        position_type: StyleProp::Value(KPositionType::SelfDirected),

        ..default()
    };

    let background_styles = KStyle {
        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),

        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),

        width: StyleProp::Value(Units::Pixels(384.0)),
        height: StyleProp::Value(Units::Pixels(400.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(20.0)),

        ..default()
    };

    let click_quit = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): EventInput| {
            match event.event_type {
                EventType::Click(_) => {
                    std::process::exit(0);
                }
                _ => {}
            }

            (event_dispatcher_context, event)
        }
    );

    let click_new_game = goto_state_event(StateTransition::Set(GameState::LevelTransition));

    let parent_id = Some(entity);

    rsx! {
        <BackgroundBundle styles={background_styles.clone()}>
            <KImageBundle
                image={KImage(assets.pause_bg.clone())}
                styles={image_styles}
            />

            <BackgroundBundle styles={background_styles.clone()}>
                <TextWidgetBundle text={TextProps {
                    content: "Shroom Boom".to_string(),
                    ..default()
                }} styles={title_styles.clone()}/>

                <KButtonBundle
                    styles={button_styles.clone()}
                    button={KButton {
                        text: "New Game".into()
                    }}
                    on_event={click_new_game}
                />

                <KButtonBundle
                    styles={button_styles.clone()}
                    button={KButton {
                        text: "Quit".into()
                    }}
                    on_event={click_quit}
                />
            </BackgroundBundle>
        </BackgroundBundle>
    };

    true
}