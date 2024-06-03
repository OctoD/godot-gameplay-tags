mod docks;
mod editor_import_plugins;
mod inspector_plugins;
mod tag_dictionary_fs;
pub mod ui;

use godot::{
    engine::{editor_plugin::DockSlot, EditorPlugin, IEditorPlugin},
    prelude::*,
};

use self::inspector_plugins::tag_dictionary_editor_inspector_plugin;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base = EditorPlugin)]
pub struct GameplayTagsEditorPlugin {
    base: Base<EditorPlugin>,
    node_tagging_dock: Option<Gd<docks::node_tagging_dock::NodeTaggingDock>>,
    import_plugin_csv: Gd<editor_import_plugins::csv_import_plugin::CsvImportPlugin>,
    tag_dictionary_editor_inspector_plugin:
        Gd<tag_dictionary_editor_inspector_plugin::TagDictionaryEditorInspectorPlugin>,
}

#[godot_api]
impl IEditorPlugin for GameplayTagsEditorPlugin {
    fn enter_tree(&mut self) {
        self.node_tagging_dock = Some(docks::node_tagging_dock::NodeTaggingDock::new_alloc());
        self.import_plugin_csv =
            editor_import_plugins::csv_import_plugin::CsvImportPlugin::new_gd();

        self.to_gd()
            .add_control_to_dock(DockSlot::RIGHT_UL, self.node_tagging_dock.to_variant().to());

        self.to_gd()
            .add_import_plugin(self.import_plugin_csv.clone().upcast());

        self.tag_dictionary_editor_inspector_plugin =
            tag_dictionary_editor_inspector_plugin::TagDictionaryEditorInspectorPlugin::new_gd();

        self.to_gd().add_inspector_plugin(
            self.tag_dictionary_editor_inspector_plugin
                .to_variant()
                .to(),
        );
    }

    fn exit_tree(&mut self) {
        self.to_gd().remove_inspector_plugin(
            self.tag_dictionary_editor_inspector_plugin
                .to_variant()
                .to(),
        );

        self.to_gd()
            .remove_import_plugin(self.import_plugin_csv.to_variant().to());

        self.to_gd()
            .remove_control_from_docks(self.node_tagging_dock.to_variant().to());
    }

    fn get_plugin_name(&self) -> GString {
        "godot_gameplay_tags".into_godot()
    }
}
