use bevy::prelude::*;
use bevy::text::{Justify, TextLayout};
use bevy::window::WindowResolution;
use bevy_mistUI::{SmokeBorder, SmokeRingPadding, SmokeRingPlugin};

const FONT_BYTES: &[u8] =
    include_bytes!("../assets/fonts/NotoSansSC-Regular.ttf");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_mistUI / bevy_mistUI_gallery".into(),
                resolution: WindowResolution::new(1366, 900),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SmokeRingPlugin)
        .insert_resource(GalleryTuning::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_tuning_input,
                apply_tuning,
                toggle_dropdown,
                select_dropdown_option,
                close_dropdown_on_backdrop,
                sync_dropdown_state,
                update_tuning_text,
            ),
        )
        .run();
}

#[derive(Resource, Clone, Copy)]
struct GalleryTuning {
    thickness: f32,
    intensity: f32,
    softness: f32,
    flow_speed: f32,
    pulse_strength: f32,
    padding: f32,
}

impl Default for GalleryTuning {
    fn default() -> Self {
        Self {
            thickness: 0.16,
            intensity: 3.0,
            softness: 0.34,
            flow_speed: 0.72,
            pulse_strength: 0.18,
            padding: 6.0,
        }
    }
}

#[derive(Component)]
struct GalleryWidget;

#[derive(Component)]
struct TuningText;

#[derive(Component)]
struct DropdownRoot;

#[derive(Component)]
struct DropdownButton;

#[derive(Component)]
struct DropdownButtonLabel;

#[derive(Component)]
struct DropdownMenu;

#[derive(Component)]
struct DropdownBackdrop;

#[derive(Component)]
struct DropdownItem {
    index: usize,
}

#[derive(Component, Clone, Copy)]
struct DropdownOwner(Entity);

#[derive(Component, Debug, Clone)]
struct DropdownOptions(Vec<String>);

#[derive(Component, Debug, Clone)]
struct DropdownState {
    open: bool,
    selected: usize,
}

impl Default for DropdownState {
    fn default() -> Self {
        Self {
            open: false,
            selected: 0,
        }
    }
}

fn setup(mut commands: Commands, mut fonts: ResMut<Assets<Font>>) {
    let font = fonts
        .add(Font::try_from_bytes(FONT_BYTES.to_vec()).expect("embedded font must be valid"));

    commands.spawn((Camera2d, Name::new("SmokeGalleryCamera")));

    let root = commands
        .spawn((
            Name::new("SmokeGalleryRoot"),
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                padding: UiRect::axes(Val::Px(40.0), Val::Px(28.0)),
                row_gap: Val::Px(20.0),
                ..default()
            },
            BackgroundColor(Color::srgb(0.035, 0.05, 0.08)),
        ))
        .id();

    commands.entity(root).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                ..default()
            },
            children![
                text_line(
                    &font,
                    "bevy_mistUI",
                    34.0,
                    Color::srgba(0.97, 0.99, 1.0, 1.0),
                ),
                text_block(
                    &font,
                    "Q/A thickness  W/S intensity  E/D softness  R/F flow  T/G pulse  Y/H padding",
                    18.0,
                    Color::srgba(0.76, 0.84, 0.94, 0.96),
                ),
                (
                    TuningText,
                    text_block(
                        &font,
                        "",
                        16.0,
                        Color::srgba(0.90, 0.96, 1.0, 0.96),
                    ),
                ),
            ],
        ));
    });

    let surface = commands
        .spawn((
            Name::new("SmokeGallerySurface"),
            Node {
                flex_grow: 1.0,
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(24.0),
                ..default()
            },
        ))
        .id();
    commands.entity(root).add_child(surface);

    let backdrop = commands
        .spawn((
            DropdownBackdrop,
            Button,
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(0.0),
                right: Val::Px(0.0),
                top: Val::Px(0.0),
                bottom: Val::Px(0.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::NONE),
            BorderColor::all(Color::NONE),
            Visibility::Hidden,
            GlobalZIndex(10),
            Name::new("DropdownBackdrop"),
        ))
        .id();
    commands.entity(root).add_child(backdrop);

    let grid = commands
        .spawn((
            Name::new("SmokeGalleryGrid"),
            Node {
                width: Val::Percent(100.0),
                flex_wrap: FlexWrap::Wrap,
                column_gap: Val::Px(18.0),
                row_gap: Val::Px(18.0),
                align_content: AlignContent::FlexStart,
                ..default()
            },
        ))
        .id();
    commands.entity(surface).add_child(grid);

    spawn_button_sample(&mut commands, grid, &font);
    spawn_dropdown_sample(&mut commands, grid, backdrop, &font);
    spawn_panel_sample(&mut commands, grid, &font);
    spawn_input_sample(&mut commands, grid, &font);
    spawn_slider_sample(&mut commands, grid, &font);
    spawn_checkbox_sample(&mut commands, grid, &font);
}

