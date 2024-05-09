mod inspector_plugins;
mod tag_dictionary_fs;
pub mod ui;

use godot::{
    engine::{EditorPlugin, IEditorPlugin},
    prelude::*,
};

use self::inspector_plugins::tag_dictionary_editor_inspector_plugin;

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base = EditorPlugin)]
pub struct GameplayTagsEditorPlugin {
    base: Base<EditorPlugin>,
    tag_dictionary_editor_inspector_plugin:
        Gd<tag_dictionary_editor_inspector_plugin::TagDictionaryEditorInspectorPlugin>,
}

#[godot_api]
impl IEditorPlugin for GameplayTagsEditorPlugin {
    fn enter_tree(&mut self) {
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
    }

    fn get_plugin_name(&self) -> GString {
        "godot_gameplay_tags".into_godot()
    }
}
