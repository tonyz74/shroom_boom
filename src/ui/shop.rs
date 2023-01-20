use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::ShopAssets;
use crate::player::skill::PlayerSkillLevels;
use crate::shop::{Shop, ShopPurchaseEvent};
use crate::shop::info::ShopItemInfo;
use crate::shop::stock::{SHOP_CATALOG_ITEMS, SHOP_CATALOG_UPGRADES, ShopItem};

use crate::state::GameState;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::style::{background_style, button_style};



#[derive(Debug, Component, PartialEq, Clone)]
pub struct ShopMenuState {
    pub skill_levels: PlayerSkillLevels
}

impl Default for ShopMenuState {
    fn default() -> Self {
        Self {
            skill_levels: PlayerSkillLevels::default()
        }
    }
}

fn update_shop_menu_state(
    p: Query<&PlayerSkillLevels>,
    mut q: Query<&mut ShopMenuState>
) {
    if p.is_empty() || q.is_empty() {
        return;
    }

    let skill_levels = p.single();
    for mut state in q.iter_mut() {
        state.skill_levels = *skill_levels;
    }
}


#[derive(Component, Clone, PartialEq, Default)]
pub struct ShopMenuProps {
}

impl Widget for ShopMenuProps {
}

#[derive(Bundle)]
pub struct ShopMenuBundle {
    pub props: ShopMenuProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for ShopMenuBundle {
    fn default() -> Self {
        Self {
            props: ShopMenuProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: ShopMenuProps::default().get_name(),
        }
    }
}



fn on_purchase(purchase: ShopPurchaseEvent) -> OnEvent {
    OnEvent::new(move |
        In((event_dispatcher_context, _, event, _entity)): EventInput,
        mut evw: EventWriter<ShopPurchaseEvent>
    | {
        match event.event_type {
            EventType::Click(_) => {
                evw.send(purchase);
            }
            _ => {}
        }

        (event_dispatcher_context, event)
    })
}


pub fn register_shop_menu_ui_systems(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
            .with_system(update_shop_menu_state)
    );
}


pub fn register_shop_menu_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<ShopMenuProps, EmptyState>();

    widget_context.add_widget_system(
        ShopMenuProps::default().get_name(),
        widget_update::<ShopMenuProps, EmptyState>,
        shop_menu_render,
    );
}

