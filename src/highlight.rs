//! Highlighting logic and components are stored here.
//! What highlighting actually does is up to the user.

use bevy_app::{Plugin, PreStartup};
use bevy_ecs::{
    component::Component,
    entity::Entity,
    lifecycle::HookContext,
    observer::On,
    query::{QueryData, With},
    system::{Commands, Query},
    world::World,
};
use bevy_picking::events::{Out, Over, Pointer};
use tiny_bail::prelude::*;

use crate::events::TooltipHighlighting;

pub(crate) struct HighlightPlugin;

impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_systems(PreStartup, setup_component_hooks);
    }
}

/// Inserts [`TooltipHighlighting`] onto entities that has a component [`TooltipHighlight`] with the same string key.
#[derive(Debug, Component, PartialEq)]
pub struct TooltipHighlightLink(pub String);

/// When a [`TooltipHighlightLink`] has been activated and shares the same string with this component
/// [`TooltipHighlighting`] will be added to this entity.
#[derive(Debug, Component)]
pub struct TooltipHighlight(pub Vec<String>);

/// Highlight specific component hooks
fn setup_component_hooks(world: &mut World) {
    world
        .register_component_hooks::<TooltipHighlightLink>()
        .on_insert(|mut world, HookContext { entity, .. }| {
            r!(world.commands().get_entity(entity))
                .observe(highlight_activate)
                .observe(highlight_deactivate);
        })
        .on_despawn(|mut world, HookContext { entity, .. }| {
            #[derive(QueryData)]
            struct HighlightingQuery {
                entity: Entity,
                highlighting: &'static TooltipHighlighting,
            }

            world.commands().run_system_cached_with(
                |entity: bevy_ecs::system::In<Entity>,
                 highlight_query: Query<HighlightingQuery>,
                 mut commands: Commands| {
                    let entity = entity.0;
                    for highlight_item in highlight_query {
                        if highlight_item.highlighting.entity == entity {
                            c!(commands.get_entity(highlight_item.entity))
                                .remove::<TooltipHighlighting>();
                        }
                    }
                },
                entity,
            );
        });
}

#[derive(QueryData)]
struct HighlightNodesQuery {
    entity: Entity,
    tooltip_highlight: &'static TooltipHighlight,
}

/// When text that highlights a node is moused over this will add marker components
/// to the user so they can then apply highlighting logic.
fn highlight_activate(
    hover: On<Pointer<Over>>,
    highlight_nodes_link_query: Query<&TooltipHighlightLink>,
    highlight_nodes_query: Query<HighlightNodesQuery>,
    mut commands: Commands,
) {
    let hover_entity = hover.entity;
    let link = r!(highlight_nodes_link_query.get(hover_entity)).0.clone();

    for node in highlight_nodes_query
        .iter()
        .filter(|x| x.tooltip_highlight.0.contains(&link))
    {
        c!(commands.get_entity(node.entity)).insert(TooltipHighlighting {
            entity: hover_entity,
        });
    }
}

/// When text that highlights a node is no longer moused over this will remove marker components
/// the user can then remove highlighting logic.
fn highlight_deactivate(
    hover: On<Pointer<Out>>,
    highlight_nodes_link_query: Query<&TooltipHighlightLink>,
    highlight_nodes_query: Query<HighlightNodesQuery, With<TooltipHighlighting>>,
    mut commands: Commands,
) {
    let link = r!(highlight_nodes_link_query.get(hover.entity)).0.clone();

    for node in highlight_nodes_query
        .iter()
        .filter(|x| x.tooltip_highlight.0.contains(&link))
    {
        c!(commands.get_entity(node.entity)).remove::<TooltipHighlighting>();
    }
}
