use std::sync::Arc;
use bevy::prelude::*;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use crate::anim::Animation;

#[derive(Component, Clone, Debug, Default)]
pub struct AnimationMap {
    pub anims: Arc<HashMap<String, Animation>>
}

impl AnimationMap {
    pub fn new(map: HashMap<String, Animation>) -> Self {
        Self {
            anims: Arc::new(map)
        }
    }
}

impl Deref for AnimationMap {
    type Target = HashMap<String, Animation>;

    fn deref(&self) -> &Self::Target {
        &self.anims
    }
}