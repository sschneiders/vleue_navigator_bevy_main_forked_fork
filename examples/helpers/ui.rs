use bevy::{color::palettes, diagnostic::DiagnosticsStore, prelude::*};
use vleue_navigator::prelude::*;

#[derive(Component)]
pub enum UiSettings {
    Simplify,
    MergeSteps,
    AgentRadius,
    AgentRadiusOuter,
    Cache,
}

#[derive(Component)]
pub enum UiSettingsButtons {
    SimplifyInc,
    SimplifyDec,
    MergeStepsInc,
    MergeStepsDec,
    AgentRadiusInc,
    AgentRadiusDec,
    AgentRadiusOuterToggle,
    ToggleCache,
}

#[derive(Resource, Default)]
pub struct ExampleSettings {
    pub cache_enabled: bool,
}

fn button(text: &str, action: UiSettingsButtons, font: &Handle<Font>, parent: &mut ChildBuilder) {
    parent
        .spawn((
            Button,
            Node {
                margin: UiRect::all(Val::Px(5.0)),
                border: UiRect::all(Val::Px(1.0)),
                justify_content: JustifyContent::Center,
                height: Val::Px(30.0),
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor(palettes::tailwind::GRAY_500.into()),
            BorderRadius::MAX,
            BackgroundColor(palettes::tailwind::GRAY_700.into()),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font: font.clone(),
                    font_size: 20.0,
                    ..default()
                },
            ));
        });
}

pub fn setup_settings<const WITH_CACHE: bool>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FiraMono-Medium.ttf");
    commands.init_resource::<ExampleSettings>();
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BorderRadius {
                top_left: Val::Px(0.),
                top_right: Val::Px(0.),
                bottom_left: Val::Px(20.0),
                bottom_right: Val::Px(0.),
            },
            BackgroundColor(palettes::tailwind::GRAY_900.with_alpha(0.8).into()),
            Ui,
        ))
        .with_children(|parent| {
            parent.spawn(Node { ..default() }).with_children(|parent| {
                parent
                    .spawn((
                        Text::default(),                        
                        Node {
                            margin: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        UiSettings::Simplify,
                    ))
                    .with_children(|parent| {
                        for (t, s) in [("Simplify: ", 30.0), ("{}", 30.0)].into_iter() {
                            parent.spawn((
                                TextSpan::new(t),
                                TextFont {
                                    font: font.clone(),
                                    font_size: s,
                                    ..default()
                                },
                            ));
                        }
                    });

                button(" - ", UiSettingsButtons::SimplifyDec, &font, parent);
                button(" + ", UiSettingsButtons::SimplifyInc, &font, parent);
            });
            parent.spawn(Node { ..default() }).with_children(|parent| {
                parent
                    .spawn((
                        Text::default(),
                        TextLayout::new_with_justify(JustifyText::Right),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        UiSettings::MergeSteps,
                    ))
                    .with_children(|parent| {
                        for (t, s) in [("Merge Steps: ", 30.0), ("{}", 30.0)].into_iter() {
                            parent.spawn((
                                TextSpan::new(t),
                                TextFont {
                                    font: font.clone(),
                                    font_size: s,
                                    ..default()
                                },
                            ));
                        }
                    });

                button(" - ", UiSettingsButtons::MergeStepsDec, &font, parent);
                button(" + ", UiSettingsButtons::MergeStepsInc, &font, parent);
            });
            parent.spawn(Node { ..default() }).with_children(|parent| {
                parent
                    .spawn((
                        Text::default(),
                        TextLayout::new_with_justify(JustifyText::Right),
                        TextFont {
                            font_size: 30.0,
                            ..default()
                        },
                        Node {
                            margin: UiRect::all(Val::Px(12.0)),
                            ..default()
                        },
                        UiSettings::AgentRadius,
                    ))
                    .with_children(|parent| {
                        for (t, s) in [("Agent Radius: ", 30.0), ("{}", 30.0)].into_iter() {
                            parent.spawn((
                                TextSpan::new(t),
                                TextFont {
                                    font: font.clone(),
                                    font_size: s,
                                    ..default()
                                },
                            ));
                        }
                    });

                button(" - ", UiSettingsButtons::AgentRadiusDec, &font, parent);
                button(" + ", UiSettingsButtons::AgentRadiusInc, &font, parent);
            });
            parent
                .spawn((
                    Button,
                    Node {
                        margin: UiRect::px(30.0, 30.0, 10.0, 30.0),
                        border: UiRect::all(Val::Px(1.0)),
                        justify_content: JustifyContent::Center,
                        height: Val::Px(30.0),
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor(palettes::tailwind::GRAY_500.into()),
                    BorderRadius::all(Val::Px(10.0)),
                    UiImage::default().with_color(palettes::tailwind::GRAY_700.into()),
                    UiSettingsButtons::AgentRadiusOuterToggle,
                    UiSettings::AgentRadiusOuter,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Agent Radius on Outer Edges".to_string()),
                        TextFont {
                            font_size: 20.0,
                            font: font.clone(),
                            ..default()
                        },
                    ));
                });
            if WITH_CACHE {
                parent
                    .spawn((
                        Button,
                        Node {
                            margin: UiRect::px(30.0, 30.0, 10.0, 30.0),
                            border: UiRect::all(Val::Px(1.0)),
                            justify_content: JustifyContent::Center,
                            height: Val::Px(30.0),
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor(palettes::tailwind::GRAY_500.into()),
                        BorderRadius::all(Val::Px(10.0)),
                        UiImage::default().with_color(palettes::tailwind::GRAY_700.into()),
                        UiSettingsButtons::ToggleCache,
                        UiSettings::Cache,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text2d("Cache".to_string()),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                        ));
                    });
            }
        });
}

