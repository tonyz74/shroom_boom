pub mod menu;
pub mod pause;
pub mod hud;
pub mod shop;

use kayak_ui::prelude::*;
use kayak_ui::widgets::*;
use bevy::prelude::*;

pub type EventInput = In<(EventDispatcherContext, WidgetState, Event, Entity)>;

#[derive(Copy, Clone, Debug, Component)]
pub struct GameUiPlugin;

impl Plugin for GameUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_ui);
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
    menu::register_menu_ui(&mut widget_context);

    let parent_id = None;

    rsx! {
        <KayakAppBundle>
            <menu::MainMenuBundle/>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
