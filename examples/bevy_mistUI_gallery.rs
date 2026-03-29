use std::env;

use bevy::prelude::*;
use bevy::render::{
    settings::{InstanceFlags, RenderCreation, WgpuSettings},
    RenderPlugin,
};
use bevy::text::{Justify, TextLayout};
use bevy::ui::{ComputedNode, Display, UiGlobalTransform};
use bevy::window::WindowResolution;
use bevy_mistUI::{
    attach_mist_tooltip, derived_screen_ring, mist_panel, spawn_mist_accordion, spawn_mist_badge,
    spawn_mist_button, spawn_mist_checkbox, spawn_mist_chip, spawn_mist_context_menu,
    spawn_mist_dialog, spawn_mist_dropdown, spawn_mist_grid_view, spawn_mist_input_field,
    spawn_mist_list_view, spawn_mist_menu_list, spawn_mist_popover, spawn_mist_progress_bar,
    spawn_mist_radio_group, spawn_mist_scroll_view, spawn_mist_segmented_action_row,
    spawn_mist_slider, spawn_mist_status_pill, spawn_mist_switch, spawn_mist_table,
    spawn_mist_tabs, spawn_mist_toast, spawn_mist_tree_view, spawn_mist_trigger,
    MistAccordionChanged, MistButtonPressed, MistCheckboxChanged, MistDialog, MistDialogDismissed,
    MistDropdown, MistDropdownChanged, MistGridItemSelected, MistInputField, MistInputSubmitted,
    MistListSelectionChanged, MistMenuAction, MistProgressBar, MistRadioChanged,
    MistSegmentedActionInvoked, MistSliderChanged, MistSliderValue, MistSmokeBackend,
    MistSmokeConfig, MistSmokeEmitter, MistSmokeParticle, MistSmokePreset, MistSwitchChanged,
    MistTableRowSelected, MistTableSortRequested, MistTabsChanged, MistToastDismissed,
    MistTreeNodeSelected, MistTreeNodeSpec, MistTreeNodeToggled, MistTriggerPressed, MistUiPlugins,
    SmokeBorder, SmokeRingPadding,
};

const FONT_BYTES: &[u8] = include_bytes!("../assets/fonts/NotoSansSC-Regular.ttf");
const GALLERY_PAGE_COUNT: usize = 4;

fn main() {
    let view_mode = GalleryViewMode::from_args();
    let default_plugins = DefaultPlugins
        .build()
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: view_mode.window_title().into(),
                resolution: WindowResolution::new(1440, 960),
                resizable: true,
                ..default()
            }),
            ..default()
        })
        .set(RenderPlugin {
            render_creation: RenderCreation::Automatic(resolved_wgpu_settings()),
            ..default()
        });

    App::new()
        .add_plugins(default_plugins)
        .add_plugins(MistUiPlugins)
        .insert_resource(view_mode)
        .insert_resource(GalleryTuning::default())
        .insert_resource(GalleryPageState::default())
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                handle_gallery_paging_input,
                sync_gallery_pages,
                handle_tuning_input,
                handle_backend_toggle,
                apply_tuning,
                sync_progress_from_slider,
                open_gallery_dialog,
                sync_status_text_actions,
                sync_status_text_selection,
                sync_status_text_data_views,
                sync_slider_readout,
                sync_tuning_text,
            ),
        )
        .run();
}

fn resolved_wgpu_settings() -> WgpuSettings {
    WgpuSettings {
        instance_flags: sanitized_instance_flags(),
        ..default()
    }
}

fn sanitized_instance_flags() -> InstanceFlags {
    let mut flags = InstanceFlags::from_build_config().with_env();

    match parse_bool_env("BEVY_MISTUI_ENABLE_WGPU_VALIDATION") {
        Some(true) => {
            flags.insert(InstanceFlags::VALIDATION | InstanceFlags::DEBUG);
        }
        Some(false) => {
            flags.remove(
                InstanceFlags::VALIDATION
                    | InstanceFlags::DEBUG
                    | InstanceFlags::GPU_BASED_VALIDATION,
            );
        }
        None => {
            if env::var_os("WGPU_VALIDATION").is_none() && env::var_os("WGPU_DEBUG").is_none() {
                flags.remove(
                    InstanceFlags::VALIDATION
                        | InstanceFlags::DEBUG
                        | InstanceFlags::GPU_BASED_VALIDATION,
                );
            }
        }
    }

    flags
}

fn parse_bool_env(key: &str) -> Option<bool> {
    let raw = env::var(key).ok()?;
    match raw.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

#[derive(Resource, Clone, Copy, Debug, Default)]
struct GalleryViewMode {
    visual_mock: bool,
}

impl GalleryViewMode {
    fn from_args() -> Self {
        let visual_mock = std::env::args().any(|arg| {
            arg.eq_ignore_ascii_case("--visual-mock") || arg.eq_ignore_ascii_case("--mock")
        });
        Self { visual_mock }
    }

    fn window_title(self) -> &'static str {
        if self.visual_mock {
            "bevy_mistUI / mock audit"
        } else {
            "bevy_mistUI / gallery"
        }
    }
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
            thickness: 0.24,
            intensity: 5.2,
            softness: 0.44,
            flow_speed: 0.96,
            pulse_strength: 0.30,
            padding: 8.0,
        }
    }
}

#[derive(Resource)]
struct GalleryHandles {
    slider: Entity,
    progress: Entity,
    status_text: Entity,
    slider_text: Entity,
    dialog: Entity,
    dialog_button: Entity,
}

#[derive(Resource)]
struct GalleryPageState {
    current_page: usize,
    total_pages: usize,
}

impl Default for GalleryPageState {
    fn default() -> Self {
        Self {
            current_page: 0,
            total_pages: GALLERY_PAGE_COUNT,
        }
    }
}

#[derive(Component)]
struct GalleryWidget;

#[derive(Component)]
struct TuningText;

#[derive(Component)]
struct GalleryPage {
    index: usize,
}

#[derive(Component)]
struct GalleryPageIndicator;

#[derive(Component)]
struct GalleryPrevPageButton;

#[derive(Component)]
struct GalleryNextPageButton;

