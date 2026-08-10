//! Terms is how tooltips find out what to display given a word to link.

use bevy_ecs::{
    component::Component,
    entity::Entity,
    observer::On,
    query::AnyOf,
    system::{Commands, Query, Res},
    template::{EntityTemplate, FromTemplate, Template},
};
use bevy_picking::events::{Over, Pointer};
use bevy_time::{Timer, TimerMode};
use tiny_bail::prelude::*;

use crate::{ActivationMethod, TooltipConfiguration, TooltipLinkTimer, TooltipPointerPresence};

/// Place this on a node or text that you want to spawn a Tooltip.
/// The tooltip displayed will be the contents of [`crate::TooltipMap`].
#[derive(Debug, Component, PartialEq)]
#[require(TooltipPointerPresence)]
pub struct TooltipTermLink {
    pub(crate) linked_string: String,
}

impl TooltipTermLink {
    /// Create a new link component
    pub fn new(linked_string: impl ToString) -> Self {
        Self {
            linked_string: linked_string.to_string(),
        }
    }
    /// The string that is used to look up the term
    pub fn linked_string(&self) -> &str {
        &self.linked_string
    }
}

/// This is used for putting links of tooltips in tooltips
/// Should not be created by end users but can safely read if you are interested in recursive case
/// Recursive case may be treated seperately in future such as shorter hover times.
#[derive(Clone, Debug, Component, PartialEq)]
pub struct TooltipTermLinkRecursive {
    pub(crate) parent_entity: Entity,
    pub(crate) linked_string: String,
}

impl TooltipTermLinkRecursive {
    /// The string that is used to look up the term.
    pub fn linked_string(&self) -> &str {
        &self.linked_string
    }

    /// The [`crate::Tooltip`] that holds this link.
    pub fn parent_entity(&self) -> Entity {
        self.parent_entity
    }
}

impl FromTemplate for TooltipTermLinkRecursive {
    type Template = TooltipTermLinkRecursiveTemplate;
}

#[derive(Default)]
pub struct TooltipTermLinkRecursiveTemplate {
    pub parent_entity: EntityTemplate,
    pub linked_string: String,
}

impl Template for TooltipTermLinkRecursiveTemplate {
    type Output = TooltipTermLinkRecursive;

    fn build_template(
        &self,
        context: &mut bevy_ecs::template::TemplateContext,
    ) -> bevy_ecs::error::Result<Self::Output> {
        Ok(TooltipTermLinkRecursive {
            parent_entity: self.parent_entity.build_template(context)?,
            linked_string: self.linked_string.clone(),
        })
    }

    fn clone_template(&self) -> Self {
        Self {
            parent_entity: self.parent_entity.clone_template(),
            linked_string: self.linked_string.clone_template(),
        }
    }
}

/// This triggers for [`crate::Tooltip`] links
/// If configured to display on hover this will add a [`crate::TooltipLinkTimer`] that unless pointer moves
/// away from will spawn a [`crate::Tooltip`].
pub(crate) fn hover_time_spawn(
    hover: On<Pointer<Over>>,
    tooltip_query: Query<AnyOf<(&TooltipTermLink, &TooltipTermLinkRecursive)>>,
    tooltip_configuration: Res<TooltipConfiguration>,
    mut commands: Commands,
) {
    // Would prefer to restrict hover to entites, but I don't know how to while having marker components
    if !tooltip_query.contains(hover.entity) {
        return;
    }

    let current_activation = tooltip_configuration.activation_method.clone();

    if let ActivationMethod::Hover { time } = current_activation {
        {
            r!(commands.get_entity(hover.entity)).insert(TooltipLinkTimer {
                timer: Timer::new(time, TimerMode::Once),
            });
        }
    }
}
