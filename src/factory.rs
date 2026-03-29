use crate::{
    attach_mist_tooltip, spawn_mist_accordion, spawn_mist_badge, spawn_mist_button,
    spawn_mist_checkbox, spawn_mist_chip, spawn_mist_context_menu, spawn_mist_dialog,
    spawn_mist_dropdown, spawn_mist_grid_view, spawn_mist_image, spawn_mist_input_field,
    spawn_mist_label, spawn_mist_list_view, spawn_mist_menu_list, spawn_mist_panel,
    spawn_mist_popover, spawn_mist_progress_bar, spawn_mist_radio_group,
    spawn_mist_scroll_view, spawn_mist_segmented_action_row, spawn_mist_slider,
    spawn_mist_status_pill, spawn_mist_switch, spawn_mist_table, spawn_mist_tabs,
    spawn_mist_toast, spawn_mist_tree_view, spawn_mist_trigger, MistInputField,
    MistTreeNodeSpec,
};
use bevy::prelude::*;

pub trait MistUiFactory {
    fn spawn_panel(&self, commands: &mut Commands) -> Entity {
        spawn_mist_panel(commands)
    }

    fn spawn_badge<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
    ) -> Entity {
        spawn_mist_badge(commands, font, label)
    }

    fn spawn_chip<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        width: f32,
    ) -> Entity {
        spawn_mist_chip(commands, font, label, width)
    }

    fn spawn_status_pill<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        active: bool,
    ) -> Entity {
        spawn_mist_status_pill(commands, font, label, active)
    }

    fn spawn_label<S: AsRef<str>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        value: S,
        size: f32,
    ) -> Entity {
        spawn_mist_label(commands, font, value, size)
    }

    fn spawn_image(
        &self,
        commands: &mut Commands,
        image: Handle<Image>,
        size: Vec2,
    ) -> Entity {
        spawn_mist_image(commands, image, size)
    }

    fn spawn_button<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        width: f32,
    ) -> Entity {
        spawn_mist_button(commands, font, label, width)
    }

    fn spawn_trigger<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        width: f32,
    ) -> Entity {
        spawn_mist_trigger(commands, font, label, width)
    }

    fn spawn_checkbox<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        checked: bool,
    ) -> Entity {
        spawn_mist_checkbox(commands, font, label, checked)
    }

    fn spawn_radio_group<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        options: I,
        selected: usize,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_radio_group(commands, font, width, options, selected)
    }

    fn spawn_switch<S: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        label: S,
        on: bool,
    ) -> Entity {
        spawn_mist_switch(commands, font, label, on)
    }

    fn spawn_scroll_view(
        &self,
        commands: &mut Commands,
        width: f32,
        height: f32,
    ) -> (Entity, Entity) {
        spawn_mist_scroll_view(commands, width, height)
    }

    fn spawn_slider(&self, commands: &mut Commands, width: f32, value: f32) -> Entity {
        spawn_mist_slider(commands, width, value)
    }

    fn spawn_progress_bar(&self, commands: &mut Commands, width: f32, target: f32) -> Entity {
        spawn_mist_progress_bar(commands, width, target)
    }

    fn spawn_dropdown<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        options: I,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_dropdown(commands, font, width, options)
    }

    fn spawn_input_field(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        field: MistInputField,
    ) -> Entity {
        spawn_mist_input_field(commands, font, width, field)
    }

    fn attach_tooltip<S: Into<String>>(
        &self,
        commands: &mut Commands,
        anchor: Entity,
        font: &Handle<Font>,
        label: S,
        max_width: f32,
    ) -> Entity {
        attach_mist_tooltip(commands, anchor, font, label, max_width)
    }

    fn spawn_tabs<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        labels: I,
        selected: usize,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_tabs(commands, font, width, labels, selected)
    }

    fn spawn_dialog<S1: Into<String>, S2: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        title: S1,
        body: S2,
        width: f32,
    ) -> Entity {
        spawn_mist_dialog(commands, font, title, body, width)
    }

    fn spawn_toast<S1: Into<String>, S2: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        title: S1,
        body: S2,
        width: f32,
    ) -> Entity {
        spawn_mist_toast(commands, font, title, body, width)
    }

    fn spawn_popover<S1: Into<String>, S2: Into<String>>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        title: S1,
        body: S2,
        width: f32,
    ) -> Entity {
        spawn_mist_popover(commands, font, title, body, width)
    }

    fn spawn_context_menu<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        options: I,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_context_menu(commands, font, width, options)
    }

    fn spawn_menu_list<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        options: I,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_menu_list(commands, font, width, options)
    }

    fn spawn_accordion(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        sections: Vec<(String, String)>,
    ) -> Entity {
        spawn_mist_accordion(commands, font, width, sections)
    }

    fn spawn_segmented_action_row<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        labels: I,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_segmented_action_row(commands, font, width, labels)
    }

    fn spawn_list_view<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        height: f32,
        items: I,
        selected: Option<usize>,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_list_view(commands, font, width, height, items, selected)
    }

    fn spawn_table(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        columns: Vec<String>,
        rows: Vec<Vec<String>>,
    ) -> Entity {
        spawn_mist_table(commands, font, width, columns, rows)
    }

    fn spawn_tree_view(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        nodes: Vec<MistTreeNodeSpec>,
        selected: Option<usize>,
    ) -> Entity {
        spawn_mist_tree_view(commands, font, width, nodes, selected)
    }

    fn spawn_grid_view<I, S>(
        &self,
        commands: &mut Commands,
        font: &Handle<Font>,
        width: f32,
        columns: usize,
        items: I,
        selected: Option<usize>,
    ) -> Entity
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        spawn_mist_grid_view(commands, font, width, columns, items, selected)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct StandardMistUiFactory;

