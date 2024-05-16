use godot::{
    engine::{control::SizeFlags, tree_item::TreeCellMode, DisplayServer, ITree, Tree, TreeItem},
    prelude::*,
};

use crate::tag_dictionary::{TagDictionary, SPLIT_CHAR};

const TAG_PATH_META: &str = "ggt_tag_dictionary_path";

#[derive(GodotClass)]
#[class(tool, init, base = Tree)]
pub struct TagTree {
    base: Base<Tree>,
    /// Whether the tag tree is editable.
    editable: bool,
    /// Whether the tag tree is selectable.
    selectable: bool,
    /// the selected tags
    selected_tags: PackedStringArray,
    /// The tag dictionary to display.
    tag_dictionary: Option<Gd<TagDictionary>>,
}

#[godot_api]
impl ITree for TagTree {
    fn ready(&mut self) {
        self.to_gd().connect(
            StringName::from("button_clicked"),
            Callable::from_object_method(&self.to_gd(), "_on_button_clicked"),
        );
        self.to_gd().connect(
            StringName::from("item_selected"),
            Callable::from_object_method(&self.to_gd(), "_on_item_selected"),
        );
        self.to_gd().connect(
            StringName::from("item_edited"),
            Callable::from_object_method(&self.to_gd(), "_on_tag_edited"),
        );
        self.to_gd().set_v_scroll_enabled(true);
        self.to_gd().set_v_size_flags(SizeFlags::EXPAND_FILL);
        self.render_tree();
    }
}

#[godot_api]
impl TagTree {
    #[signal]
    pub fn tag_path_edited(old_tag: String, new_tag: String);
    #[signal]
    pub fn tag_path_removed(tag: String);
    #[signal]
    pub fn tags_added(tags: PackedStringArray);
    #[signal]
    pub fn tags_removed(tags: PackedStringArray);

    /// Gets whether the tag tree is editable.
    #[func]
    pub fn get_editable(&self) -> bool {
        self.editable
    }

    #[func]
    pub fn get_selectable(&self) -> bool {
        self.selectable
    }

    #[func]
    pub fn get_selected_tags(&self) -> PackedStringArray {
        self.selected_tags.clone()
    }

    /// Gets the tag dictionary to display.
    #[func]
    pub fn get_tag_dictionary(&self) -> Option<Gd<TagDictionary>> {
        self.tag_dictionary.clone()
    }

    #[func]
    pub fn is_path_selected(&self, _path: GString) -> bool {
        self.selected_tags
            .as_slice()
            .iter()
            .any(|x| x.to_string().starts_with(_path.to_string().as_str()))
    }

    /// Sets whether the tag tree is editable.
    #[func]
    pub fn set_editable(&mut self, editable: bool) {
        self.editable = editable;
        self.render_tree();
    }

    /// Sets whether the tag tree is selectable.
    #[func]
    pub fn set_selectable(&mut self, selectable: bool) {
        self.selectable = selectable;
        self.render_tree();
    }

    #[func]
    pub fn set_selected_tags(&mut self, selected_tags: PackedStringArray) {
        self.selected_tags = selected_tags;
        self.render_tree();
    }

    /// Sets the tag dictionary to display.
    #[func]
    pub fn set_tag_dictionary(&mut self, tag_dictionary: Option<Gd<TagDictionary>>) {
        self.tag_dictionary = tag_dictionary;
        self.render_tree();
    }

    #[func]
    fn _on_button_clicked(
        &mut self,
        tree_item: Gd<TreeItem>,
        _column_id: i32,
        id: i32,
        _mouse_button_index: i32,
    ) {
        let tag_path = tree_item.clone().get_meta(StringName::from(TAG_PATH_META));

        if let Some(mut tag_dictionary) = self.tag_dictionary.clone() {
            if id == 1 {
                tag_dictionary.bind_mut().remove_path(tag_path.to_string());
                self.to_gd()
                    .emit_signal(StringName::from("tag_path_removed"), &[tag_path]);

                self.render_tree();
            } else {
                let child_count = tree_item.clone().get_child_count();
                let new_tag_name = String::from("new_tag_") + &child_count.to_string();
                let new_tag = if tag_path.clone().to_string().is_empty() {
                    new_tag_name
                } else {
                    format!("{}{}{}", tag_path.to_string(), SPLIT_CHAR, new_tag_name)
                };

                tag_dictionary
                    .bind_mut()
                    .add_tag(new_tag.clone().to_godot());

                self.render_tree();
            }
        }
    }

