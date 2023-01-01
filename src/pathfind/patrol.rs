use bevy::prelude::*;
use bevy_rapier2d::rapier::crossbeam::channel::internal::select;
use rand::prelude::*;
use crate::pathfind::grid::PathfindingGrid;
use crate::pathfind::Region;

#[derive(Copy, Clone, Debug)]
pub enum WalkPathfinderPatrolPoint {
    Left,
    Right
}

impl Default for WalkPathfinderPatrolPoint {
    fn default() -> Self {
        Self::Left
    }
}

impl WalkPathfinderPatrolPoint {
    pub fn advance(&mut self) {
        match self {
            Self::Left => {
                *self = Self::Right;
            },
            Self::Right => {
                *self = Self::Left
            },
        }
    }
}