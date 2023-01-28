use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::ShopAssets;
use crate::player::skill::PlayerSkillLevels;
use crate::shop::{ShopPurchaseEvent};
use crate::shop::info::ShopItemInfo;
use crate::shop::stock::{SHOP_CATALOG_ITEMS, SHOP_CATALOG_UPGRADES, ShopItem};


use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::shop_button;



use bevy_inspector_egui::{Inspectable, InspectorPlugin};

#[derive(Resource, Inspectable, Debug)]
pub struct UiShopStyleData {
    pub window_color: Color,
    pub title_color: Color,
    pub label_color: Color,
    pub catalog_color: Color,
    pub container_color: Color,
    pub button_color: Color,
    pub items_color: Color,
    pub sale_color: Color,
    pub item_label_color: Color,
    pub cost_label_color: Color,
    pub h_sep_color: Color,
    pub frame_color: Color,
}


impl Default for UiShopStyleData {
    fn default() -> Self {
        // Self {
        //     frame_color: Color::rgb_u8(0x42, 0x45, 0x57),
        //     h_sep_color: Color::RED,
        //     cost_label_color: Color::GOLD,
        //     item_label_color: Color::ORANGE_RED,
        //     sale_color: Color::AQUAMARINE,
        //     items_color: Color::PURPLE,
        //     button_color: Color::BLUE,
        //     container_color: Color::GREEN,
        //     catalog_color: Color::RED,
        //     label_color: Color::ORANGE_RED,
        //     title_color: Color::BLACK,
        //     window_color: Color::DARK_GRAY
        // }

        use Color::Rgba;

        UiShopStyleData {
            window_color: Rgba {
                red: 0.5019608,
                green: 0.28627452,
                blue: 0.1254902,
                alpha: 1.0
            },
            title_color: Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0
            },
            label_color: Rgba {
                red: 1.0,
                green: 0.92941177,
                blue: 0.72156864,
                alpha: 1.0
            },
            catalog_color: Rgba {
                red: 0.5921569,
                green: 0.45882353,
                blue: 0.3372549,
                alpha: 1.0
            },
            container_color: Rgba {
                red: 0.20392157,
                green: 0.12156863,
                blue: 0.0,
                alpha: 1.0
            },
            button_color: Rgba {
                red: 0.7372549,
                green: 0.5529412,
                blue: 0.26666668,
                alpha: 1.0
            },
            items_color: Rgba {
                red: 0.07058824,
                green: 0.02745098,
                blue: 0.003921569,
                alpha: 1.0
            },
            sale_color: Rgba {
                red: 0.8117647,
                green: 0.7137255,
                blue: 0.5411765,
                alpha: 1.0
            },
            item_label_color: Rgba {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0
            },
            cost_label_color: Rgba {
                red: 0.99607843,
                green: 0.9411765,
                blue: 0.0,
                alpha: 1.0
            },
            h_sep_color: Rgba {
                red: 0.35686275,
                green: 0.23921569,
                blue: 0.07450981,
                alpha: 1.0
            },
            frame_color: Rgba {
                red: 0.13725491,
                green: 0.07058824,
                blue: 0.03137255,
                alpha: 1.0
            }
        }
    }
}






#[derive(Debug, Component, PartialEq, Clone)]
pub struct ShopMenuState {
    pub skill_levels: PlayerSkillLevels,
}

