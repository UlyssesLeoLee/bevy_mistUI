#![allow(non_snake_case)]

//! bevy_mistUI: community-facing mist / smoke-ring UI crate incubated inside ROPE.
//!
//! Public surface:
//! - [`SmokeBorder`] for procedural ring styling
//! - [`SmokeRingMaterial`] / [`SmokeRingParams`] for the GPU ring material
//! - [`MistSmokePlugin`] for the combined screen-space smoke runtime
//! - [`MistRingSmokePlugin`] / [`MistParticleSmokePlugin`] for atomic smoke backends
//! - [`MistUiPlugins`] as the full plugin group
//! - [`SmokeRingPlugin`] for registration and automatic size syncing
//! - [`SmokeRingPadding`] / [`SmokeRingBundle`] for common attachment cases
//! - [`MistUiPlugin`] as the compatibility widget bundle
//! - atomic widget plugins for messages, smoke hosts, actions, selections, scalar controls,
//!   inputs, and overlays
//! - `spawn_mist_*` helpers for common controls such as trigger, button, checkbox,
//!   radio group, switch, tabs, scroll view, tooltip, dialog, slider, progress bar,
//!   input field, and dropdown

mod border;
mod factory;
mod group;
mod material;
mod particles;
mod plugin;
mod theme;
mod widgets;

pub use border::SmokeBorder;
pub use factory::{MistUiFactory, StandardMistUiFactory};
pub use group::{MistParticleSmokePlugin, MistRingSmokePlugin, MistSmokePlugin, MistUiPlugins};
pub use group::{
    MistActionControlsPlugin, MistDataViewControlsPlugin, MistFeedbackControlsPlugin,
    MistOverlayControlsPlugin, MistScalarControlsPlugin, MistSmokeCorePlugin,
    MistTextEntryControlsPlugin, MistToggleControlsPlugin,
};
pub use material::{
    init_smoke_ring_shader, SmokeRingMaterial, SmokeRingParams, SMOKE_RING_SHADER_HANDLE,
};
pub use particles::{
    derived_screen_ring, MistSmokeBudget, MistSmokeConfig, MistSmokeDomain, MistSmokeOverlayMode,
    MistSmokeEmitter, MistSmokeParticle, MistSmokePlacement, MistSmokePreset,
    MistSmokeRuntimeSet, MistSmokeSurface, MistSmokeTarget, NoMistSmoke, SmokeParticlesPlugin,
};
pub use plugin::{
    MistSmokeBackend, SmokeRingBundle, SmokeRingPadding, SmokeRingPlugin, SmokeRingSettings,
};
pub use theme::MistTheme;
pub use widgets::{
    attach_mist_tooltip, mist_image, mist_label, mist_panel, spawn_mist_button,
    spawn_mist_badge, spawn_mist_checkbox, spawn_mist_chip, spawn_mist_context_menu,
    spawn_mist_dialog, spawn_mist_dropdown, spawn_mist_grid_view, spawn_mist_image,
    spawn_mist_input_field, spawn_mist_label, spawn_mist_list_view, spawn_mist_menu_list,
    spawn_mist_panel, spawn_mist_popover, spawn_mist_progress_bar, spawn_mist_radio_group,
    spawn_mist_scroll_view, spawn_mist_segmented_action_row, spawn_mist_slider,
    spawn_mist_status_pill, spawn_mist_switch, spawn_mist_table, spawn_mist_tabs,
    spawn_mist_toast, spawn_mist_tree_view, spawn_mist_trigger, spawn_mist_accordion,
    MistAccordion, MistAccordionChanged, MistAccordionOwner, MistAccordionParts,
    MistAccordionSection, MistAccordionSections, MistAccordionState, MistBadge, MistButton,
    MistButtonPressed, MistCheckbox, MistCheckboxChanged, MistCheckboxParts, MistCheckboxState,
    MistChip, MistContextMenu, MistContextMenuItem, MistContextMenuOptions, MistContextMenuOwner,
    MistContextMenuParts, MistDialog, MistDialogBackdrop, MistDialogCloseButton,
    MistDialogDismissed, MistDialogOwner, MistDialogParts, MistDropdown, MistDropdownChanged,
    MistDropdownItem, MistDropdownOptions, MistDropdownOwner,
    MistDropdownParts, MistDropdownTrigger, MistGridItem,
    MistGridItemSelected, MistGridItems, MistGridOwner, MistGridView, MistImage,
    MistInputChanged, MistInputField, MistInputFocused, MistInputParts, MistInputSubmitted,
    MistInteractiveStyle, MistLabel, MistListItem, MistListItems, MistListOwner,
    MistListSelectionChanged, MistListView, MistMenuAction, MistMenuList, MistMenuListItem,
    MistMenuListOptions, MistMenuListOwner, MistPanel, MistPopover, MistPopoverAnchor,
    MistPopoverOwner, MistPopoverParts, MistProgressBar, MistProgressParts, MistRadioChanged,
    MistRadioGroup, MistRadioOption, MistRadioOptions, MistRadioOwner, MistRadioParts,
    MistScrollContent, MistScrollParts, MistScrollThumb, MistScrollTrack, MistScrollView,
    MistScrollViewport, MistSegmentedActionButton, MistSegmentedActionInvoked,
    MistSegmentedActionLabels, MistSegmentedActionOwner, MistSegmentedActionParts,
    MistSegmentedActionRow, MistSlider, MistSliderChanged, MistSliderParts, MistSliderValue,
    MistStatusPill, MistSwitch, MistSwitchChanged, MistSwitchParts, MistSwitchState,
    MistTabButton, MistTabLabels, MistTabOwner, MistTabParts, MistTable, MistTableColumns,
    MistTableHeaderButton, MistTableOwner, MistTableRowButton, MistTableRowSelected,
    MistTableRows, MistTableSortRequested, MistTabs, MistTabsChanged, MistToast,
    MistToastCloseButton, MistToastDismissed, MistToastOwner, MistToastParts, MistTooltip,
    MistTooltipAnchor, MistTooltipOwner, MistTreeItem, MistTreeItemParts, MistTreeNodeSelected,
    MistTreeNodeSpec, MistTreeNodeToggled, MistTreeNodes, MistTreeOwner, MistTreeView,
    MistTrigger, MistTriggerPressed, MistUiActionPlugin, MistUiDataViewPlugin,
    MistUiFeedbackPlugin, MistUiInputPlugin, MistUiMessagesPlugin, MistUiOverlayPlugin,
    MistUiPlugin, MistUiScalarPlugin, MistUiSelectionPlugin, MistUiSmokeHostPlugin,
};