#[allow(dead_code)]
fn setup_legacy_unused(
    mut commands: Commands,
    mut fonts: ResMut<Assets<Font>>,
    view_mode: Res<GalleryViewMode>,
) {
    let font =
        fonts.add(Font::try_from_bytes(FONT_BYTES.to_vec()).expect("embedded font must be valid"));

    commands.spawn((Camera2d, Name::new("MistGalleryCamera")));

    let root = commands
        .spawn((
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

    let tuning_text = commands
        .spawn((
            TuningText,
            text_block(&font, "", 16.0, Color::srgba(0.90, 0.96, 1.0, 0.96)),
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
                    "1 particles (default)  2 shader ring (fallback)  Q/A thickness  W/S intensity  E/D softness  R/F flow  T/G pulse  Y/H padding  Hold Shift for larger steps",
                    18.0,
                    Color::srgba(0.76, 0.84, 0.94, 0.96),
                ),
            ],
        ));
    });
    commands.entity(root).add_child(tuning_text);

    let trigger_hero =
        sample_frame_with_size(&mut commands, root, "Trigger Hero", &font, 1180.0, 218.0);
    let trigger_hero_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.0),
            ..default()
        },))
        .id();
    let trigger_hero_copy = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                max_width: Val::Px(540.0),
                ..default()
            },
            children![
                text_line(
                    &font,
                    "Primary gaseous trigger",
                    26.0,
                    Color::srgba(0.97, 0.99, 1.0, 0.99),
                ),
                text_block(
                    &font,
                    "边界应该由连续旋走的烟圈读出来，内部只保留低密度雾层托底，不能退回实体线框按钮。",
                    18.0,
                    Color::srgba(0.80, 0.88, 0.96, 0.96),
                ),
            ],
        ))
        .id();
    let hero_trigger = spawn_mist_trigger(&mut commands, &font, "Open Signal", 348.0);
    commands.entity(hero_trigger).insert(GalleryWidget);
    commands
        .entity(trigger_hero_row)
        .add_children(&[trigger_hero_copy, hero_trigger]);
    commands.entity(trigger_hero).add_child(trigger_hero_row);

    let secondary_hero_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            column_gap: Val::Px(18.0),
            align_items: AlignItems::Stretch,
            ..default()
        },))
        .id();
    commands.entity(root).add_child(secondary_hero_row);

    let checkbox_hero = sample_frame_with_size(
        &mut commands,
        secondary_hero_row,
        "Checkbox Hero",
        &font,
        580.0,
        210.0,
    );
    let checkbox_hero_stack = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .id();
    let checkbox_on = spawn_mist_checkbox(&mut commands, &font, "Live bounds synchronized", true);
    let checkbox_off = spawn_mist_checkbox(&mut commands, &font, "Deferred wake field", false);
    commands.entity(checkbox_on).insert(GalleryWidget);
    commands.entity(checkbox_off).insert(GalleryWidget);
    commands
        .entity(checkbox_hero_stack)
        .add_children(&[checkbox_on, checkbox_off]);
    commands
        .entity(checkbox_hero)
        .add_child(checkbox_hero_stack);

    let scroll_hero = sample_frame_with_size(
        &mut commands,
        secondary_hero_row,
        "Scroll Hero",
        &font,
        580.0,
        210.0,
    );
    let scroll_hero_stack = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(14.0),
            ..default()
        },))
        .id();
    let scroll_hint = commands
        .spawn(text_block(
            &font,
            "轨道和滑块都应该是烟雾构件，不是消失的默认滚动条。",
            16.0,
            Color::srgba(0.82, 0.88, 0.96, 0.95),
        ))
        .id();
    let (scroll_hero_view, scroll_hero_content) =
        spawn_mist_scroll_view(&mut commands, 520.0, 122.0);
    commands.entity(scroll_hero_view).insert(GalleryWidget);
    for index in 0..16 {
        let row = commands
            .spawn(text_line(
                &font,
                &format!("Signal {:02}  |  gaseous scrollbar visible", index + 1),
                15.0,
                if index % 2 == 0 {
                    Color::srgba(0.92, 0.97, 1.0, 0.96)
                } else {
                    Color::srgba(0.80, 0.88, 0.96, 0.92)
                },
            ))
            .id();
        commands.entity(scroll_hero_content).add_child(row);
    }
    commands
        .entity(scroll_hero_stack)
        .add_children(&[scroll_hint, scroll_hero_view]);
    commands.entity(scroll_hero).add_child(scroll_hero_stack);

    let grid = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(18.0),
            row_gap: Val::Px(18.0),
            align_content: AlignContent::FlexStart,
            ..default()
        },))
        .id();
    commands.entity(root).add_child(grid);

    if view_mode.visual_mock {
        let visual_mock = spawn_visual_mock_board(&mut commands, grid, &font);
        commands.entity(visual_mock).insert(GalleryWidget);
    }

    let button_card = sample_frame(&mut commands, grid, "Trigger / Button", &font);
    let trigger = spawn_mist_trigger(&mut commands, &font, "Open Signal", 172.0);
    let button = spawn_mist_button(&mut commands, &font, "Confirm Route", 196.0);
    commands.entity(trigger).insert(GalleryWidget);
    commands.entity(button).insert(GalleryWidget);
    commands
        .entity(button_card)
        .add_children(&[trigger, button]);

    let dropdown_card = sample_frame(&mut commands, grid, "Dropdown", &font);
    let dropdown = spawn_mist_dropdown(&mut commands, &font, 220.0, ["English", "中文", "日本語"]);
    commands.entity(dropdown).insert((
        GalleryWidget,
        MistDropdown {
            open: true,
            selected: 0,
        },
    ));
    commands.entity(dropdown_card).add_child(dropdown);

    let input_card = sample_frame(&mut commands, grid, "Input", &font);
    let input = spawn_mist_input_field(
        &mut commands,
        &font,
        280.0,
        MistInputField::new("operator@rope.dev").with_max_chars(42),
    );
    commands.entity(input).insert(GalleryWidget);
    commands.entity(input_card).add_child(input);

    let checkbox_card = sample_frame(&mut commands, grid, "Checkbox", &font);
    let checkbox = spawn_mist_checkbox(&mut commands, &font, "Glow follows live bounds", true);
    commands.entity(checkbox).insert(GalleryWidget);
    commands.entity(checkbox_card).add_child(checkbox);

    let radio_card = sample_frame(&mut commands, grid, "Radio / Switch", &font);
    let radio = spawn_mist_radio_group(
        &mut commands,
        &font,
        280.0,
        ["Balanced", "Dense", "Signal"],
        1,
    );
    let switch = spawn_mist_switch(&mut commands, &font, "Reactive pulse", true);
    commands.entity(radio).insert(GalleryWidget);
    commands.entity(switch).insert(GalleryWidget);
    commands.entity(radio_card).add_children(&[radio, switch]);

    let slider_card = sample_frame(&mut commands, grid, "Slider / Progress", &font);
    let slider_text = commands
        .spawn(text_block(
            &font,
            "Density 72%",
            18.0,
            Color::srgba(0.90, 0.96, 1.0, 0.96),
        ))
        .id();
    let slider = spawn_mist_slider(&mut commands, 260.0, 0.72);
    let progress = spawn_mist_progress_bar(&mut commands, 260.0, 0.72);
    commands.entity(slider).insert(GalleryWidget);
    commands.entity(progress).insert(GalleryWidget);
    commands
        .entity(slider_card)
        .add_children(&[slider_text, slider, progress]);

    let tabs_card = sample_frame(&mut commands, grid, "Tabs / Segmented", &font);
    let tabs = spawn_mist_tabs(
        &mut commands,
        &font,
        320.0,
        ["Overview", "Nodes", "Logs"],
        0,
    );
    commands.entity(tabs).insert(GalleryWidget);
    commands.entity(tabs_card).add_child(tabs);

    let scroll_card = sample_frame(&mut commands, grid, "Scroll View", &font);
    let (scroll_view, scroll_content) = spawn_mist_scroll_view(&mut commands, 320.0, 184.0);
    commands.entity(scroll_view).insert(GalleryWidget);
    commands.entity(scroll_card).add_child(scroll_view);
    for index in 0..14 {
        let row = commands
            .spawn(text_line(
                &font,
                &format!("Node {:02}  |  Mist layer synchronized", index + 1),
                16.0,
                if index % 2 == 0 {
                    Color::srgba(0.90, 0.96, 1.0, 0.96)
                } else {
                    Color::srgba(0.80, 0.88, 0.96, 0.92)
                },
            ))
            .id();
        commands.entity(scroll_content).add_child(row);
    }

    let dialog_card = sample_frame(&mut commands, grid, "Tooltip / Dialog", &font);
    let dialog_button = spawn_mist_button(&mut commands, &font, "Open Modal", 188.0);
    commands.entity(dialog_button).insert(GalleryWidget);
    let tooltip = attach_mist_tooltip(
        &mut commands,
        dialog_button,
        &font,
        "Opens a modal overlay and exercises dialog dismissal through close, backdrop, and Esc.",
        260.0,
    );
    commands.entity(tooltip).insert(GalleryWidget);
    commands.entity(dialog_card).add_child(dialog_button);

    let panel_card = sample_frame(&mut commands, grid, "Panel", &font);
    let panel = commands.spawn((GalleryWidget, mist_panel())).id();
    commands.entity(panel).insert((
        Node {
            width: Val::Px(320.0),
            min_height: Val::Px(122.0),
            ..default()
        },
        children![
            text_line(
                &font,
                "Mist Surface",
                22.0,
                Color::srgba(0.97, 0.99, 1.0, 0.98),
            ),
            text_block(
                &font,
                "This panel uses the same public smoke-ring API as every control in the gallery.",
                16.0,
                Color::srgba(0.82, 0.88, 0.96, 0.96),
            ),
        ],
    ));
    commands.entity(panel_card).add_child(panel);

    let status_card = sample_frame(&mut commands, grid, "Event Feed", &font);
    let status_text = commands
        .spawn(text_block(
            &font,
            "Interact with the widgets. Events are emitted by bevy_mistUI systems.",
            16.0,
            Color::srgba(0.86, 0.93, 1.0, 0.96),
        ))
        .id();
    commands.entity(status_card).add_child(status_text);

    let feedback_card = sample_frame(&mut commands, grid, "Feedback / Chips", &font);
    let badge = spawn_mist_badge(&mut commands, &font, "SMOKE READY");
    let chip = spawn_mist_chip(&mut commands, &font, "Dense Mist", 152.0);
    let pill = spawn_mist_status_pill(&mut commands, &font, "Cluster Stable", true);
    commands.entity(badge).insert(GalleryWidget);
    commands.entity(chip).insert(GalleryWidget);
    commands.entity(pill).insert(GalleryWidget);
    commands
        .entity(feedback_card)
        .add_children(&[badge, chip, pill]);

    let menu_card = sample_frame(&mut commands, grid, "Menu / Context", &font);
    let menu_list = spawn_mist_menu_list(
        &mut commands,
        &font,
        220.0,
        ["Inspect", "Duplicate", "Archive"],
    );
    let context_menu =
        spawn_mist_context_menu(&mut commands, &font, 220.0, ["Promote", "Pause", "Retire"]);
    commands.entity(menu_list).insert(GalleryWidget);
    commands.entity(context_menu).insert(GalleryWidget);
    commands
        .entity(menu_card)
        .add_children(&[menu_list, context_menu]);

    let accordion_card = sample_frame(&mut commands, grid, "Accordion / Actions", &font);
    let segmented = spawn_mist_segmented_action_row(
        &mut commands,
        &font,
        320.0,
        ["Deploy", "Trace", "Quarantine"],
    );
    let accordion = spawn_mist_accordion(
        &mut commands,
        &font,
        320.0,
        vec![
            (
                "Signal routing".to_string(),
                "Primary plume follows active borders and keeps the control body readable."
                    .to_string(),
            ),
            (
                "Density discipline".to_string(),
                "Surface fog stays weaker than frame smoke to avoid losing the silhouette."
                    .to_string(),
            ),
        ],
    );
    commands.entity(segmented).insert(GalleryWidget);
    commands.entity(accordion).insert(GalleryWidget);
    commands
        .entity(accordion_card)
        .add_children(&[segmented, accordion]);

    let overlay_card = sample_frame(&mut commands, grid, "Popover", &font);
    let popover = spawn_mist_popover(
        &mut commands,
        &font,
        "Mist Popover",
        "Standalone overlay body uses the same frame/body smoke grammar as dialog and menu.",
        320.0,
    );
    commands.entity(popover).insert(GalleryWidget);
    commands.entity(overlay_card).add_child(popover);

    let list_card = sample_frame(&mut commands, grid, "List View", &font);
    let list_view = spawn_mist_list_view(
        &mut commands,
        &font,
        320.0,
        184.0,
        [
            "Queue A", "Queue B", "Queue C", "Queue D", "Queue E", "Queue F",
        ],
        Some(1),
    );
    commands.entity(list_view).insert(GalleryWidget);
    commands.entity(list_card).add_child(list_view);

    let table_card = sample_frame_with_size(&mut commands, grid, "Table", &font, 802.0, 206.0);
    let table = spawn_mist_table(
        &mut commands,
        &font,
        760.0,
        vec!["Node".to_string(), "Region".to_string(), "Load".to_string()],
        vec![
            vec![
                "alpha".to_string(),
                "us-east".to_string(),
                "72%".to_string(),
            ],
            vec!["beta".to_string(), "eu-west".to_string(), "58%".to_string()],
            vec![
                "gamma".to_string(),
                "ap-south".to_string(),
                "66%".to_string(),
            ],
        ],
    );
    commands.entity(table).insert(GalleryWidget);
    commands.entity(table_card).add_child(table);

    let data_card = sample_frame_with_size(&mut commands, grid, "Tree / Grid", &font, 802.0, 240.0);
    let data_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            column_gap: Val::Px(18.0),
            align_items: AlignItems::Start,
            ..default()
        },))
        .id();
    let tree = spawn_mist_tree_view(
        &mut commands,
        &font,
        360.0,
        vec![
            MistTreeNodeSpec::root("Edge Cluster"),
            MistTreeNodeSpec::child("Ingress plume", 0),
            MistTreeNodeSpec::child("Archive veil", 0),
            MistTreeNodeSpec::root("Relay Cluster"),
            MistTreeNodeSpec::child("Signal fanout", 3),
        ],
        Some(0),
    );
    let grid_view = spawn_mist_grid_view(
        &mut commands,
        &font,
        360.0,
        3,
        ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta"],
        Some(2),
    );
    commands.entity(tree).insert(GalleryWidget);
    commands.entity(grid_view).insert(GalleryWidget);
    commands.entity(data_row).add_children(&[tree, grid_view]);
    commands.entity(data_card).add_child(data_row);

    let dialog = spawn_mist_dialog(
        &mut commands,
        &font,
        "Mist Dialog",
        "This modal is part of the public bevy_mistUI API. It can be dismissed by the close button, the backdrop, or the Escape key.",
        460.0,
    );
    commands.entity(root).add_child(dialog);

    let toast = spawn_mist_toast(
        &mut commands,
        &font,
        "Mist Toast",
        "Toast notifications are now part of the public smoke component suite.",
        320.0,
    );
    commands.entity(toast).insert(GalleryWidget);
    commands.entity(root).add_child(toast);

    commands.insert_resource(GalleryHandles {
        slider,
        progress,
        status_text,
        slider_text,
        dialog,
        dialog_button,
    });
}

