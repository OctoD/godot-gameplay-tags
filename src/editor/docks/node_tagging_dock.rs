use godot::{engine::{IVBoxContainer, VBoxContainer}, prelude::*};

use crate::editor::{tag_dictionary_fs::TagDictionaryFs, ui::tag_tree::TagTree};

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
	fn render_tag_trees(&mut self) {
		let mut tag_dictionary_fs = TagDictionaryFs::new_gd();

		tag_dictionary_fs.bind_mut().scan_fs("res://".into());

		let dictionaries = tag_dictionary_fs.bind().get_dictionaries();

		for dict in dictionaries.iter_shared() {
			let mut tree = TagTree::new_alloc();

			tree.bind_mut().set_tag_dictionary(Some(dict.clone()));
			tree.bind_mut().set_selectable(true);

			self.to_gd().add_child(tree.to_variant().to());
		}
	}
}

#[godot_api]
impl IVBoxContainer for NodeTaggingDock {
	fn ready(&mut self) {
		self.to_gd().set_name("Node tags".into());
		self.render_tag_trees();
	}
}
