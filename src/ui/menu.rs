use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;

use crate::state::GameState;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
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
        SystemSet::new()
    );
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
) -> bool {
    let background_styles = background_style();
    let button_styles = button_style();

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
        <BackgroundBundle styles={background_styles}>
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
                    text: "Load Game".into()
                }}
            />

            <KButtonBundle
                styles={button_styles.clone()}
                button={KButton {
                    text: "Quit".into()
                }}
                on_event={click_quit}
            />
        </BackgroundBundle>
    };

    true
}