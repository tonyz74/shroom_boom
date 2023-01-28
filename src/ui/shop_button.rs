use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::ShopAssets;
use crate::shop::ShopPurchaseEvent;

use crate::state::GameState;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::style::{background_style, button_style};

#[derive(Component, Clone, PartialEq, Default)]
pub struct ShopButtonProps {
    pub purchase: Option<ShopPurchaseEvent>
}

impl Widget for ShopButtonProps {
}

#[derive(Debug, Component, PartialEq, Clone)]
pub enum ShopButtonState {
    Normal,
    Hover,
    Press
}

impl Default for ShopButtonState {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Bundle)]
pub struct ShopButtonBundle {
    pub props: ShopButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for ShopButtonBundle {
    fn default() -> Self {
        Self {
            props: ShopButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: ShopButtonProps::default().get_name(),
        }
    }
}


pub fn register_shop_button_systems(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
    );
}


pub fn register_shop_button_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<ShopButtonProps, EmptyState>();

    widget_context.add_widget_system(
        ShopButtonProps::default().get_name(),
        widget_update::<ShopButtonProps, EmptyState>,
        shop_button_render,
    );
}

pub fn shop_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    props: Query<&ShopButtonProps>,
    mut commands: Commands,
    state_query: Query<&ShopButtonState>,
    assets: Res<ShopAssets>
) -> bool {

    let state_entity = widget_context.use_state(
        &mut commands,
        entity,
        ShopButtonState::default()
    );

    let state = match state_query.get(state_entity) {
        Ok(s) => s,
        _ => return false
    };

    let props = props.get(entity).unwrap();
    let purchase = props.purchase.clone();

    let style = KStyle {
        width: StyleProp::Value(Units::Pixels(64.0)),
        height: StyleProp::Value(Units::Pixels(64.0)),
        offset: StyleProp::Value(Edge::all(Units::Stretch(1.0))),
        ..default()
    };

    let on_event = OnEvent::new(move |
        In((event_dispatcher_context, widget_state, event, entity)): EventInput,
        mut states: Query<&mut ShopButtonState>,
        mut evw: EventWriter<ShopPurchaseEvent>,
    | {
        let mut state = states.get_mut(state_entity).unwrap();

        match event.event_type {
            EventType::Click(_) => {
                if let Some(p) = purchase {
                    evw.send(p);
                }
            }

            EventType::Hover(_) => { if *state != ShopButtonState::Press { *state = ShopButtonState::Hover } },
            EventType::MouseDown(_) => { *state = ShopButtonState::Press },
            EventType::MouseUp(_) => { *state = ShopButtonState::Hover },
            EventType::MouseOut(_) => { *state = ShopButtonState::Normal },

            _ => {}
        };

        (event_dispatcher_context, event)
    });

    let image_handle = if props.purchase.is_none() {
        assets.blank.clone()
    } else {
        match state {
            ShopButtonState::Normal => assets.buy.clone(),
            ShopButtonState::Hover => assets.buy_hover.clone(),
            ShopButtonState::Press => assets.buy_pressed.clone()
        }
    };

    let parent_id = Some(entity);

    rsx! {
        <BackgroundBundle styles={style.clone()} on_event={on_event}>
            <KImageBundle
                styles={style.clone()}
                image={KImage(image_handle)}
            />
        </BackgroundBundle>
    };


    true
}