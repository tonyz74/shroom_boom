use std::iter::Map;
use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::prelude::StyleProp::Value;
use kayak_ui::widgets::*;
use crate::assets::ShopAssets;
use crate::shop::{Shop, ShopPurchaseEvent};
use crate::shop::info::ShopItemInfo;
use crate::shop::stock::{SHOP_CATALOG_ITEMS, SHOP_CATALOG_UPGRADES, ShopItem};

use crate::state::GameState;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::style::{background_style, button_style};

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
    app.add_system_set(
        SystemSet::new()
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
    assets: Res<ShopAssets>
) -> bool {
    use StyleProp::Value;

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
        width: Value(Units::Percentage(80.0)),
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

    let info_for_items = |items: &[ShopItem]| {
        items.iter().map(|i| {
            let info = ShopItemInfo::for_item(&assets, *i);

            let purchase = ShopPurchaseEvent {
                cost: info.cost,
                order: *i
            };

            let on_event = OnEvent::new(
                move |
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
                }
            );

            (info.icon, info.cost, info.name, on_event)
        }).collect::<Vec<_>>()
    };

    let items = info_for_items(SHOP_CATALOG_ITEMS);
    let upgrades = info_for_items(SHOP_CATALOG_UPGRADES);

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

                </BackgroundBundle>

                <BackgroundBundle styles={container_styles.clone()}>

                    <TextWidgetBundle styles={label_styles.clone()} text={TextProps {
                        content: "Items".into(),
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}/>

                    <BackgroundBundle styles={items_styles.clone()}>
                    {items.iter().for_each(|(icon, cost, content, on_event)| {
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