pub fn display_settings(
    settings: Query<Ref<NavMeshSettings>>,
    example_settings: Res<ExampleSettings>,
    text_query: Query<(Entity, &UiSettings)>,
    mut writer: Text2dWriter,
    mut buttons: Query<(&mut BackgroundColor, &UiSettings), With<Button>>,
) {
    let settings = settings.single();
    if settings.is_changed() {
        for (e, param) in &text_query {
            match param {
                UiSettings::Simplify => *writer.text(e, 2) = format!("{:.2}", settings.simplify),
                UiSettings::MergeSteps => *writer.text(e, 2) = format!("{}", settings.merge_steps),
                UiSettings::AgentRadius => {
                    *writer.text(e, 2) = format!("{:.1}", settings.agent_radius)
                }
                UiSettings::AgentRadiusOuter => (),
                UiSettings::Cache => (),
            }
        }
    }
    if example_settings.is_changed() || settings.is_changed() {
        for (mut color, param) in &mut buttons {
            match param {
                UiSettings::Simplify => (),
                UiSettings::MergeSteps => (),
                UiSettings::AgentRadius => (),
                UiSettings::Cache => {
                    *color = if example_settings.cache_enabled {
                        palettes::tailwind::GREEN_400.into()
                    } else {
                        palettes::tailwind::RED_600.into()
                    }
                }
                UiSettings::AgentRadiusOuter => {
                    *color = if settings.agent_radius_on_outer_edge {
                        palettes::tailwind::GREEN_400.into()
                    } else {
                        palettes::tailwind::RED_600.into()
                    }
                }
            }
        }
    }
}

pub fn update_settings<const STEP: u32>(
    mut interaction_query: Query<
        (&Interaction, &UiSettingsButtons, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut settings: Query<&mut NavMeshSettings>,
    mut example_settings: ResMut<ExampleSettings>,
) {
    for (interaction, button, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                let mut settings = settings.single_mut();
                match *button {
                    UiSettingsButtons::SimplifyDec => {
                        settings.simplify = (settings.simplify - STEP as f32 / 1000.0).max(0.0);
                    }
                    UiSettingsButtons::SimplifyInc => {
                        settings.simplify = (settings.simplify + STEP as f32 / 1000.0).min(10.0);
                    }
                    UiSettingsButtons::MergeStepsDec => {
                        settings.merge_steps = settings.merge_steps.saturating_sub(1);
                    }
                    UiSettingsButtons::MergeStepsInc => {
                        settings.merge_steps = (settings.merge_steps + 1).min(5);
                    }
                    UiSettingsButtons::AgentRadiusDec => {
                        settings.agent_radius = (settings.agent_radius - 0.1).max(0.0);
                    }
                    UiSettingsButtons::AgentRadiusInc => {
                        settings.agent_radius = (settings.agent_radius + 0.1).min(5.0);
                    }
                    UiSettingsButtons::ToggleCache => {
                        example_settings.cache_enabled = !example_settings.cache_enabled;
                    }
                    UiSettingsButtons::AgentRadiusOuterToggle => {
                        settings.agent_radius_on_outer_edge = !settings.agent_radius_on_outer_edge;
                    }
                }
            }
            Interaction::Hovered => {
                if !matches!(button, UiSettingsButtons::ToggleCache)
                    && !matches!(button, UiSettingsButtons::AgentRadiusOuterToggle)
                {
                    *color = palettes::tailwind::GRAY_600.into();
                }
            }
            Interaction::None => {
                if !matches!(button, UiSettingsButtons::ToggleCache)
                    && !matches!(button, UiSettingsButtons::AgentRadiusOuterToggle)
                {
                    *color = palettes::tailwind::GRAY_700.into();
                }
            }
        }
    }
}

