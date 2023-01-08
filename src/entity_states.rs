use bevy::prelude::*;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Jump;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Fall;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Shoot;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Move;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Hurt;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Idle;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Active;

#[derive(Default, Copy, Clone, Component, Reflect)]
pub struct Die {
    pub should_despawn: bool
}


