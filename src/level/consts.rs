use bevy_rapier2d::prelude::*;

pub const TILE_SIZE: f32 = 8.0;
pub const RENDERED_TILE_SIZE: f32 = 32.0;
pub const SCALE_FACTOR: f32 = RENDERED_TILE_SIZE / TILE_SIZE;


pub const SOLID_PLATFORM_GROUP_MASK: u32        = 0b00000001;
pub const ONE_WAY_PLATFORM_GROUP_MASK: u32      = 0b00000010;
pub const ALL_PLATFORMS_GROUP_MASK: u32         = 0b00000011;

pub const ALL_PLATFORMS_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(
    Group::from_bits_truncate(ALL_PLATFORMS_GROUP_MASK),
    Group::from_bits_truncate(ALL_PLATFORMS_GROUP_MASK),
);

pub const ONE_WAY_PLATFORMS_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(
    Group::from_bits_truncate(ONE_WAY_PLATFORM_GROUP_MASK),
    Group::from_bits_truncate(ONE_WAY_PLATFORM_GROUP_MASK),
);

pub const SOLIDS_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(
    Group::from_bits_truncate(SOLID_PLATFORM_GROUP_MASK),
    Group::from_bits_truncate(SOLID_PLATFORM_GROUP_MASK),
);

pub const ALL_PLATFORMS_INTERACTION_GROUP: InteractionGroups = InteractionGroups::new(
    bevy_rapier2d::rapier::geometry::Group::from_bits_truncate(ALL_PLATFORMS_GROUP_MASK),
    bevy_rapier2d::rapier::geometry::Group::from_bits_truncate(ALL_PLATFORMS_GROUP_MASK),
);

pub const SOLIDS_INTERACTION_GROUP: InteractionGroups = InteractionGroups::new(
    bevy_rapier2d::rapier::geometry::Group::from_bits_truncate(SOLID_PLATFORM_GROUP_MASK),
    bevy_rapier2d::rapier::geometry::Group::from_bits_truncate(SOLID_PLATFORM_GROUP_MASK),
);