pub mod ui;

use godot::{engine::{EditorPlugin, IEditorPlugin}, prelude::*};

#[derive(GodotClass)]
#[class(tool, init, editor_plugin, base = EditorPlugin)]
pub struct GameplayTagsEditorPlugin {
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for GameplayTagsEditorPlugin {
    fn enter_tree(&mut self) {
        // Perform typical plugin operations here.
    }

    fn exit_tree(&mut self) {
        // Perform typical plugin operations here.
    }
}