fn setup(mut commands: Commands, mut fonts: ResMut<Assets<Font>>, view_mode: Res<GalleryViewMode>) {
    let font =
        fonts.add(Font::try_from_bytes(FONT_BYTES.to_vec()).expect("embedded font must be valid"));
    let page_state = GalleryPageState::default();

    commands.spawn((Camera2d, Name::new("MistGalleryCamera")));

    let root = commands
        .spawn((
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

    let tuning_text = commands
        .spawn((
            TuningText,
            text_block(&font, "", 16.0, Color::srgba(0.90, 0.96, 1.0, 0.96)),
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
                    "1 particles (default)  2 shader ring (fallback)  Left/Right page  Q/A thickness  W/S intensity  E/D softness  R/F flow  T/G pulse  Y/H padding  Hold Shift for larger steps",
                    18.0,
                    Color::srgba(0.76, 0.84, 0.94, 0.96),
                ),
            ],
        ));
    });
    commands.entity(root).add_child(tuning_text);

    let nav_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(18.0),
            ..default()
        },))
        .id();
    let prev_page = spawn_mist_button(&mut commands, &font, "Prev Page", 176.0);
    let next_page = spawn_mist_button(&mut commands, &font, "Next Page", 176.0);
    let page_indicator = commands
        .spawn((
            GalleryPageIndicator,
            text_line(
                &font,
                &gallery_page_label(page_state.current_page, page_state.total_pages),
                18.0,
                Color::srgba(0.86, 0.93, 1.0, 0.98),
            ),
        ))
        .id();
    commands
        .entity(prev_page)
        .insert((GalleryWidget, GalleryPrevPageButton));
    commands
        .entity(next_page)
        .insert((GalleryWidget, GalleryNextPageButton));
    commands
        .entity(nav_row)
        .add_children(&[prev_page, page_indicator, next_page]);
    commands.entity(root).add_child(nav_row);

    let page_host = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_grow: 1.0,
            min_height: Val::Px(0.0),
            ..default()
        },))
        .id();
    commands.entity(root).add_child(page_host);

    let page_one = spawn_gallery_page(&mut commands, page_host, 0);
    let page_two = spawn_gallery_page(&mut commands, page_host, 1);
    let page_three = spawn_gallery_page(&mut commands, page_host, 2);
    let page_four = spawn_gallery_page(&mut commands, page_host, 3);

    build_core_controls_page(&mut commands, page_one, &font);
    let (slider, progress, slider_text) = build_selection_page(&mut commands, page_two, &font);
    let dialog_button = build_overlay_page(&mut commands, page_three, &font, *view_mode);
    build_data_page(&mut commands, page_four, &font);

    let status_card =
        sample_frame_with_size(&mut commands, root, "Event Feed", &font, 1180.0, 120.0);
    let status_text = commands
        .spawn(text_block(
            &font,
            "Interact with the widgets. Events are emitted by bevy_mistUI systems.",
            16.0,
            Color::srgba(0.86, 0.93, 1.0, 0.96),
        ))
        .id();
    commands.entity(status_card).add_child(status_text);

    let dialog = spawn_mist_dialog(
        &mut commands,
        &font,
        "Mist Dialog",
        "This modal is part of the public bevy_mistUI API. It can be dismissed by the close button, the backdrop, or the Escape key.",
        460.0,
    );
    commands.entity(root).add_child(dialog);

    let toast = spawn_mist_toast(
        &mut commands,
        &font,
        "Mist Toast",
        "Toast notifications are now part of the public smoke component suite.",
        320.0,
    );
    commands.entity(toast).insert(GalleryWidget);
    commands.entity(root).add_child(toast);

    commands.insert_resource(GalleryHandles {
        slider,
        progress,
        status_text,
        slider_text,
        dialog,
        dialog_button,
    });
}