pub fn shop_menu_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    assets: Res<ShopAssets>,
    state_query: Query<&ShopMenuState>
) -> bool {
    use StyleProp::Value;

    let state_entity = widget_context.use_state(
        &mut commands,
        entity,
        ShopMenuState::default()
    );

    let state = match state_query.get(state_entity) {
        Ok(s) => s,
        _ => return false
    };

    let window_styles = KStyle {
        width: Value(Units::Pixels(1024.0)),
        height: Value(Units::Pixels(640.0)),
        background_color: Value(Color::WHITE),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Column),
        ..default()
    };

    let label_styles = KStyle {
        width: Value(Units::Percentage(10.0)),
        height: Value(Units::Pixels(48.0)),
        color: Value(Color::ORANGE_RED),
        font_size: Value(30.0),
        top: Value(Units::Stretch(1.0)),
        ..default()
    };

    let catalog_styles = KStyle {
        width: Value(Units::Percentage(90.0)),
        height: Value(Units::Percentage(80.0)),
        background_color: Value(Color::RED),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Row),
        ..default()
    };

    let container_styles = KStyle {
        width: Value(Units::Percentage(45.0)),
        height: Value(Units::Percentage(90.0)),
        background_color: Value(Color::GREEN),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Column),
        ..default()
    };

    let button_styles = KStyle {
        width: Value(Units::Percentage(20.0)),
        height: Value(Units::Pixels(48.0)),
        background_color: Value(Color::BLUE),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let items_styles = KStyle {
        layout_type: Value(LayoutType::Column),
        width: Value(Units::Percentage(85.0)),
        height: Value(Units::Percentage(80.0)),
        background_color: Value(Color::PURPLE),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let sale_styles = KStyle {
        layout_type: Value(LayoutType::Row),
        width: Value(Units::Percentage(100.0)),
        height: Value(Units::Pixels(48.0)),
        background_color: Value(Color::AQUAMARINE),
        ..default()
    };

    let item_label_styles = KStyle {
        color: Value(Color::ORANGE_RED),
        font_size: Value(24.0),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        left: Value(Units::Pixels(8.0)),
        ..default()
    };

    let cost_label_styles = KStyle {
        color: Value(Color::GOLD),
        font_size: Value(24.0),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        right: Value(Units::Pixels(8.0)),
        ..default()
    };

    let h_sep_styles = KStyle {
        width: Value(Units::Percentage(100.0)),
        height: Value(Units::Pixels(4.0)),
        background_color: Value(Color::RED),
        ..default()
    };

    let icon_styles = KStyle {
        width: Value(Units::Pixels(48.0)),
        height: Value(Units::Pixels(48.0)),
        ..default()
    };

    let purchase_button_styles = KStyle {
        width: Value(Units::Pixels(48.0)),
        height: Value(Units::Pixels(48.0)),
        border_radius: Value(Corner::all(0.0)),
        ..default()
    };

    let parent_id = Some(entity);

    let click_return_to_gameplay = goto_state_event(StateTransition::Pop);

    let items = SHOP_CATALOG_ITEMS.iter().map(|i| {
        let info = ShopItemInfo::for_item(&assets, *i, None);
        let purchase = ShopPurchaseEvent { cost: info.cost, order: *i };
        (info.icon, info.cost, info.name, on_purchase(purchase))
    });

    let upgrades = SHOP_CATALOG_UPGRADES.iter().filter_map(|i| {
        let lvl = match i {
            ShopItem::HealthUpgrade => state.skill_levels.health_lvl,
            ShopItem::AmmoUpgrade => state.skill_levels.ammo_lvl,
            ShopItem::ShootUpgrade => state.skill_levels.shoot_lvl,
            ShopItem::SlashUpgrade => state.skill_levels.slash_lvl,
            ShopItem::DashUpgrade => state.skill_levels.dash_lvl,
            _ => panic!("Unknown upgrade {:?}!", i)
        };

        let info = ShopItemInfo::for_item(&assets, *i, Some(lvl));
        let purchase = ShopPurchaseEvent { cost: info.cost, order: *i };

        if lvl < 5 {
            Some((
                info.icon,
                info.cost.to_string(),
                format!("{} (Lv. {})", info.name, lvl + 1),
                on_purchase(purchase))
            )
        } else {
            Some((
                info.icon,
                "".to_string(),
                format!("{} (MAX)", info.name),
                OnEvent::default())
            )
        }
    });

    rsx! {
        <BackgroundBundle styles={window_styles}>
            <TextWidgetBundle styles={label_styles.clone()} text={TextProps {
                content: "Toadstool".into(),
                alignment: Alignment::Middle,
                ..Default::default()
            }}/>

            <BackgroundBundle styles={catalog_styles}>
                <BackgroundBundle styles={container_styles.clone()}>

                    <TextWidgetBundle styles={label_styles.clone()} text={TextProps {
                        content: "Upgrades".into(),
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}/>

                    <BackgroundBundle styles={items_styles.clone()}>
                    {upgrades.for_each(|(icon, cost, content, on_event)| {
                        constructor! {
                        <BackgroundBundle styles={sale_styles.clone()}>

                            <KImageBundle
                                styles={icon_styles.clone()}
                                image={KImage(icon.clone())}
                            />

                            <TextWidgetBundle
                                styles={item_label_styles.clone()}
                                text={TextProps {
                                    content,
                                    ..Default::default()
                                }}
                            />

                            <TextWidgetBundle
                                styles={cost_label_styles.clone()}
                                text={TextProps {
                                    content: cost,
                                    ..Default::default()
                                }}
                            />

                            <KButtonBundle
                                styles={purchase_button_styles.clone()}
                                on_event={on_event.clone()}
                            />

                        </BackgroundBundle>
                        };
                        constructor! {
                        <BackgroundBundle styles={h_sep_styles.clone()}/>
                        };
                    })}

                    </BackgroundBundle>

                </BackgroundBundle>

                <BackgroundBundle styles={container_styles.clone()}>

                    <TextWidgetBundle styles={label_styles.clone()} text={TextProps {
                        content: "Items".into(),
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}/>

                    <BackgroundBundle styles={items_styles.clone()}>
                    {items.for_each(|(icon, cost, content, on_event)| {
                        constructor! {
                        <BackgroundBundle styles={sale_styles.clone()}>

                            <KImageBundle
                                styles={icon_styles.clone()}
                                image={KImage(icon.clone())}
                            />

                            <TextWidgetBundle
                                styles={item_label_styles.clone()}
                                text={TextProps {
                                    content: content.to_string(),
                                    ..Default::default()
                                }}
                            />

                            <TextWidgetBundle
                                styles={cost_label_styles.clone()}
                                text={TextProps {
                                    content: cost.to_string(),
                                    ..Default::default()
                                }}
                            />

                            <KButtonBundle
                                styles={purchase_button_styles.clone()}
                                on_event={on_event.clone()}
                            />

                        </BackgroundBundle>
                        };

                        constructor! {
                        <BackgroundBundle styles={h_sep_styles.clone()}/>
                        };
                    })}

                    </BackgroundBundle>

                </BackgroundBundle>
            </BackgroundBundle>

            <KButtonBundle styles={button_styles} on_event={click_return_to_gameplay}/>
        </BackgroundBundle>
    };

    true
}