fn sample_frame(commands: &mut Commands, parent: Entity, title: &str, font: &Handle<Font>) -> Entity {
    let card = commands
        .spawn((
            Name::new(format!("Sample::{title}")),
            Node {
                width: Val::Px(392.0),
                min_height: Val::Px(164.0),
                padding: UiRect::all(Val::Px(16.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(14.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.08, 0.11, 0.16, 0.78)),
            BorderColor::all(Color::srgba(0.78, 0.88, 1.0, 0.08)),
        ))
        .id();
    commands.entity(parent).add_child(card);
    commands.entity(card).with_children(|parent| {
        parent.spawn(text_line(
            font,
            title,
            20.0,
            Color::srgba(0.96, 0.99, 1.0, 0.98),
        ));
    });
    card
}

fn spawn_button_sample(commands: &mut Commands, parent: Entity, font: &Handle<Font>) {
    let card = sample_frame(commands, parent, "Button", font);
    commands.entity(card).with_children(|parent| {
        parent.spawn((
            GalleryWidget,
            Button,
            SmokeBorder::gaseous_idle(1),
            SmokeRingPadding::all(6.0),
            Node {
                width: Val::Px(220.0),
                min_height: Val::Px(52.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.14, 0.20, 0.84)),
            BorderColor::all(Color::srgba(0.76, 0.90, 1.0, 0.14)),
            children![text_line(
                font,
                "Primary Action",
                22.0,
                Color::srgba(0.97, 0.99, 1.0, 0.98),
            )],
        ));
    });
}

fn spawn_dropdown_sample(
    commands: &mut Commands,
    parent: Entity,
    backdrop: Entity,
    font: &Handle<Font>,
) {
    let card = sample_frame(commands, parent, "Dropdown", font);
    let root = commands
        .spawn((
            DropdownRoot,
            DropdownOptions(vec!["English".into(), "中文".into(), "日本語".into()]),
            DropdownState::default(),
            Name::new("DropdownRoot"),
            Node {
                position_type: PositionType::Relative,
                width: Val::Px(240.0),
                min_height: Val::Px(48.0),
                ..default()
            },
        ))
        .id();
    commands.entity(card).add_child(root);

    let button = commands
        .spawn((
            GalleryWidget,
            DropdownButton,
            DropdownOwner(root),
            Button,
            SmokeBorder::gaseous_idle(2),
            SmokeRingPadding::all(6.0),
            Name::new("DropdownButton"),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(48.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.10, 0.14, 0.20, 0.84)),
            BorderColor::all(Color::srgba(0.76, 0.90, 1.0, 0.14)),
        ))
        .id();
    commands.entity(root).add_child(button);
    commands.entity(button).with_children(|parent| {
        parent.spawn((
            DropdownButtonLabel,
            text_line(
                font,
                "Language: English",
                20.0,
                Color::srgba(0.97, 0.99, 1.0, 0.98),
            ),
        ));
        parent.spawn(text_line(
            font,
            "v",
            20.0,
            Color::srgba(0.80, 0.92, 1.0, 0.96),
        ));
    });

    let menu = commands
        .spawn((
            GalleryWidget,
            DropdownMenu,
            DropdownOwner(root),
            SmokeBorder::gaseous_idle(3),
            SmokeRingPadding::all(6.0),
            Name::new("DropdownMenu"),
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(52.0),
                left: Val::Px(0.0),
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(8.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                display: Display::None,
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.09, 0.14, 0.96)),
            BorderColor::all(Color::srgba(0.80, 0.91, 1.0, 0.12)),
            Visibility::Hidden,
            GlobalZIndex(40),
        ))
        .id();
    commands.entity(root).add_child(menu);

    for (index, label) in ["English", "中文", "日本語"].iter().enumerate() {
        commands.entity(menu).with_children(|parent| {
            parent.spawn((
                DropdownItem { index },
                DropdownOwner(root),
                Button,
                Node {
                    width: Val::Percent(100.0),
                    min_height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                BackgroundColor(Color::srgba(0.08, 0.12, 0.18, 0.88)),
                BorderColor::all(Color::srgba(0.68, 0.82, 1.0, 0.08)),
                children![text_line(
                    font,
                    label,
                    18.0,
                    Color::srgba(0.95, 0.98, 1.0, 0.98),
                )],
            ));
        });
    }

    commands.entity(backdrop).insert(DropdownOwner(root));
}

fn spawn_panel_sample(commands: &mut Commands, parent: Entity, font: &Handle<Font>) {
    let card = sample_frame(commands, parent, "Panel", font);
    commands.entity(card).with_children(|parent| {
        parent.spawn((
            GalleryWidget,
            SmokeBorder::gaseous_idle(4),
            SmokeRingPadding::all(8.0),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(88.0),
                padding: UiRect::all(Val::Px(14.0)),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(6.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.06, 0.10, 0.16, 0.80)),
            BorderColor::all(Color::srgba(0.70, 0.84, 1.0, 0.12)),
            children![
                text_line(
                    font,
                    "Signal Chamber",
                    22.0,
                    Color::srgba(0.97, 0.99, 1.0, 0.98),
                ),
                text_block(
                    font,
                    "The ring follows the real computed node bounds instead of a hand-tuned quad.",
                    16.0,
                    Color::srgba(0.82, 0.88, 0.96, 0.96),
                ),
            ],
        ));
    });
}

