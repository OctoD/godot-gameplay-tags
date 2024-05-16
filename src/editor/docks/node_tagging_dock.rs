use godot::{
    engine::{EditorInterface, IVBoxContainer, VBoxContainer},
    prelude::*,
};

use crate::{
    editor::{tag_dictionary_fs::TagDictionaryFs, ui::tag_tree::TagTree},
    tag_manager::TagManager,
};

#[derive(GodotClass)]
#[class(tool, init, base = VBoxContainer)]
pub struct NodeTaggingDock {
    base: Base<VBoxContainer>,
}

#[godot_api]
impl NodeTaggingDock {
    #[func]
    fn on_visibility_changed(&mut self) {
        if !self.to_gd().is_visible() {
            return;
        }

        // clear all children when the dock is visible
        for mut child in self.to_gd().get_children().iter_shared() {
            child.queue_free();
        }

        self.render_tag_trees();
    }

    #[func]
    fn on_tags_added(&mut self, tags: PackedStringArray) {
        let tag_manager = TagManager::new_alloc();

        if let Some(mut selection) = EditorInterface::singleton().get_selection() {
            for node in selection.get_selected_nodes().iter_shared() {
                tag_manager.bind().add_tags(tags.clone(), node);
            }
            EditorInterface::singleton().mark_scene_as_unsaved();
        }
    }

    #[func]
    fn on_tags_removed(&mut self, tags: PackedStringArray) {
        let tag_manager = TagManager::new_alloc();

        if let Some(mut selection) = EditorInterface::singleton().get_selection() {
            for node in selection.get_selected_nodes().iter_shared() {
                tag_manager.bind().remove_tags(tags.clone(), node);
            }
            EditorInterface::singleton().mark_scene_as_unsaved();
        }
    }

    #[func]
    fn render_tag_trees(&mut self) {
		// let's start with cleaning up child elements
		for mut child in self.to_gd().get_children().iter_shared() {
			child.queue_free();
		}

        let mut tag_dictionary_fs = TagDictionaryFs::new_gd();
        let tag_manager = TagManager::new_alloc();

        tag_dictionary_fs.bind_mut().scan_fs("res://".into());

        let dictionaries = tag_dictionary_fs.bind().get_dictionaries();

        for dict in dictionaries.iter_shared() {
            let mut tree = TagTree::new_alloc();

            if let Some(mut selection) = EditorInterface::singleton().get_selection() {
                for node in selection.get_selected_nodes().iter_shared() {
                    tree.bind_mut()
                        .set_selected_tags(tag_manager.bind().get_tags(node));
                }
            }

            tree.bind_mut().set_tag_dictionary(Some(dict.clone()));
            tree.bind_mut().set_selectable(true);
            tree.connect(
                "tags_added".into(),
                Callable::from_object_method(&self.to_gd(), "on_tags_added"),
            );
            tree.connect(
                "tags_removed".into(),
                Callable::from_object_method(&self.to_gd(), "on_tags_removed"),
            );

            self.to_gd().add_child(tree.to_variant().to());
        }
    }
}

#[godot_api]
impl IVBoxContainer for NodeTaggingDock {
    fn ready(&mut self) {
        self.to_gd().set_name("Node tags".into());

		if let Some(mut selection) = EditorInterface::singleton().get_selection() {
			selection.connect(
				"selection_changed".into(),
				Callable::from_object_method(&self.to_gd(), "render_tag_trees"),
			);
		}
		
    }
}