impl MistUiFactory for StandardMistUiFactory {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MistAccordion, MistBadge, MistButton, MistCheckbox, MistChip, MistContextMenu, MistDialog,
        MistDropdown, MistGridView, MistImage, MistInputField, MistLabel, MistListView,
        MistMenuList, MistPanel, MistPopover, MistProgressBar, MistRadioGroup, MistScrollContent,
        MistScrollView, MistSegmentedActionRow, MistSlider, MistStatusPill, MistSwitch,
        MistTable, MistTabs, MistToast, MistTooltip, MistTrigger, MistTreeNodeSpec, MistTreeView,
    };
    use bevy::{asset::AssetPlugin, ecs::world::CommandQueue};

    #[test]
    fn standard_factory_spawns_full_component_suite() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));

        let factory = StandardMistUiFactory;
        let font = Handle::<Font>::default();
        let image = Handle::<Image>::default();

        let (
            panel,
            label,
            graphic,
            button,
            trigger,
            checkbox,
            radio_group,
            switcher,
            scroll_view,
            scroll_content,
            slider,
            progress,
            dropdown,
            input,
            tooltip,
            tabs,
            dialog,
            badge,
            chip,
            status_pill,
            toast,
            popover,
            context_menu,
            menu_list,
            accordion,
            segmented,
            list_view,
            table,
            tree,
            grid,
        ) = {
            let mut queue = CommandQueue::default();
            let spawned = {
                let world = app.world_mut();
                let mut commands = Commands::new(&mut queue, world);

                let panel = factory.spawn_panel(&mut commands);
                let label = factory.spawn_label(&mut commands, &font, "Signal", 18.0);
                let graphic =
                    factory.spawn_image(&mut commands, image.clone(), Vec2::new(64.0, 64.0));
                let button = factory.spawn_button(&mut commands, &font, "Confirm", 220.0);
                let trigger = factory.spawn_trigger(&mut commands, &font, "Open", 180.0);
                let checkbox = factory.spawn_checkbox(&mut commands, &font, "Track", true);
                let radio_group = factory.spawn_radio_group(
                    &mut commands,
                    &font,
                    220.0,
                    ["Balanced", "Dense", "Signal"],
                    1,
                );
                let switcher = factory.spawn_switch(&mut commands, &font, "Reactive", true);
                let (scroll_view, scroll_content) =
                    factory.spawn_scroll_view(&mut commands, 220.0, 160.0);
                let slider = factory.spawn_slider(&mut commands, 220.0, 0.72);
                let progress = factory.spawn_progress_bar(&mut commands, 220.0, 0.46);
                let dropdown =
                    factory.spawn_dropdown(&mut commands, &font, 220.0, ["English", "中文"]);
                let input = factory.spawn_input_field(
                    &mut commands,
                    &font,
                    260.0,
                    MistInputField::new("operator@rope.dev").with_value("rope"),
                );
                let tooltip =
                    factory.attach_tooltip(&mut commands, button, &font, "Tooltip body", 240.0);
                let tabs = factory.spawn_tabs(
                    &mut commands,
                    &font,
                    240.0,
                    ["Overview", "Nodes", "Logs"],
                    0,
                );
                let dialog = factory.spawn_dialog(&mut commands, &font, "Mist", "Body", 420.0);
                let badge = factory.spawn_badge(&mut commands, &font, "READY");
                let chip = factory.spawn_chip(&mut commands, &font, "Dense Mist", 160.0);
                let status_pill =
                    factory.spawn_status_pill(&mut commands, &font, "Cluster Stable", true);
                let toast =
                    factory.spawn_toast(&mut commands, &font, "Toast", "Body", 320.0);
                let popover =
                    factory.spawn_popover(&mut commands, &font, "Popover", "Body", 280.0);
                let context_menu = factory.spawn_context_menu(
                    &mut commands,
                    &font,
                    220.0,
                    ["Inspect", "Archive", "Retire"],
                );
                let menu_list = factory.spawn_menu_list(
                    &mut commands,
                    &font,
                    220.0,
                    ["Inspect", "Archive", "Retire"],
                );
                let accordion = factory.spawn_accordion(
                    &mut commands,
                    &font,
                    320.0,
                    vec![
                        ("A".to_string(), "Body A".to_string()),
                        ("B".to_string(), "Body B".to_string()),
                    ],
                );
                let segmented = factory.spawn_segmented_action_row(
                    &mut commands,
                    &font,
                    280.0,
                    ["Deploy", "Trace", "Quarantine"],
                );
                let list_view = factory.spawn_list_view(
                    &mut commands,
                    &font,
                    220.0,
                    160.0,
                    ["A", "B", "C"],
                    Some(1),
                );
                let table = factory.spawn_table(
                    &mut commands,
                    &font,
                    420.0,
                    vec!["Node".to_string(), "Load".to_string()],
                    vec![
                        vec!["alpha".to_string(), "72%".to_string()],
                        vec!["beta".to_string(), "58%".to_string()],
                    ],
                );
                let tree = factory.spawn_tree_view(
                    &mut commands,
                    &font,
                    320.0,
                    vec![
                        MistTreeNodeSpec::root("Root"),
                        MistTreeNodeSpec::child("Child", 0),
                    ],
                    Some(0),
                );
                let grid = factory.spawn_grid_view(
                    &mut commands,
                    &font,
                    320.0,
                    2,
                    ["Alpha", "Beta", "Gamma", "Delta"],
                    Some(2),
                );

                (
                    panel,
                    label,
                    graphic,
                    button,
                    trigger,
                    checkbox,
                    radio_group,
                    switcher,
                    scroll_view,
                    scroll_content,
                    slider,
                    progress,
                    dropdown,
                    input,
                    tooltip,
                    tabs,
                    dialog,
                    badge,
                    chip,
                    status_pill,
                    toast,
                    popover,
                    context_menu,
                    menu_list,
                    accordion,
                    segmented,
                    list_view,
                    table,
                    tree,
                    grid,
                )
            };
            queue.apply(app.world_mut());
            spawned
        };

        assert!(app.world().entity(panel).contains::<MistPanel>());
        assert!(app.world().entity(label).contains::<MistLabel>());
        assert!(app.world().entity(graphic).contains::<MistImage>());
        assert!(app.world().entity(button).contains::<MistButton>());
        assert!(app.world().entity(trigger).contains::<MistTrigger>());
        assert!(app.world().entity(checkbox).contains::<MistCheckbox>());
        assert!(app.world().entity(radio_group).contains::<MistRadioGroup>());
        assert!(app.world().entity(switcher).contains::<MistSwitch>());
        assert!(app.world().entity(scroll_view).contains::<MistScrollView>());
        assert!(app.world().entity(scroll_content).contains::<MistScrollContent>());
        assert!(app.world().entity(slider).contains::<MistSlider>());
        assert!(app.world().entity(progress).contains::<MistProgressBar>());
        assert!(app.world().entity(dropdown).contains::<MistDropdown>());
        assert!(app.world().entity(input).contains::<MistInputField>());
        assert!(app.world().entity(tooltip).contains::<MistTooltip>());
        assert!(app.world().entity(tabs).contains::<MistTabs>());
        assert!(app.world().entity(dialog).contains::<MistDialog>());
        assert!(app.world().entity(badge).contains::<MistBadge>());
        assert!(app.world().entity(chip).contains::<MistChip>());
        assert!(app.world().entity(status_pill).contains::<MistStatusPill>());
        assert!(app.world().entity(toast).contains::<MistToast>());
        assert!(app.world().entity(popover).contains::<MistPopover>());
        assert!(app.world().entity(context_menu).contains::<MistContextMenu>());
        assert!(app.world().entity(menu_list).contains::<MistMenuList>());
        assert!(app.world().entity(accordion).contains::<MistAccordion>());
        assert!(app.world().entity(segmented).contains::<MistSegmentedActionRow>());
        assert!(app.world().entity(list_view).contains::<MistListView>());
        assert!(app.world().entity(table).contains::<MistTable>());
        assert!(app.world().entity(tree).contains::<MistTreeView>());
        assert!(app.world().entity(grid).contains::<MistGridView>());
    }
}