fn spawn_input_sample(commands: &mut Commands, parent: Entity, font: &Handle<Font>) {
    let card = sample_frame(commands, parent, "Input", font);
    commands.entity(card).with_children(|parent| {
        parent.spawn((
            GalleryWidget,
            SmokeBorder::gaseous_idle(5),
            SmokeRingPadding::all(6.0),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(52.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.09, 0.12, 0.18, 0.84)),
            BorderColor::all(Color::srgba(0.74, 0.88, 1.0, 0.14)),
            children![
                text_line(
                    font,
                    "operator@rope.dev",
                    20.0,
                    Color::srgba(0.96, 0.99, 1.0, 0.96),
                ),
                (
                    Node {
                        width: Val::Px(2.0),
                        height: Val::Px(24.0),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.96, 0.99, 1.0, 0.88)),
                ),
            ],
        ));
    });
}

fn spawn_slider_sample(commands: &mut Commands, parent: Entity, font: &Handle<Font>) {
    let card = sample_frame(commands, parent, "Slider", font);
    commands.entity(card).with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(12.0),
                width: Val::Percent(100.0),
                ..default()
            },
            children![
                text_block(
                    font,
                    "Density 72%",
                    18.0,
                    Color::srgba(0.90, 0.96, 1.0, 0.96),
                ),
                (
                    GalleryWidget,
                    SmokeBorder::gaseous_idle(6),
                    SmokeRingPadding::all(5.0),
                    Node {
                        width: Val::Percent(100.0),
                        min_height: Val::Px(26.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.11, 0.16, 0.84)),
                    BorderColor::all(Color::srgba(0.72, 0.86, 1.0, 0.10)),
                    children![(
                        Node {
                            width: Val::Percent(82.0),
                            height: Val::Px(6.0),
                            justify_content: JustifyContent::FlexStart,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(Color::srgba(0.18, 0.24, 0.34, 0.90)),
                        children![
                            (
                                Node {
                                    width: Val::Percent(72.0),
                                    height: Val::Percent(100.0),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.86, 0.95, 1.0, 0.92)),
                            ),
                            (
                                Node {
                                    width: Val::Px(16.0),
                                    height: Val::Px(16.0),
                                    margin: UiRect::left(Val::Px(-8.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgba(0.98, 1.0, 1.0, 1.0)),
                            ),
                        ],
                    )],
                ),
            ],
        ));
    });
}