    #[func]
    fn _on_item_selected(&mut self) {
        if let Some(item) = self.to_gd().get_selected() {
            DisplayServer::singleton()
                .clipboard_set(item.get_meta(TAG_PATH_META.into()).to_string().into());

            if self.selectable {
                let mut found_tags = PackedStringArray::new();
                let mut tags_added = PackedStringArray::new();
                let mut tags_removed = PackedStringArray::new();
                let tag_path = item.get_meta(TAG_PATH_META.into()).to_string();
                let is_selected = item.is_checked(0);

                if let Some(tag_dictionary) = self.tag_dictionary.clone() {
                    found_tags = tag_dictionary.bind().get_tags_from_path(tag_path.clone().into());
                }

                for tag in found_tags.as_slice().iter() {
                    if is_selected {
                        if let Some(index) = self.selected_tags.find(tag.clone(), Some(0)) {
                            self.selected_tags.remove(index);
                            tags_removed.push(tag.clone());
                        }
                    } else {
                        if !self.selected_tags.contains(tag.clone()) {
                            self.selected_tags.push(tag.clone());
                            tags_added.push(tag.clone());
                        }
                    }
                }

                if !tags_added.clone().is_empty() {
                    self.to_gd().emit_signal("tags_added".into(), &[tags_added.to_variant()]);
                }

                if !tags_removed.clone().is_empty() {
                    self.to_gd().emit_signal("tags_removed".into(), &[tags_removed.to_variant()]);
                }

                self.to_gd().call_deferred("render_tree".into(), &[]);
            }
        }
    }

    #[func]
    fn _on_tag_edited(&mut self) {
        if let Some(edited) = self.to_gd().get_edited() {
            let meta_path = edited.get_meta(TAG_PATH_META.into()).to_string();
            let edited_chunk = edited.get_text(0).to_string();

            if meta_path == edited_chunk {
                return;
            }

            // takes the meta_path except for the last element
            let parent_path = meta_path
                .rsplit_once(SPLIT_CHAR)
                .map(|(a, _)| a)
                .unwrap_or("");
            let new_path = if parent_path.is_empty() {
                edited_chunk.clone()
            } else {
                format!("{}{}{}", parent_path, SPLIT_CHAR, edited_chunk.clone())
            };

            if let Some(mut tag_dictionary) = self.tag_dictionary.clone() {
                tag_dictionary
                    .bind_mut()
                    .update_path(meta_path.clone(), new_path.clone());
            }

            self.to_gd().emit_signal(
                "tag_path_edited".into(),
                &[meta_path.to_variant(), new_path.to_variant()],
            );
            self.render_tree();
        }
    }

    fn render_dictionary(&self, dictionary: Dictionary, mut parent: Gd<TreeItem>, path: GString) {
        let keys = dictionary.keys_array();

        for key in keys.iter_shared() {
            let keystring = key.to_string();

            if let Some(variant) = dictionary.get(keystring.clone()) {
                match Dictionary::try_from_variant(&variant) {
                    Ok(dict) => {
                        let new_path = if path.is_empty() {
                            keystring.clone()
                        } else {
                            format!("{}{}{}", path, SPLIT_CHAR, keystring.clone())
                        };

                        let mut item = parent.call("create_child".into(), &[]).to::<Gd<TreeItem>>();

                        if self.selectable {
                            self.set_tree_item_checkable(
                                item.clone(),
                                path.clone(),
                                keystring.clone().into(),
                            );
                        }

                        if self.editable {
                            item.set_editable(0, true);
                            self.set_tree_item_editable_icon(item.clone());
                        }

                        item.set_meta(TAG_PATH_META.into(), new_path.to_variant());
                        item.set_text(0, keystring.clone().to_godot());
                        item.set_tooltip_text(0, new_path.clone().into());

                        self.render_dictionary(dict, item, new_path.clone().into());
                    }
                    Err(convert_error) => {
                        godot::engine::utilities::printerr(
                            convert_error.to_string().to_variant(),
                            &[],
                        );
                    }
                }
            }
        }
    }

    #[func]
    fn render_tree(&mut self) {
        self.to_gd().clear();

        let mut root = self
            .to_gd()
            .call("create_item".into(), &[])
            .to::<Gd<TreeItem>>();

        root.set_meta(TAG_PATH_META.into(), "".to_variant());

        if let Some(tag_dictionary) = self.tag_dictionary.clone() {
            root.set_text(0, tag_dictionary.get_path());
        }

        if self.selectable {
            self.set_tree_item_checkable(root.clone(), "".into(), "".into());
        }

        if self.editable {
            self.set_tree_item_editable_icon(root.clone());
        }

        if let Some(tag_dictionary) = self.get_tag_dictionary() {
            self.render_dictionary(tag_dictionary.bind().get_tree(), root, "".into())
        }
    }

    fn set_tree_item_checkable(
        &self,
        mut item: Gd<TreeItem>,
        p_current_path: GString,
        key: GString,
    ) {
        let new_path = if p_current_path.is_empty() {
            key.clone().to_string()
        } else {
            format!("{}{}{}", p_current_path, SPLIT_CHAR, key.clone())
        };

        item.set_cell_mode(0, TreeCellMode::CHECK);

        if new_path.is_empty() {
            item.set_checked(0, false);
        } else {
            item.set_checked(0, self.is_path_selected(new_path.into()));
        }
    }

    fn set_tree_item_editable_icon(&self, mut item: Gd<TreeItem>) {
        if let Some(icon) = self.to_gd().get_theme_icon("Add".into()) {
            item.add_button(0, icon);
            item.set_button_tooltip_text(0, 0, "Add tag".into());
        }

        if let Some(icon) = self.to_gd().get_theme_icon("Remove".into()) {
            item.add_button(0, icon);
            item.set_button_tooltip_text(0, 1, "Remove tag and descendants".into());
        }
    }
}