fn sample_frame(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    font: &Handle<Font>,
) -> Entity {
    sample_frame_with_size(commands, parent, title, font, 392.0, 164.0)
}

fn sample_frame_with_size(
    commands: &mut Commands,
    parent: Entity,
    title: &str,
    font: &Handle<Font>,
    width: f32,
    min_height: f32,
) -> Entity {
    let card = commands.spawn(mist_panel()).id();
    commands.entity(card).insert(Node {
        width: Val::Px(width),
        min_height: Val::Px(min_height),
        ..default()
    });
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

fn spawn_gallery_page(commands: &mut Commands, parent: Entity, index: usize) -> Entity {
    let page = commands
        .spawn((
            GalleryPage { index },
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(18.0),
                display: if index == 0 {
                    Display::Flex
                } else {
                    Display::None
                },
                ..default()
            },
        ))
        .id();
    commands.entity(parent).add_child(page);
    page
}

fn spawn_gallery_grid(commands: &mut Commands, parent: Entity) -> Entity {
    let grid = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_wrap: FlexWrap::Wrap,
            column_gap: Val::Px(18.0),
            row_gap: Val::Px(18.0),
            align_content: AlignContent::FlexStart,
            ..default()
        },))
        .id();
    commands.entity(parent).add_child(grid);
    grid
}

fn gallery_page_name(index: usize) -> &'static str {
    match index {
        0 => "Action Signatures",
        1 => "Selection & Scalar",
        2 => "Overlays & Feedback",
        3 => "Data Views",
        _ => "Gallery",
    }
}

fn gallery_page_label(index: usize, total_pages: usize) -> String {
    format!(
        "Page {}/{}  {}  |  Left/Right arrows",
        index.saturating_add(1),
        total_pages.max(1),
        gallery_page_name(index)
    )
}

fn step_gallery_page(page_state: &mut GalleryPageState, delta: isize) -> bool {
    if page_state.total_pages == 0 {
        return false;
    }

    let max_index = page_state.total_pages.saturating_sub(1) as isize;
    let next = (page_state.current_page as isize + delta).clamp(0, max_index) as usize;
    if next == page_state.current_page {
        return false;
    }

    page_state.current_page = next;
    true
}

fn build_core_controls_page(commands: &mut Commands, page: Entity, font: &Handle<Font>) {
    let trigger_hero = sample_frame_with_size(commands, page, "Trigger Hero", font, 1180.0, 218.0);
    let trigger_hero_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.0),
            ..default()
        },))
        .id();
    let trigger_hero_copy = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(10.0),
                max_width: Val::Px(540.0),
                ..default()
            },
            children![
                text_line(font, "Primary gaseous trigger", 26.0, Color::srgba(0.97, 0.99, 1.0, 0.99)),
                text_block(
                    font,
                    "边界应该由连续旋走的烟圈读出来，内部只保留低密度雾层托底，不能退回实体线框按钮。",
                    18.0,
                    Color::srgba(0.80, 0.88, 0.96, 0.96),
                ),
            ],
        ))
        .id();
    let hero_trigger = spawn_mist_trigger(commands, font, "Open Signal", 348.0);
    commands.entity(hero_trigger).insert(GalleryWidget);
    commands
        .entity(trigger_hero_row)
        .add_children(&[trigger_hero_copy, hero_trigger]);
    commands.entity(trigger_hero).add_child(trigger_hero_row);

    let checkbox_hero =
        sample_frame_with_size(commands, page, "Checkbox Hero", font, 1180.0, 178.0);
    let checkbox_hero_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.0),
            ..default()
        },))
        .id();
    let checkbox_hero_copy = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                max_width: Val::Px(520.0),
                ..default()
            },
            children![
                text_line(font, "Latched square indicator", 22.0, Color::srgba(0.97, 0.99, 1.0, 0.98)),
                text_block(
                    font,
                    "checkbox 保持方形锁定腔体 + 常驻细烟粒子，右侧状态牌单独报 ON/OFF，避免和 trigger / button 混淆。",
                    16.0,
                    Color::srgba(0.82, 0.88, 0.96, 0.95),
                ),
            ],
        ))
        .id();
    let checkbox_hero_stack = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(16.0),
            ..default()
        },))
        .id();
    let checkbox_on = spawn_mist_checkbox(commands, font, "Live bounds synchronized", true);
    let checkbox_off = spawn_mist_checkbox(commands, font, "Deferred wake field", false);
    commands.entity(checkbox_on).insert(GalleryWidget);
    commands.entity(checkbox_off).insert(GalleryWidget);
    commands
        .entity(checkbox_hero_stack)
        .add_children(&[checkbox_on, checkbox_off]);
    commands
        .entity(checkbox_hero_row)
        .add_children(&[checkbox_hero_copy, checkbox_hero_stack]);
    commands.entity(checkbox_hero).add_child(checkbox_hero_row);

    let grid = spawn_gallery_grid(commands, page);

    let button_card = sample_frame(commands, grid, "Trigger / Button", font);
    let trigger = spawn_mist_trigger(commands, font, "Open Signal", 172.0);
    let button = spawn_mist_button(commands, font, "Confirm Route", 196.0);
    commands.entity(trigger).insert(GalleryWidget);
    commands.entity(button).insert(GalleryWidget);
    commands
        .entity(button_card)
        .add_children(&[trigger, button]);

    let dropdown_card = sample_frame(commands, grid, "Dropdown", font);
    let dropdown = spawn_mist_dropdown(commands, font, 220.0, ["English", "中文", "日本語"]);
    commands.entity(dropdown).insert((
        GalleryWidget,
        MistDropdown {
            open: true,
            selected: 0,
        },
    ));
    commands.entity(dropdown_card).add_child(dropdown);

    let input_card = sample_frame(commands, grid, "Input", font);
    let input = spawn_mist_input_field(
        commands,
        font,
        280.0,
        MistInputField::new("operator@rope.dev").with_max_chars(42),
    );
    commands.entity(input).insert(GalleryWidget);
    commands.entity(input_card).add_child(input);

    let checkbox_card = sample_frame(commands, grid, "Checkbox", font);
    let checkbox = spawn_mist_checkbox(commands, font, "Glow follows live bounds", true);
    commands.entity(checkbox).insert(GalleryWidget);
    commands.entity(checkbox_card).add_child(checkbox);
}

