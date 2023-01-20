pub mod menu;
pub mod pause;
pub mod hud;
pub mod shop;
pub mod style;
pub mod ui;

use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use bevy::prelude::*;

pub type EventInput = In<(EventDispatcherContext, WidgetState, Event, Entity)>;

#[derive(Copy, Clone, Debug, Component)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_ui);
        ui::register_ui_systems(app);
        menu::register_menu_ui_systems(app);
        pause::register_pause_systems(app);
        hud::register_hud_ui_systems(app);
    }
}

fn startup_ui(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
) {
    font_mapping.set_default(asset_server.load("fonts/roboto.kayak_font"));
    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);

    ui::register_ui(&mut widget_context);
    menu::register_menu_ui(&mut widget_context);
    pause::register_pause_ui(&mut widget_context);

    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            <ui::UiBundle />
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}