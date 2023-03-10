use bevy::prelude::*;
use bevy_rapier2d::parry::query::SplitResult::Positive;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::UiAssets;
use crate::state::GameState;
use crate::ui::event_handlers::{goto_state_event, StateTransition};
use crate::ui::EventInput;
use crate::ui::menu::GotoMenuEvent;
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
    assets: Res<UiAssets>,
) -> bool {
    let button_styles = button_style();

    let title_styles = KStyle {
        top: StyleProp::Value(Units::Pixels(0.0)),
        bottom: StyleProp::Value(Units::Percentage(16.0)),
        font_size: StyleProp::Value(40.0),
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

    let click_exit = OnEvent::new(move |
        In((event_dispatcher_context, _, event, _entity)): EventInput,
        mut state: ResMut<State<GameState>>,
        mut goto: ResMut<GotoMenuEvent>
    | {
        match event.event_type {
            EventType::Click(_) => {
                state.pop().unwrap();
                goto.attempt = true;
            }
            _ => {}
        }

        (event_dispatcher_context, event)
    });

    // let click_exit = goto_state_event(StateTransition::Set(GameState::MainMenu));

    rsx! {
        <BackgroundBundle styles={background_styles.clone()}>

            <KImageBundle
                styles={image_styles.clone()}
                image={KImage(assets.pause_bg.clone())}
            />

            <BackgroundBundle styles={background_styles.clone()}>
                <TextWidgetBundle
                    text={TextProps {
                        content: "Paused".to_string(),
                        ..default()
                    }}
                    styles={title_styles}
                />

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
                        text: "Quit".into()
                    }}
                    on_event={click_exit}
                />
            </BackgroundBundle>

        </BackgroundBundle>
    };

    true
}