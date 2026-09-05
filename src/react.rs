//! The structs that are intended for the user too trigger or react too.
//!

use bevy_ecs::{component::Component, entity::Entity, event::Event};

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

/// Marker to indicate that this `Tooltip` should not be despawned.
/// When this component is added user should apply styling so it's obvious to the player
/// that the tooltip will not be despawned by timeout or pointer leaving.
#[derive(Debug, Component)]
pub struct TooltipLocked;

/// Manually spawn a `Tooltip`, useful for icons users may click.
/// Not currently supported for nested tooltips.
#[derive(Event)]
pub struct SpawnTooltip {
    /// The term to lookup
    pub term: String,

    /// The entity spawning this, will quick return if existing tooltips
    pub entity: Entity,
}

impl SpawnTooltip {
    pub fn new(term: impl Into<String>, entity: Entity) -> Self {
        Self {
            term: term.into(),
            entity,
        }
    }
}