fn build_selection_page(
    commands: &mut Commands,
    page: Entity,
    font: &Handle<Font>,
) -> (Entity, Entity, Entity) {
    let scroll_hero =
        sample_frame_with_size(commands, page, "Scroll / Slider Hero", font, 1180.0, 224.0);
    let scroll_hero_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            column_gap: Val::Px(24.0),
            ..default()
        },))
        .id();
    let scroll_hero_copy = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(8.0),
                max_width: Val::Px(520.0),
                ..default()
            },
            children![
                text_line(font, "Scalar rail with keyed handle", 22.0, Color::srgba(0.97, 0.99, 1.0, 0.98)),
                text_block(
                    font,
                    "slider 用内轨 + 高把手识别，scroll 保留独立烟雾轨道。选择控件集中到这一页，不再和 trigger / overlay 挤在一起。",
                    16.0,
                    Color::srgba(0.82, 0.88, 0.96, 0.95),
                ),
            ],
        ))
        .id();
    let scroll_hero_stack = commands
        .spawn((Node {
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(14.0),
            ..default()
        },))
        .id();
    let scroll_hint = commands
        .spawn(text_block(
            font,
            "轨道和滑块都应该是烟雾构件，不是消失的默认滚动条。",
            16.0,
            Color::srgba(0.82, 0.88, 0.96, 0.95),
        ))
        .id();
    let (scroll_hero_view, scroll_hero_content) = spawn_mist_scroll_view(commands, 520.0, 122.0);
    commands.entity(scroll_hero_view).insert(GalleryWidget);
    for index in 0..16 {
        let row = commands
            .spawn(text_line(
                font,
                &format!("Signal {:02}  |  gaseous scrollbar visible", index + 1),
                15.0,
                if index % 2 == 0 {
                    Color::srgba(0.92, 0.97, 1.0, 0.96)
                } else {
                    Color::srgba(0.80, 0.88, 0.96, 0.92)
                },
            ))
            .id();
        commands.entity(scroll_hero_content).add_child(row);
    }
    commands
        .entity(scroll_hero_stack)
        .add_children(&[scroll_hint, scroll_hero_view]);
    commands
        .entity(scroll_hero_row)
        .add_children(&[scroll_hero_copy, scroll_hero_stack]);
    commands.entity(scroll_hero).add_child(scroll_hero_row);

    let grid = spawn_gallery_grid(commands, page);

    let radio_card = sample_frame(commands, grid, "Radio / Switch", font);
    let radio = spawn_mist_radio_group(commands, font, 280.0, ["Balanced", "Dense", "Signal"], 1);
    let switch = spawn_mist_switch(commands, font, "Reactive pulse", true);
    commands.entity(radio).insert(GalleryWidget);
    commands.entity(switch).insert(GalleryWidget);
    commands.entity(radio_card).add_children(&[radio, switch]);

    let tabs_card = sample_frame(commands, grid, "Tabs / Segmented", font);
    let tabs = spawn_mist_tabs(commands, font, 320.0, ["Overview", "Nodes", "Logs"], 0);
    commands.entity(tabs).insert(GalleryWidget);
    commands.entity(tabs_card).add_child(tabs);

    let slider_card = sample_frame(commands, grid, "Slider / Progress", font);
    let slider_text = commands
        .spawn(text_block(
            font,
            "Density 72%",
            18.0,
            Color::srgba(0.90, 0.96, 1.0, 0.96),
        ))
        .id();
    let slider = spawn_mist_slider(commands, 260.0, 0.72);
    let progress = spawn_mist_progress_bar(commands, 260.0, 0.72);
    commands.entity(slider).insert(GalleryWidget);
    commands.entity(progress).insert(GalleryWidget);
    commands
        .entity(slider_card)
        .add_children(&[slider_text, slider, progress]);

    let scroll_card = sample_frame(commands, grid, "Scroll View", font);
    let (scroll_view, scroll_content) = spawn_mist_scroll_view(commands, 320.0, 184.0);
    commands.entity(scroll_view).insert(GalleryWidget);
    commands.entity(scroll_card).add_child(scroll_view);
    for index in 0..14 {
        let row = commands
            .spawn(text_line(
                font,
                &format!("Node {:02}  |  Mist layer synchronized", index + 1),
                16.0,
                if index % 2 == 0 {
                    Color::srgba(0.90, 0.96, 1.0, 0.96)
                } else {
                    Color::srgba(0.80, 0.88, 0.96, 0.92)
                },
            ))
            .id();
        commands.entity(scroll_content).add_child(row);
    }

    (slider, progress, slider_text)
}

