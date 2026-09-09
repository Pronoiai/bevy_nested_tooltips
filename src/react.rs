//! The structs that are intended for the user too trigger or react too.
//!

use bevy_ecs::{component::Component, entity::Entity, event::Event};

use crate::{IntoTooltipsContent, TooltipsData};

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
/// Not currently supported for doing this in an existing tooltip.
#[derive(Event)]
pub struct SpawnTooltip {
    /// The term to lookup
    pub term: String,

    /// The entity spawning this, will quick return if an existing tooltip
    /// was spawned using the entity
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

/// Spawn a `Tooltip` with content done at runtime.
/// The motivating case is for sliders, in order to present a value.
///
/// Note this will overwrite any existing `Tooltip`, which is different
/// from the other ways of creating tooltips and maybe surprising
#[derive(Event)]
pub struct SpawnArbitraryTooltip {
    /// The entity spawning this, will quick return if an existing tooltip
    /// was spawned using the entity
    pub entity: Entity,

    /// The display content for the tooltip
    pub tooltips_data: TooltipsData,
}

impl SpawnArbitraryTooltip {
    /// Create new tooltip for the basic cases you can use a string for
    /// the content parameter.
    ///
    /// If you already have a `TooltipsData` instance then consider using `from_tooltips_data`
    pub fn new(
        entity: Entity,
        title: impl Into<String>,
        content: impl IntoTooltipsContent,
    ) -> SpawnArbitraryTooltip {
        let title = title.into();
        let data = TooltipsData {
            title,
            content: content.into_tooltips_content(),
        };
        SpawnArbitraryTooltip {
            entity,
            tooltips_data: data,
        }
    }

    /// Spawn a tooltip using the existing `TooltipsData`
    pub fn from_tooltips_data(
        entity: Entity,
        tooltips_data: TooltipsData,
    ) -> SpawnArbitraryTooltip {
        SpawnArbitraryTooltip {
            entity,
            tooltips_data,
        }
    }
}