fn spawn_checkbox_sample(commands: &mut Commands, parent: Entity, font: &Handle<Font>) {
    let card = sample_frame(commands, parent, "Checkbox", font);
    commands.entity(card).with_children(|parent| {
        parent.spawn((
            GalleryWidget,
            SmokeBorder::gaseous_idle(7),
            SmokeRingPadding::all(6.0),
            Node {
                width: Val::Percent(100.0),
                min_height: Val::Px(52.0),
                padding: UiRect::axes(Val::Px(14.0), Val::Px(10.0)),
                column_gap: Val::Px(12.0),
                align_items: AlignItems::Center,
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.09, 0.12, 0.18, 0.84)),
            BorderColor::all(Color::srgba(0.74, 0.88, 1.0, 0.14)),
            children![
                (
                    Node {
                        width: Val::Px(22.0),
                        height: Val::Px(22.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.18, 0.24, 0.34, 0.90)),
                    BorderColor::all(Color::srgba(0.82, 0.92, 1.0, 0.18)),
                    children![text_line(
                        font,
                        "✓",
                        18.0,
                        Color::srgba(0.98, 1.0, 1.0, 1.0),
                    )],
                ),
                text_line(
                    font,
                    "Glow follows interaction bounds",
                    18.0,
                    Color::srgba(0.96, 0.99, 1.0, 0.96),
                ),
            ],
        ));
    });
}

fn handle_tuning_input(keys: Res<ButtonInput<KeyCode>>, mut tuning: ResMut<GalleryTuning>) {
    adjust_on_key(&keys, &mut tuning.thickness, KeyCode::KeyQ, KeyCode::KeyA, 0.02, 0.04, 0.32);
    adjust_on_key(&keys, &mut tuning.intensity, KeyCode::KeyW, KeyCode::KeyS, 0.20, 0.8, 6.0);
    adjust_on_key(&keys, &mut tuning.softness, KeyCode::KeyE, KeyCode::KeyD, 0.03, 0.08, 0.70);
    adjust_on_key(&keys, &mut tuning.flow_speed, KeyCode::KeyR, KeyCode::KeyF, 0.05, 0.10, 1.80);
    adjust_on_key(
        &keys,
        &mut tuning.pulse_strength,
        KeyCode::KeyT,
        KeyCode::KeyG,
        0.03,
        0.0,
        0.90,
    );
    adjust_on_key(&keys, &mut tuning.padding, KeyCode::KeyY, KeyCode::KeyH, 1.0, 0.0, 18.0);
}

fn adjust_on_key(
    keys: &ButtonInput<KeyCode>,
    value: &mut f32,
    up: KeyCode,
    down: KeyCode,
    delta: f32,
    min: f32,
    max: f32,
) {
    if keys.just_pressed(up) {
        *value = (*value + delta).clamp(min, max);
    }
    if keys.just_pressed(down) {
        *value = (*value - delta).clamp(min, max);
    }
}

fn apply_tuning(
    tuning: Res<GalleryTuning>,
    mut smoke_query: Query<&mut SmokeBorder, With<GalleryWidget>>,
    mut padding_query: Query<&mut SmokeRingPadding, With<GalleryWidget>>,
) {
    if !tuning.is_changed() {
        return;
    }

    for mut smoke in &mut smoke_query {
        smoke.thickness = tuning.thickness;
        smoke.intensity = tuning.intensity;
        smoke.softness = tuning.softness;
        smoke.flow_speed = tuning.flow_speed;
        smoke.pulse_strength = tuning.pulse_strength;
    }

    for mut padding in &mut padding_query {
        *padding = SmokeRingPadding::all(tuning.padding);
    }
}