fn build_overlay_page(
    commands: &mut Commands,
    page: Entity,
    font: &Handle<Font>,
    view_mode: GalleryViewMode,
) -> Entity {
    if view_mode.visual_mock {
        let visual_mock = spawn_visual_mock_board(commands, page, font);
        commands.entity(visual_mock).insert(GalleryWidget);
    }

    let grid = spawn_gallery_grid(commands, page);

    let dialog_card = sample_frame(commands, grid, "Tooltip / Dialog", font);
    let dialog_button = spawn_mist_button(commands, font, "Open Modal", 188.0);
    commands.entity(dialog_button).insert(GalleryWidget);
    let tooltip = attach_mist_tooltip(
        commands,
        dialog_button,
        font,
        "Opens a modal overlay and exercises dialog dismissal through close, backdrop, and Esc.",
        260.0,
    );
    commands.entity(tooltip).insert(GalleryWidget);
    commands.entity(dialog_card).add_child(dialog_button);

    let panel_card = sample_frame(commands, grid, "Panel", font);
    let panel = commands.spawn((GalleryWidget, mist_panel())).id();
    commands.entity(panel).insert((
        Node {
            width: Val::Px(320.0),
            min_height: Val::Px(122.0),
            ..default()
        },
        children![
            text_line(
                font,
                "Mist Surface",
                22.0,
                Color::srgba(0.97, 0.99, 1.0, 0.98)
            ),
            text_block(
                font,
                "This panel uses the same public smoke-ring API as every control in the gallery.",
                16.0,
                Color::srgba(0.82, 0.88, 0.96, 0.96),
            ),
        ],
    ));
    commands.entity(panel_card).add_child(panel);

    let feedback_card = sample_frame(commands, grid, "Feedback / Chips", font);
    let badge = spawn_mist_badge(commands, font, "SMOKE READY");
    let chip = spawn_mist_chip(commands, font, "Dense Mist", 152.0);
    let pill = spawn_mist_status_pill(commands, font, "Cluster Stable", true);
    commands.entity(badge).insert(GalleryWidget);
    commands.entity(chip).insert(GalleryWidget);
    commands.entity(pill).insert(GalleryWidget);
    commands
        .entity(feedback_card)
        .add_children(&[badge, chip, pill]);

    let menu_card = sample_frame(commands, grid, "Menu / Context", font);
    let menu_list =
        spawn_mist_menu_list(commands, font, 220.0, ["Inspect", "Duplicate", "Archive"]);
    let context_menu =
        spawn_mist_context_menu(commands, font, 220.0, ["Promote", "Pause", "Retire"]);
    commands.entity(menu_list).insert(GalleryWidget);
    commands.entity(context_menu).insert(GalleryWidget);
    commands
        .entity(menu_card)
        .add_children(&[menu_list, context_menu]);

    let accordion_card = sample_frame(commands, grid, "Accordion / Actions", font);
    let segmented =
        spawn_mist_segmented_action_row(commands, font, 320.0, ["Deploy", "Trace", "Quarantine"]);
    let accordion = spawn_mist_accordion(
        commands,
        font,
        320.0,
        vec![
            (
                "Signal routing".to_string(),
                "Primary plume follows active borders and keeps the control body readable."
                    .to_string(),
            ),
            (
                "Density discipline".to_string(),
                "Surface fog stays weaker than frame smoke to avoid losing the silhouette."
                    .to_string(),
            ),
        ],
    );
    commands.entity(segmented).insert(GalleryWidget);
    commands.entity(accordion).insert(GalleryWidget);
    commands
        .entity(accordion_card)
        .add_children(&[segmented, accordion]);

    let overlay_card = sample_frame(commands, grid, "Popover", font);
    let popover = spawn_mist_popover(
        commands,
        font,
        "Mist Popover",
        "Standalone overlay body uses the same frame/body smoke grammar as dialog and menu.",
        320.0,
    );
    commands.entity(popover).insert(GalleryWidget);
    commands.entity(overlay_card).add_child(popover);

    dialog_button
}

fn build_data_page(commands: &mut Commands, page: Entity, font: &Handle<Font>) {
    let grid = spawn_gallery_grid(commands, page);

    let list_card = sample_frame(commands, grid, "List View", font);
    let list_view = spawn_mist_list_view(
        commands,
        font,
        320.0,
        184.0,
        [
            "Queue A", "Queue B", "Queue C", "Queue D", "Queue E", "Queue F",
        ],
        Some(1),
    );
    commands.entity(list_view).insert(GalleryWidget);
    commands.entity(list_card).add_child(list_view);

    let table_card = sample_frame_with_size(commands, grid, "Table", font, 802.0, 206.0);
    let table = spawn_mist_table(
        commands,
        font,
        760.0,
        vec!["Node".to_string(), "Region".to_string(), "Load".to_string()],
        vec![
            vec![
                "alpha".to_string(),
                "us-east".to_string(),
                "72%".to_string(),
            ],
            vec!["beta".to_string(), "eu-west".to_string(), "58%".to_string()],
            vec![
                "gamma".to_string(),
                "ap-south".to_string(),
                "66%".to_string(),
            ],
        ],
    );
    commands.entity(table).insert(GalleryWidget);
    commands.entity(table_card).add_child(table);

    let data_card = sample_frame_with_size(commands, grid, "Tree / Grid", font, 802.0, 240.0);
    let data_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            column_gap: Val::Px(18.0),
            align_items: AlignItems::Start,
            ..default()
        },))
        .id();
    let tree = spawn_mist_tree_view(
        commands,
        font,
        360.0,
        vec![
            MistTreeNodeSpec::root("Edge Cluster"),
            MistTreeNodeSpec::child("Ingress plume", 0),
            MistTreeNodeSpec::child("Archive veil", 0),
            MistTreeNodeSpec::root("Relay Cluster"),
            MistTreeNodeSpec::child("Signal fanout", 3),
        ],
        Some(0),
    );
    let grid_view = spawn_mist_grid_view(
        commands,
        font,
        360.0,
        3,
        ["Alpha", "Beta", "Gamma", "Delta", "Epsilon", "Zeta"],
        Some(2),
    );
    commands.entity(tree).insert(GalleryWidget);
    commands.entity(grid_view).insert(GalleryWidget);
    commands.entity(data_row).add_children(&[tree, grid_view]);
    commands.entity(data_card).add_child(data_row);
}

