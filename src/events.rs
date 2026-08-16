//! The events that are intended to be read by the user to react to are stored here.
//!

use bevy_ecs::{component::Component, entity::Entity};

/// Marker to indicate this node is currently being highlighted by this tooltip
/// When this component is added user should apply styling so it's obvious to the player
/// what is being highlighted.
///
/// See the highlight module for details on highlighting.
///
/// This also includes which Entity triggered the highlight, this is used for handling cases of the highlight link being despawned
/// End users may also use this for their own purpose
#[derive(Debug, Component)]
pub struct TooltipHighlighting {
    pub entity: Entity,
}

/// Marker to indicate that this `ToolTip` should not be despawned.
/// When this component is added user should apply styling so it's obvious to the player
/// that the tooltip will not be despawned by timeout or pointer leaving.
#[derive(Debug, Component)]
pub struct TooltipLocked;