impl Default for ShopMenuState {
    fn default() -> Self {
        Self {
            skill_levels: PlayerSkillLevels::default(),
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


pub fn register_shop_menu_ui_systems(app: &mut App) {
    app
        .init_resource::<UiShopStyleData>()
        .add_system_set(
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
    state_query: Query<&ShopMenuState>,
    data: Res<UiShopStyleData>
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

    let window_color = data.window_color;
    let title_color = data.title_color;
    let label_color = data.label_color;
    let catalog_color = data.catalog_color;
    let container_color = data.container_color;
    let button_color = data.button_color;
    let items_color = data.items_color;
    let sale_color = data.sale_color;
    let item_label_color = data.item_label_color;
    let cost_label_color = data.cost_label_color;
    let h_sep_color = data.h_sep_color;
    let frame_color = data.frame_color;






    let window_styles = KStyle {
        width: Value(Units::Pixels(840.0)),
        height: Value(Units::Pixels(680.0)),
        background_color: Value(window_color),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Column),
        ..default()
    };

    let title_styles = KStyle {
        width: Value(Units::Percentage(10.0)),
        height: Value(Units::Pixels(64.0)),
        color: Value(title_color),
        line_height: Value(256.0),
        font_size: Value(32.0),
        top: Value(Units::Stretch(0.0)),
        ..default()
    };

    let label_styles = KStyle {
        width: Value(Units::Percentage(10.0)),
        height: Value(Units::Pixels(32.0)),
        color: Value(label_color),
        font_size: Value(32.0),
        top: Value(Units::Pixels(-24.0)),
        ..default()
    };

    let catalog_styles = KStyle {
        width: Value(Units::Percentage(94.0)),
        height: Value(Units::Percentage(80.0)),
        background_color: Value(catalog_color),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Row),
        ..default()
    };

    let container_styles = KStyle {
        width: Value(Units::Percentage(47.0)),
        height: Value(Units::Percentage(94.0)),
        background_color: Value(container_color),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        border_radius: Value(Corner::all(32.0)),
        layout_type: Value(LayoutType::Column),
        ..default()
    };

    let button_styles = KStyle {
        width: Value(Units::Percentage(20.0)),
        height: Value(Units::Pixels(48.0)),
        background_color: Value(button_color),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let items_styles = KStyle {
        layout_type: Value(LayoutType::Column),
        width: Value(Units::Percentage(85.0)),
        height: Value(Units::Percentage(85.0)),
        background_color: Value(items_color),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let sale_styles = KStyle {
        layout_type: Value(LayoutType::Row),
        width: Value(Units::Percentage(100.0)),
        height: Value(Units::Pixels(64.0)),
        background_color: Value(sale_color),
        ..default()
    };

    let item_label_styles = KStyle {
        color: Value(item_label_color),
        font_size: Value(18.0),
        line_height: Value(52.0),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        left: Value(Units::Pixels(8.0)),
        ..default()
    };

    let cost_label_styles = KStyle {
        color: Value(cost_label_color),
        font_size: Value(18.0),
        line_height: Value(52.0),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        right: Value(Units::Pixels(8.0)),
        ..default()
    };

    let h_sep_styles = KStyle {
        width: Value(Units::Percentage(100.0)),
        height: Value(Units::Pixels(4.0)),
        background_color: Value(h_sep_color),
        ..default()
    };

    let frame_styles = KStyle {
        width: Value(Units::Pixels(64.0)),
        height: Value(Units::Pixels(64.0)),
        // background_color: Value(Color::rgb_u8(0x42, 0x45, 0x57)),
        background_color: Value(frame_color),
        ..default()
    };

    let icon_styles = KStyle {
        width: Value(Units::Pixels(52.0)),
        height: Value(Units::Pixels(52.0)),
        offset: Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let purchase_container_styles = KStyle {
        width: Value(Units::Pixels(64.0)),
        height: Value(Units::Pixels(64.0)),
        border_radius: Value(Corner::all(0.0)),
        ..default()
    };

    let buy_image_styles = KStyle {
        width: Value(Units::Pixels(64.0)),
        height: Value(Units::Pixels(64.0)),
        ..default()
    };

    let parent_id = Some(entity);

    let click_return_to_gameplay = goto_state_event(StateTransition::Pop);

    let items = SHOP_CATALOG_ITEMS.iter().map(|i| {
        let info = ShopItemInfo::for_item(&assets, *i, None);
        let purchase = ShopPurchaseEvent { cost: info.cost, order: *i };
        (info.icon, format!("${:?}", info.cost), info.name, Some(purchase))
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
                format!("${:?}", info.cost),
                format!("{} lv. {}", info.name, lvl + 1),
                Some(purchase)
            ))
        } else {
            Some((
                info.icon,
                "".to_string(),
                format!("{} MAX", info.name),
                None
            ))
        }
    });


    rsx! {
        <BackgroundBundle styles={window_styles}>
            <TextWidgetBundle styles={title_styles.clone()} text={TextProps {
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
                    {upgrades.for_each(|(icon, cost, content, purchase)| {
                        constructor! {
                        <BackgroundBundle styles={sale_styles.clone()}>

                            <BackgroundBundle styles={frame_styles.clone()}>
                                <KImageBundle
                                    styles={icon_styles.clone()}
                                    image={KImage(icon.clone())}
                                />
                            </BackgroundBundle>

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

                            <BackgroundBundle styles={purchase_container_styles.clone()}>
                                <shop_button::ShopButtonBundle
                                    props={shop_button::ShopButtonProps {
                                        purchase
                                    }}
                                />
                            </BackgroundBundle>

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
                    {items.for_each(|(icon, cost, content, purchase)| {
                        constructor! {
                        <BackgroundBundle styles={sale_styles.clone()}>

                            <BackgroundBundle styles={frame_styles.clone()}>
                                <KImageBundle
                                    styles={icon_styles.clone()}
                                    image={KImage(icon.clone())}
                                />
                            </BackgroundBundle>

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
                                    content: cost,
                                    ..Default::default()
                                }}
                            />

                            <BackgroundBundle styles={purchase_container_styles.clone()}>
                                <shop_button::ShopButtonBundle
                                    props={shop_button::ShopButtonProps {
                                        purchase
                                    }}
                                />
                            </BackgroundBundle>


                        </BackgroundBundle>
                        };

                        constructor! {
                        <BackgroundBundle styles={h_sep_styles.clone()}/>
                        };
                    })}

                    </BackgroundBundle>

                </BackgroundBundle>
            </BackgroundBundle>

            <KButtonBundle
                button={KButton {
                    text: "Ok".into(),
                    ..default()
                }}
                styles={button_styles}
                on_event={click_return_to_gameplay}
            />
        </BackgroundBundle>
    };

    true
}