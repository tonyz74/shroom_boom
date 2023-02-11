use bevy::prelude::*;
use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use crate::assets::UiAssets;
use crate::coin::drops::CoinHolder;
use crate::player::logic::PlayerScore;
use crate::player::Player;
use crate::state::GameState;
use crate::ui::EventInput;
use crate::ui::menu::GotoMenuEvent;
use crate::ui::style::{background_style, button_style};

#[derive(Component, Clone, PartialEq, Default)]
pub struct LoseMenuProps {
}

impl Widget for LoseMenuProps {
}

#[derive(Bundle)]
pub struct LoseMenuBundle {
    pub props: LoseMenuProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for LoseMenuBundle {
    fn default() -> Self {
        Self {
            props: LoseMenuProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: LoseMenuProps::default().get_name(),
        }
    }
}


pub fn register_lose_menu_ui(widget_context: &mut KayakRootContext) {
    widget_context.add_widget_data::<LoseMenuProps, EmptyState>();

    widget_context.add_widget_system(
        LoseMenuProps::default().get_name(),
        widget_update::<LoseMenuProps, EmptyState>,
        lose_menu_render,
    );
}

fn lose_menu_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    assets: Res<UiAssets>,
    score: Res<PlayerScore>
) -> bool {
    let button_styles = button_style();

    let score_styles = KStyle {
        width: StyleProp::Value(Units::Percentage(40.0)),
        height: StyleProp::Value(Units::Pixels(24.0)),
        left: StyleProp::Value(Units::Stretch(0.0)),
        font_size: StyleProp::Value(20.0),
        ..default()
    };

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

    let click_go_back = OnEvent::new(move |
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

    let content = "You Lost!";

    let click_exit = OnEvent::new(move |
        In((event_dispatcher_context, _, event, _entity)): EventInput,
    | {
        match event.event_type {
            EventType::Click(_) => {
                std::process::exit(0);
            }
            _ => {}
        }

        (event_dispatcher_context, event)
    });

    rsx! {
        <BackgroundBundle styles={background_styles.clone()}>

            <KImageBundle
                styles={image_styles.clone()}
                image={KImage(assets.pause_bg.clone())}
            />

            <BackgroundBundle styles={background_styles.clone()}>
                <TextWidgetBundle
                    text={TextProps {
                        content: content.to_string(),
                        ..default()
                    }}
                    styles={title_styles}
                />

                <KButtonBundle
                    styles={button_styles.clone()}
                    button={KButton {
                        text: "Main Menu".into()
                    }}
                    on_event={click_go_back}
                />

                <KButtonBundle
                    styles={button_styles.clone()}
                    button={KButton {
                        text: "Quit".into()
                    }}
                    on_event={click_exit}
                />

                <TextWidgetBundle text={TextProps {
                    content: format!("Score: {:?}", score.score),
                    alignment: Alignment::Middle,
                    ..default()
                }} styles={score_styles}/>
            </BackgroundBundle>

        </BackgroundBundle>
    };

    true
}