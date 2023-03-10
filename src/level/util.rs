use bevy::prelude::*;
use bevy_ecs_ldtk::{prelude::*, ldtk::FieldInstanceEntityReference};

pub fn val_expect_i32(fv: &FieldValue) -> Option<i32> {
    match fv {
        FieldValue::Int(Some(i)) => Some(i.clone()),
        _ => None
    }
}

pub fn val_expect_ent_ref(fv: &FieldValue) -> Option<FieldInstanceEntityReference> {
    match fv {
        FieldValue::EntityRef(Some(e)) => Some(e.clone()),
        _ => None
    }
}

pub fn val_expect_string(fv: &FieldValue) -> Option<String> {
    match fv {
        FieldValue::String(Some(e)) => Some(e.clone()),
        _ => None
    }
}

pub fn val_expect_point(fv: &FieldValue) -> Option<IVec2> {
    match fv {
        FieldValue::Point(Some(e)) => Some(e.clone()),
        _ => None
    }
}