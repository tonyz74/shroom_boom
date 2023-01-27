use crate::state::GameState;
use bevy::prelude::*;

use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::ui::{menu, pause, shop};


#[derive(Debug, Component, PartialEq, Clone)]
pub struct UiState {
    pub state: Option<GameState>
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            state: None
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct UiProps {
}

impl Widget for UiProps {
}

#[derive(Bundle)]
pub struct UiBundle {
    pub props: UiProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for UiBundle {
    fn default() -> Self {
        Self {
            props: UiProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: UiProps::default().get_name(),
        }
    }
}


pub fn register_ui_systems(app: &mut App) {
    app.add_system_set(
        SystemSet::new()
            .with_system(update_state)
    );
}

fn update_state(
    state: Res<State<GameState>>,
    mut ui_state: Query<&mut UiState>
) {
    for mut ui_state in ui_state.iter_mut() {
        ui_state.state = Some(state.current().clone());
    }
}

pub fn register_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<UiProps, EmptyState>();

    widget_context.add_widget_system(
        UiProps::default().get_name(),
        widget_update::<UiProps, EmptyState>,
        ui_render,
    );
}

fn ui_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    state_query: Query<&UiState>
) -> bool {
    let state_entity = widget_context.use_state(
        &mut commands,
        entity,
        UiState::default()
    );

    let state = match state_query.get(state_entity) {
        Ok(s) => s,
        _ => return false
    };

    let parent_id = Some(entity);

    rsx! {
        <ElementBundle>
            {if state.state == Some(GameState::MainMenu) {
                constructor! {
                    <menu::MainMenuBundle/>
                }
            }}

            {if state.state == Some(GameState::PauseMenu) {
                constructor! {
                    <pause::PauseMenuBundle/>
                }
            }}

            {if state.state == Some(GameState::ShopMenu) {
                constructor! {
                    <shop::ShopMenuBundle/>
                }
            }}
        </ElementBundle>
    };

    true
}