fn toggle_dropdown(
    mut buttons: Query<(&Interaction, &DropdownOwner), (Changed<Interaction>, With<DropdownButton>)>,
    mut dropdowns: Query<&mut DropdownState, With<DropdownRoot>>,
) {
    for (interaction, owner) in &mut buttons {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut state) = dropdowns.get_mut(owner.0) {
            state.open = !state.open;
        }
    }
}

fn select_dropdown_option(
    mut items: Query<
        (&Interaction, &DropdownItem, &DropdownOwner),
        (Changed<Interaction>, With<Button>),
    >,
    mut dropdowns: Query<(&DropdownOptions, &mut DropdownState), With<DropdownRoot>>,
) {
    for (interaction, item, owner) in &mut items {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok((options, mut state)) = dropdowns.get_mut(owner.0) {
            state.selected = item.index.min(options.0.len().saturating_sub(1));
            state.open = false;
        }
    }
}

fn close_dropdown_on_backdrop(
    mut backdrops: Query<(&Interaction, &DropdownOwner), (Changed<Interaction>, With<DropdownBackdrop>)>,
    mut dropdowns: Query<&mut DropdownState, With<DropdownRoot>>,
) {
    for (interaction, owner) in &mut backdrops {
        if *interaction != Interaction::Pressed {
            continue;
        }
        if let Ok(mut state) = dropdowns.get_mut(owner.0) {
            state.open = false;
        }
    }
}

fn sync_dropdown_state(
    dropdowns: Query<(Entity, &DropdownOptions, &DropdownState), With<DropdownRoot>>,
    mut labels: Query<(&DropdownOwner, &mut Text), With<DropdownButtonLabel>>,
    mut menus: Query<(&DropdownOwner, &mut Node, &mut Visibility), With<DropdownMenu>>,
    mut backdrops: Query<(&DropdownOwner, &mut Node, &mut Visibility), With<DropdownBackdrop>>,
) {
    for (entity, options, state) in &dropdowns {
        let selected = options
            .0
            .get(state.selected)
            .map(String::as_str)
            .unwrap_or("English");

        for (owner, mut text) in &mut labels {
            if owner.0 != entity {
                continue;
            }
            text.clear();
            text.push_str(&format!("Language: {selected}"));
        }

        for (owner, mut node, mut visibility) in &mut menus {
            if owner.0 != entity {
                continue;
            }
            node.display = if state.open {
                Display::Flex
            } else {
                Display::None
            };
            *visibility = if state.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }

        for (owner, mut node, mut visibility) in &mut backdrops {
            if owner.0 != entity {
                continue;
            }
            node.display = if state.open {
                Display::Flex
            } else {
                Display::None
            };
            *visibility = if state.open {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
        }
    }
}

fn update_tuning_text(
    tuning: Res<GalleryTuning>,
    mut texts: Query<&mut Text, With<TuningText>>,
) {
    if !tuning.is_changed() {
        return;
    }
    let Ok(mut text) = texts.single_mut() else {
        return;
    };
    text.clear();
    text.push_str(&format!(
        "thickness {:.2}  intensity {:.2}  softness {:.2}  flow {:.2}  pulse {:.2}  padding {:.0}px",
        tuning.thickness,
        tuning.intensity,
        tuning.softness,
        tuning.flow_speed,
        tuning.pulse_strength,
        tuning.padding,
    ));
}

fn text_line(font: &Handle<Font>, value: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        Node::default(),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left).with_no_wrap(),
    )
}

fn text_block(font: &Handle<Font>, value: &str, size: f32, color: Color) -> impl Bundle {
    (
        Text::new(value),
        Node::default(),
        TextFont {
            font: font.clone(),
            font_size: size,
            ..default()
        },
        TextColor(color),
        TextLayout::new_with_justify(Justify::Left),
    )
}
