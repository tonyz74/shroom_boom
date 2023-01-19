use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::state::GameState;
use crate::ui::EventInput;
use crate::ui::style::{background_style, button_style};

pub fn register_pause_systems(app: &mut App) {
    app
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(pause_if_needed)
        );
}

fn pause_if_needed(
    mut state: ResMut<State<GameState>>,
    key: Res<Input<KeyCode>>,
) {
    if !key.just_pressed(KeyCode::Escape) || state.current() != &GameState::Gameplay {
        return;
    }

    state.push(GameState::PauseMenu).unwrap();
}


#[derive(Component, Clone, PartialEq, Default)]
pub struct PauseMenuProps {
}

impl Widget for PauseMenuProps {
}

#[derive(Bundle)]
pub struct PauseMenuBundle {
    pub props: PauseMenuProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for PauseMenuBundle {
    fn default() -> Self {
        Self {
            props: PauseMenuProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: PauseMenuProps::default().get_name(),
        }
    }
}


pub fn register_pause_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<PauseMenuProps, EmptyState>();

    widget_context.add_widget_system(
        PauseMenuProps::default().get_name(),
        widget_update::<PauseMenuProps, EmptyState>,
        pause_menu_render,
    );
}

fn pause_menu_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
) -> bool {
    let button_styles = button_style();
    let mut background_styles = background_style();
    background_styles.background_color = StyleProp::Value(Color::RED);

    let parent_id = Some(entity);

    let click_resume = OnEvent::new(move |
        In((event_dispatcher_context, _, event, _entity)): EventInput,
        mut state: ResMut<State<GameState>>,
    | {
        match event.event_type {
            EventType::Click(_) => {
                state.pop().unwrap();
            }
            _ => {}
        }

        (event_dispatcher_context, event)
    });

    rsx! {
        <BackgroundBundle styles={background_styles}>
            <KButtonBundle
                styles={button_styles.clone()}
                button={KButton {
                    text: "Resume".into()
                }}
                on_event={click_resume}
            />

            <KButtonBundle
                styles={button_styles.clone()}
                button={KButton {
                    text: "Save & Quit".into()
                }}
            />
        </BackgroundBundle>
    };

    true
}