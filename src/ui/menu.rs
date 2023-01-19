use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;

use crate::state::GameState;
use crate::ui::EventInput;



#[derive(Debug, Component, PartialEq, Clone)]
pub struct MainMenuState {
    pub shown: bool
}

impl Default for MainMenuState {
    fn default() -> Self {
        Self {
            shown: true
        }
    }
}

#[derive(Component, Clone, PartialEq, Default)]
pub struct MainMenuProps {
}

impl Widget for MainMenuProps {
}

// Now we need a widget bundle this can represent a collection of components our widget might have
// Note: You can include custom data here. Just don't expect it to get diffed during update!
#[derive(Bundle)]
pub struct MainMenuBundle {
    pub props: MainMenuProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    // This allows us to hook into on click events!
    pub on_event: OnEvent,
    // Widget name is required by Kayak UI!
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
            // Kayak uses this component to find out more information about your widget.
            // This is done because bevy does not have the ability to query traits.
            widget_name: MainMenuProps::default().get_name(),
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
    state_query: Query<&MainMenuState>
) -> bool {
    let state_entity = widget_context.use_state(
        // Bevy commands
        &mut commands,
        // The widget entity.
        entity,
        // The default starting values for the state.
        MainMenuState::default()
    );

    let state = match state_query.get(state_entity) {
        Ok(s) => s,
        _ => return false
    };

    let background_styles = KStyle {
        border_radius: StyleProp::Value(Corner::all(15.0)),
        background_color: StyleProp::Value(Color::rgb(0.03, 0.03, 0.03)),

        left: StyleProp::Value(Units::Stretch(1.0)),
        right: StyleProp::Value(Units::Stretch(1.0)),
        top: StyleProp::Value(Units::Stretch(1.0)),
        bottom: StyleProp::Value(Units::Stretch(1.0)),

        padding: StyleProp::Value(Edge::all(Units::Stretch(1.0))),

        width: StyleProp::Value(Units::Pixels(360.0)),
        height: StyleProp::Value(Units::Pixels(500.0)),

        layout_type: StyleProp::Value(LayoutType::Column),
        row_between: StyleProp::Value(Units::Pixels(20.0)),

        ..Default::default()
    };

    let button_styles = KStyle {
        background_color: StyleProp::Value(Color::BLACK),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),

        ..Default::default()
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

    let click_new_game = OnEvent::new(move |
            In((event_dispatcher_context, _, event, _entity)): EventInput,
            mut state: ResMut<State<GameState>>,
            mut menu_states: Query<&mut MainMenuState>
        | {
            match event.event_type {
                EventType::Click(_) => {
                    state.set(GameState::LevelTransition).unwrap();

                    if let Ok(mut state) = menu_states.get_mut(state_entity) {
                        state.shown = false;
                    }
                }
                _ => {}
            }

            (event_dispatcher_context, event)
        }
    );


    let parent_id = Some(entity);

    rsx! {
        <ElementBundle>
        {if state.shown {
            constructor! {
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
            }
        }}
        </ElementBundle>
    };

    true
}