fn spawn_visual_mock_board(commands: &mut Commands, parent: Entity, font: &Handle<Font>) -> Entity {
    let card = sample_frame_with_size(
        commands,
        parent,
        "Visual Mock / Target Audit",
        font,
        802.0,
        236.0,
    );

    let layout = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            column_gap: Val::Px(18.0),
            align_items: AlignItems::Stretch,
            ..default()
        },))
        .id();

    let mock_stage = commands
        .spawn((Node {
            flex_grow: 1.0,
            min_height: Val::Px(176.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(12.0),
            ..default()
        },))
        .id();

    let top_row = commands
        .spawn((Node {
            width: Val::Percent(100.0),
            column_gap: Val::Px(12.0),
            align_items: AlignItems::Start,
            ..default()
        },))
        .id();

    let trigger = spawn_mist_trigger(commands, font, "LANG", 126.0);
    let button = spawn_mist_button(commands, font, "MENU", 126.0);
    let dropdown = spawn_mist_dropdown(commands, font, 220.0, ["English", "中文"]);
    commands.entity(trigger).insert(GalleryWidget);
    commands.entity(button).insert(GalleryWidget);
    commands.entity(dropdown).insert((
        GalleryWidget,
        MistDropdown {
            open: true,
            selected: 0,
        },
    ));
    commands
        .entity(top_row)
        .add_children(&[trigger, button, dropdown]);

    let lower_panel = commands.spawn((GalleryWidget, mist_panel())).id();
    commands.entity(lower_panel).insert(Node {
        width: Val::Percent(100.0),
        min_height: Val::Px(92.0),
        padding: UiRect::all(Val::Px(14.0)),
        row_gap: Val::Px(8.0),
        ..default()
    });
    commands.entity(lower_panel).with_children(|parent| {
        parent.spawn(text_line(
            font,
            "Target Read",
            18.0,
            Color::srgba(0.97, 0.99, 1.0, 0.98),
        ));
        parent.spawn(text_block(
            font,
            "按钮、下拉和容器都应该是空心或低密中心，边界由走马灯烟团读出来，实体边线不应该主导视觉。",
            15.0,
            Color::srgba(0.82, 0.88, 0.96, 0.96),
        ));
    });

    commands
        .entity(mock_stage)
        .add_children(&[top_row, lower_panel]);

    let criteria = commands
        .spawn((Node {
            width: Val::Px(292.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(8.0),
            ..default()
        },))
        .id();
    commands.entity(criteria).with_children(|parent| {
        parent.spawn(text_line(
            font,
            "Pass Criteria",
            18.0,
            Color::srgba(0.97, 0.99, 1.0, 0.98),
        ));
        parent.spawn(text_block(
            font,
            "1. 文字之外没有稳定实线框。\n2. 轮廓是旋转的烟团，不是均匀描边。\n3. 下拉母体和子菜单都保留黑体 + 烟边语言。\n4. Hover / Press 通过烟团变厚、变密来反馈。\n5. 按 Digit2 后若看到僵硬 ring，说明你在看 fallback，不是目标态。",
            15.0,
            Color::srgba(0.82, 0.88, 0.96, 0.96),
        ));
    });

    commands
        .entity(layout)
        .add_children(&[mock_stage, criteria]);
    commands.entity(card).add_child(layout);
    card
}

fn handle_gallery_paging_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut page_state: ResMut<GalleryPageState>,
    prev_buttons: Query<&Interaction, (Changed<Interaction>, With<GalleryPrevPageButton>)>,
    next_buttons: Query<&Interaction, (Changed<Interaction>, With<GalleryNextPageButton>)>,
) {
    if keys.just_pressed(KeyCode::ArrowLeft) {
        step_gallery_page(&mut page_state, -1);
    }
    if keys.just_pressed(KeyCode::ArrowRight) {
        step_gallery_page(&mut page_state, 1);
    }

    for interaction in &prev_buttons {
        if *interaction == Interaction::Pressed {
            step_gallery_page(&mut page_state, -1);
        }
    }
    for interaction in &next_buttons {
        if *interaction == Interaction::Pressed {
            step_gallery_page(&mut page_state, 1);
        }
    }
}

fn sync_gallery_pages(
    page_state: Res<GalleryPageState>,
    mut pages: Query<(&GalleryPage, &mut Node)>,
    mut indicators: Query<&mut Text, With<GalleryPageIndicator>>,
) {
    if !page_state.is_changed() {
        return;
    }

    for (page, mut node) in &mut pages {
        node.display = if page.index == page_state.current_page {
            Display::Flex
        } else {
            Display::None
        };
    }

    let label = gallery_page_label(page_state.current_page, page_state.total_pages);
    for mut text in &mut indicators {
        text.clear();
        text.push_str(&label);
    }
}

fn handle_tuning_input(keys: Res<ButtonInput<KeyCode>>, mut tuning: ResMut<GalleryTuning>) {
    adjust_on_key(
        &keys,
        &mut tuning.thickness,
        KeyCode::KeyQ,
        KeyCode::KeyA,
        0.02,
        0.04,
        0.72,
    );
    adjust_on_key(
        &keys,
        &mut tuning.intensity,
        KeyCode::KeyW,
        KeyCode::KeyS,
        0.20,
        0.8,
        14.0,
    );
    adjust_on_key(
        &keys,
        &mut tuning.softness,
        KeyCode::KeyE,
        KeyCode::KeyD,
        0.03,
        0.08,
        1.30,
    );
    adjust_on_key(
        &keys,
        &mut tuning.flow_speed,
        KeyCode::KeyR,
        KeyCode::KeyF,
        0.05,
        0.10,
        3.60,
    );
    adjust_on_key(
        &keys,
        &mut tuning.pulse_strength,
        KeyCode::KeyT,
        KeyCode::KeyG,
        0.03,
        0.0,
        2.40,
    );
    adjust_on_key(
        &keys,
        &mut tuning.padding,
        KeyCode::KeyY,
        KeyCode::KeyH,
        1.0,
        0.0,
        48.0,
    );
}

fn handle_backend_toggle(keys: Res<ButtonInput<KeyCode>>, mut backend: ResMut<MistSmokeBackend>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *backend = MistSmokeBackend::Particles;
    }
    if keys.just_pressed(KeyCode::Digit2) {
        *backend = MistSmokeBackend::ShaderRing;
    }
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
    let step_scale = if keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight) {
        4.0
    } else {
        1.0
    };

    if keys.just_pressed(up) {
        *value = (*value + delta * step_scale).clamp(min, max);
    }
    if keys.just_pressed(down) {
        *value = (*value - delta * step_scale).clamp(min, max);
    }
}

fn apply_tuning(
    tuning: Res<GalleryTuning>,
    mut smoke_query: Query<(&mut SmokeBorder, Option<&mut MistSmokeConfig>), With<GalleryWidget>>,
    mut padding_query: Query<&mut SmokeRingPadding, With<GalleryWidget>>,
) {
    if !tuning.is_changed() {
        return;
    }

    for (mut smoke, config) in &mut smoke_query {
        let mut tuned = config
            .as_ref()
            .map(|config| **config)
            .unwrap_or_else(|| MistSmokeConfig::screen_preset(MistSmokePreset::StandardButton));
        let density_boost =
            1.45 + (tuning.intensity / 14.0) * 0.75 + (tuning.thickness / 0.72) * 0.30;
        let size_tighten = 0.92 - (tuning.intensity / 14.0) * 0.05;
        tuned.thickness = tuning.thickness;
        tuned.intensity = tuning.intensity;
        tuned.softness = tuning.softness;
        tuned.flow_speed = tuning.flow_speed;
        tuned.pulse_strength = tuning.pulse_strength;
        tuned.particle_density = (tuned.particle_density * density_boost).clamp(1.4, 6.8);
        tuned.particle_size_scale = (tuned.particle_size_scale * size_tighten).clamp(0.54, 1.28);
        if let Some(mut config) = config {
            *config = tuned;
        }
        *smoke = derived_screen_ring(tuned);
    }

    for mut padding in &mut padding_query {
        *padding = SmokeRingPadding::all(tuning.padding);
    }
}

fn sync_tuning_text(
    tuning: Res<GalleryTuning>,
    backend: Res<MistSmokeBackend>,
    particles: Query<&UiGlobalTransform, With<MistSmokeParticle>>,
    emitters: Query<
        (&Name, Option<&UiGlobalTransform>, Option<&ComputedNode>),
        With<MistSmokeEmitter>,
    >,
    mut texts: Query<&mut Text, With<TuningText>>,
) {
    let Ok(mut text) = texts.single_mut() else {
        return;
    };

    let backend_label = match *backend {
        MistSmokeBackend::Particles => "Particles(default)",
        MistSmokeBackend::ShaderRing => "ShaderRing(fallback)",
    };

    let mut particle_count = 0usize;
    let mut avg_particle = Vec2::ZERO;
    let mut min_particle = Vec2::splat(f32::MAX);
    let mut max_particle = Vec2::splat(f32::MIN);
    for transform in &particles {
        let position = transform.to_scale_angle_translation().2;
        particle_count += 1;
        avg_particle += position;
        min_particle = min_particle.min(position);
        max_particle = max_particle.max(position);
    }
    if particle_count > 0 {
        avg_particle /= particle_count as f32;
    } else {
        min_particle = Vec2::ZERO;
        max_particle = Vec2::ZERO;
    }

    let emitter_count = emitters.iter().count();
    let emitter_label = emitters
        .iter()
        .next()
        .map(|(name, transform, computed)| {
            let center = transform
                .map(|transform| transform.to_scale_angle_translation().2)
                .unwrap_or(Vec2::ZERO);
            let size = computed.map(ComputedNode::size).unwrap_or(Vec2::ZERO);
            format!(
                "{}@({:.0},{:.0}) size {:.0}x{:.0}",
                name.as_str(),
                center.x,
                center.y,
                size.x,
                size.y
            )
        })
        .unwrap_or_else(|| "none".to_string());

    text.clear();
    text.push_str(&format!(
        "backend {}  thickness {:.2}  intensity {:.2}  softness {:.2}  flow {:.2}  pulse {:.2}  padding {:.0}px  particles {} emitters {}  avg ({:.0},{:.0})  bbox ({:.0},{:.0})-({:.0},{:.0})  first {}",
        backend_label,
        tuning.thickness,
        tuning.intensity,
        tuning.softness,
        tuning.flow_speed,
        tuning.pulse_strength,
        tuning.padding,
        particle_count,
        emitter_count,
        avg_particle.x,
        avg_particle.y,
        min_particle.x,
        min_particle.y,
        max_particle.x,
        max_particle.y,
        emitter_label,
    ));
}