pub fn setup_stats<const INTERACTIVE: bool>(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("FiraMono-Medium.ttf");

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                min_width: Val::Px(300.0),
                ..default()
            },
            BorderRadius {
                top_left: Val::Px(0.),
                top_right: Val::Px(0.),
                bottom_right: Val::Px(20.0),
                bottom_left: Val::Px(0.),
            },
            BackgroundColor(palettes::tailwind::GRAY_900.with_alpha(0.8).into()),
            Ui,
        ))
        .with_children(|parent| {
            let mut text = vec![
                ("Status: ", 30.0),
                ("{}", 30.0),
                ("\nObstacles: ", 30.0),
                ("{}", 30.0),
                ("\nPolygons: ", 30.0),
                ("{}", 30.0),
                ("\nBuild Duration: ", 30.0),
                ("{}", 30.0),
                ("ms", 30.0),
            ];
            if INTERACTIVE {
                text.push(("\n\nClick to add an obstacle", 25.0));
                text.push(("\nPress spacebar to reset", 25.0));
            }
            parent
                .spawn((
                    Text::default(),
                    Node {
                        margin: UiRect::all(Val::Px(12.0)),
                        ..default()
                    },
                    UiStats,
                ))
                .with_children(|p| {
                    for (t, s) in text.into_iter() {
                        p.spawn((
                            TextSpan(t.into()),
                            TextFont {
                                font: font.clone(),
                                font_size: s,
                                ..default()
                            },
                        ));
                    }
                });
        });
}

pub fn update_stats<T: Component>(
    text_query: Query<Entity, With<UiStats>>,
    mut writer: Text2dWriter,
    obstacles: Query<&T>,
    navmesh: Query<(Ref<NavMeshStatus>, &NavMeshWrapper)>,
    navmeshes: Res<Assets<NavMesh>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let (status, handle) = navmesh.single();

    if !status.is_changed() && !status.is_added() {
        return;
    }

    let text_entity = text_query.single();
    *writer.text(text_entity, 2) = format!("{:?}", *status);
    *writer.color(text_entity, 2) = match *status {
        NavMeshStatus::Building => palettes::tailwind::AMBER_500.into(),
        NavMeshStatus::Built => palettes::tailwind::GREEN_400.into(),
        NavMeshStatus::Failed => palettes::tailwind::RED_600.into(),
        NavMeshStatus::Cancelled => palettes::tailwind::AMBER_500.into(),
        NavMeshStatus::Invalid => palettes::tailwind::RED_800.into(),
    };
    *writer.text(text_entity, 4) = format!("{}", obstacles.iter().len());
    *writer.text(text_entity, 6) = format!(
        "{}",
        navmeshes
            .get(&handle.0)
            .map(|nm| nm
                .get()
                .layers
                .iter()
                .map(|l| l.polygons.len())
                .sum::<usize>())
            .unwrap_or_default()
    );
    *writer.text(text_entity, 8) = format!(
        "{:.3}",
        diagnostics
            .get(&NAVMESH_BUILD_DURATION)
            .unwrap()
            .smoothed()
            .unwrap_or_default()
            * 1000.0
    );
}

#[derive(Component)]
pub struct UiStats;

#[derive(Component)]
pub struct Ui;

#[allow(dead_code)]
pub fn toggle_ui(
    mut stats: Query<&mut Visibility, With<Ui>>,
    mut entered: EventReader<CursorEntered>,
    mut left: EventReader<CursorLeft>,
) {
    for _ in entered.read() {
        for mut visibility in &mut stats {
            *visibility = Visibility::Visible
        }
    }
    for _ in left.read() {
        for mut visibility in &mut stats {
            *visibility = Visibility::Hidden
        }
    }
}
