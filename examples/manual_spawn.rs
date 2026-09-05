//! A quick example of manually triggering a tooltip spawn

use bevy::prelude::*;

use bevy_color::palettes::css::{BLACK, ORANGE, WHITE, YELLOW_GREEN};
use bevy_nested_tooltips::prelude::*;

use bevy_platform::collections::HashMap;
use bevy_window::WindowMode;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            //This library only works for fullscreen
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            NestedTooltipPlugin,
        ))
        .add_systems(Startup, spawn_scene)
        .add_observer(style_tooltip)
        .run()
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn_scene(bsn! {Camera2d});

    let background_colour = BackgroundColor(Oklcha::lch(0.7, 0.1, 229.).into());

    commands.spawn_scene(bsn! {
        template_value(background_colour)
        Node{
            position_type:PositionType::Absolute,
            display:Display::Grid,
            left:percent(30),
            top:percent(30),
            width:vw(45),
            height:vh(45),
            grid_template_rows:vec![GridTrack::fr(1.), GridTrack::fr(5.)],
        }
        Children[
            Node{
                display:Display::Flex,
                justify_content:JustifyContent::Center,
                width:percent(100),
            }
            Children[
                Text::new("Bevy nested tooltips")
                TextFont{
                    font_size:FontSize::Px(50.)
                }
            ],


            Node{
                width:percent(100),
            }
            BackgroundColor(YELLOW_GREEN)
            Children[
                Node{
                    width:percent(100),
                    border:UiRect::all(px(2)),
                }
                BorderColor::all(BLACK)
                on(clicked_me)
                Children[
                    Text::new("Click me!")
                ]
            ]
        ]
    });

    let mut tooltip_map = TooltipMap {
        map: HashMap::new(),
    };

    tooltip_map.insert(
        "click_info".into(),
        TooltipsData::new(
            "You clicked me",
            vec![
                TooltipsContent::String("This popup is your prize ".into()),
                TooltipsContent::Term(TooltipsContentDetail::new("recursive")),
            ],
        ),
    );

    tooltip_map.insert(
        "recursive".into(),
        TooltipsData::new(
            "Recursive",
            vec![TooltipsContent::String("I still nest".to_string())],
        ),
    );

    commands.insert_resource(tooltip_map);
}

/// This is the example of triggering manually
fn clicked_me(click: On<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(SpawnTooltip::new("click_info", click.entity));
}

// This is how you style a tooltip!
// If you want to change the default node consider using TooltipReference
fn style_tooltip(tooltip: On<Add, Tooltip>, mut commands: Commands) {
    commands
        .get_entity(tooltip.entity)
        .unwrap()
        .insert((BackgroundColor(ORANGE.into()), BorderColor::all(WHITE)));
}