fn sync_progress_from_slider(
    handles: Res<GalleryHandles>,
    slider_query: Query<&MistSliderValue>,
    mut progress_query: Query<&mut MistProgressBar>,
) {
    let Ok(slider) = slider_query.get(handles.slider) else {
        return;
    };
    let Ok(mut progress) = progress_query.get_mut(handles.progress) else {
        return;
    };
    progress.target = slider.0.clamp(0.0, 1.0);
}

fn sync_slider_readout(
    handles: Res<GalleryHandles>,
    slider_query: Query<&MistSliderValue>,
    mut texts: Query<&mut Text>,
) {
    let Ok(slider) = slider_query.get(handles.slider) else {
        return;
    };
    let Ok(mut text) = texts.get_mut(handles.slider_text) else {
        return;
    };
    text.clear();
    text.push_str(&format!("Density {:.0}%", slider.0.clamp(0.0, 1.0) * 100.0));
}

fn open_gallery_dialog(
    handles: Res<GalleryHandles>,
    mut button_events: MessageReader<MistButtonPressed>,
    mut dialogs: Query<&mut MistDialog>,
) {
    for event in button_events.read() {
        if event.entity != handles.dialog_button {
            continue;
        }
        if let Ok(mut dialog) = dialogs.get_mut(handles.dialog) {
            dialog.open = true;
        }
    }
}

fn sync_status_text_actions(
    handles: Res<GalleryHandles>,
    mut status_texts: Query<&mut Text>,
    mut button_events: MessageReader<MistButtonPressed>,
    mut trigger_events: MessageReader<MistTriggerPressed>,
    mut checkbox_events: MessageReader<MistCheckboxChanged>,
    mut radio_events: MessageReader<MistRadioChanged>,
    mut switch_events: MessageReader<MistSwitchChanged>,
    mut slider_events: MessageReader<MistSliderChanged>,
) {
    let Ok(mut text) = status_texts.get_mut(handles.status_text) else {
        return;
    };

    for event in button_events.read() {
        text.clear();
        text.push_str(&format!("Button pressed: {:?}", event.entity));
    }
    for event in trigger_events.read() {
        text.clear();
        text.push_str(&format!("Trigger fired: {:?}", event.entity));
    }
    for event in checkbox_events.read() {
        text.clear();
        text.push_str(&format!(
            "Checkbox => {}",
            if event.checked { "ON" } else { "OFF" }
        ));
    }
    for event in radio_events.read() {
        text.clear();
        text.push_str(&format!("Radio => {}", event.label));
    }
    for event in switch_events.read() {
        text.clear();
        text.push_str(&format!(
            "Switch => {}",
            if event.on { "ON" } else { "OFF" }
        ));
    }
    for event in slider_events.read() {
        text.clear();
        text.push_str(&format!(
            "Slider => {:.0}%",
            event.value.clamp(0.0, 1.0) * 100.0
        ));
    }
}

fn sync_status_text_selection(
    handles: Res<GalleryHandles>,
    mut status_texts: Query<&mut Text>,
    mut input_submit_events: MessageReader<MistInputSubmitted>,
    mut dropdown_events: MessageReader<MistDropdownChanged>,
    mut tabs_events: MessageReader<MistTabsChanged>,
    mut dialog_events: MessageReader<MistDialogDismissed>,
    mut menu_events: MessageReader<MistMenuAction>,
    mut accordion_events: MessageReader<MistAccordionChanged>,
    mut segmented_events: MessageReader<MistSegmentedActionInvoked>,
) {
    let Ok(mut text) = status_texts.get_mut(handles.status_text) else {
        return;
    };

    for event in input_submit_events.read() {
        text.clear();
        text.push_str(&format!("Input submitted => {}", event.value));
    }
    for event in dropdown_events.read() {
        text.clear();
        text.push_str(&format!("Dropdown => {}", event.label));
    }
    for event in tabs_events.read() {
        text.clear();
        text.push_str(&format!("Tab => {}", event.label));
    }
    for event in dialog_events.read() {
        text.clear();
        text.push_str(&format!("Dialog dismissed: {:?}", event.entity));
    }
    for event in menu_events.read() {
        text.clear();
        text.push_str(&format!("Menu action => {}", event.label));
    }
    for event in accordion_events.read() {
        text.clear();
        text.push_str(&format!(
            "Accordion => {} {}",
            event.label,
            if event.open { "open" } else { "closed" }
        ));
    }
    for event in segmented_events.read() {
        text.clear();
        text.push_str(&format!("Segmented action => {}", event.label));
    }
}

fn sync_status_text_data_views(
    handles: Res<GalleryHandles>,
    mut status_texts: Query<&mut Text>,
    mut list_events: MessageReader<MistListSelectionChanged>,
    mut table_row_events: MessageReader<MistTableRowSelected>,
    mut table_sort_events: MessageReader<MistTableSortRequested>,
    mut tree_select_events: MessageReader<MistTreeNodeSelected>,
    mut tree_toggle_events: MessageReader<MistTreeNodeToggled>,
    mut grid_events: MessageReader<MistGridItemSelected>,
    mut toast_events: MessageReader<MistToastDismissed>,
) {
    let Ok(mut text) = status_texts.get_mut(handles.status_text) else {
        return;
    };

    for event in list_events.read() {
        text.clear();
        text.push_str(&format!("List selection => {}", event.label));
    }
    for event in table_row_events.read() {
        text.clear();
        text.push_str(&format!("Table row => {}", event.row));
    }
    for event in table_sort_events.read() {
        text.clear();
        text.push_str(&format!("Table sort => {}", event.label));
    }
    for event in tree_select_events.read() {
        text.clear();
        text.push_str(&format!("Tree select => {}", event.label));
    }
    for event in tree_toggle_events.read() {
        text.clear();
        text.push_str(&format!(
            "Tree toggle => {} {}",
            event.label,
            if event.expanded {
                "expanded"
            } else {
                "collapsed"
            }
        ));
    }
    for event in grid_events.read() {
        text.clear();
        text.push_str(&format!("Grid selection => {}", event.label));
    }
    for event in toast_events.read() {
        text.clear();
        text.push_str(&format!("Toast dismissed: {:?}", event.entity));
    }
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
