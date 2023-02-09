use kayak_ui::prelude::*;
use bevy::prelude::*;

pub fn background_style() -> KStyle {
    KStyle {
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

        ..default()
    }
}

pub fn button_style() -> KStyle {
    KStyle {
        background_color: StyleProp::Value(Color::rgb(0.06, 0.06, 0.06)),
        border_color: StyleProp::Value(Color::rgb_u8(0x2B, 0x20, 0x1D)),
        height: StyleProp::Value(Units::Pixels(50.0)),
        width: StyleProp::Value(Units::Pixels(200.0)),
        padding_top: StyleProp::Value(Units::Stretch(1.0)),
        padding_bottom: StyleProp::Value(Units::Stretch(1.0)),
        border_radius: StyleProp::Value(Corner::all(0.0)),
        font_size: StyleProp::Value(48.0),
        line_height: StyleProp::Value(40.0),

        ..Default::default()
    }
}