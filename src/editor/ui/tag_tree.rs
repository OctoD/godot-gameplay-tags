use godot::{
    engine::{control::SizeFlags, ITree, Tree, TreeItem},
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

    pub fn _set_editable(&mut self, editable: bool) {
        self.editable = editable;
    }

    pub fn _set_tag_dictionary(&mut self, tag_dictionary: Option<Gd<TagDictionary>>) {
        self.tag_dictionary = tag_dictionary;
    }

    /// Gets whether the tag tree is editable.
    #[func]
    pub fn get_editable(&self) -> bool {
        self.editable
    }

    /// Gets the tag dictionary to display.
    #[func]
    pub fn get_tag_dictionary(&self) -> Option<Gd<TagDictionary>> {
        self.tag_dictionary.clone()
    }

    /// Sets whether the tag tree is editable.
    #[func]
    pub fn set_editable(&mut self, editable: bool) {
        self._set_editable(editable);
        self.render_tree();
    }

    /// Sets the tag dictionary to display.
    #[func]
    pub fn set_tag_dictionary(&mut self, tag_dictionary: Option<Gd<TagDictionary>>) {
        self._set_tag_dictionary(tag_dictionary);
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
                    format!(
                        "{}{}{}",
                        tag_path.to_string(),
                        SPLIT_CHAR,
                        new_tag_name
                    )
                };

                tag_dictionary
                    .bind_mut()
                    .add_tag(new_tag.clone().to_godot());

                self.render_tree();
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

    fn render_tree(&mut self) {
        self.to_gd().clear();

        let mut root = self
            .to_gd()
            .call("create_item".into(), &[])
            .to::<Gd<TreeItem>>();

        root.set_meta(TAG_PATH_META.into(), "".to_variant());

        if let Some(tag_dictionary) = self.tag_dictionary.clone() {
            root.set_text(0, tag_dictionary.get_name());
        }

        if self.editable {
            self.set_tree_item_editable_icon(root.clone());
        }

        if let Some(tag_dictionary) = self.get_tag_dictionary() {
            self.render_dictionary(tag_dictionary.bind().get_tree(), root, "".into())
        }
    }

    fn set_tree_item_editable_icon(&self, mut item: Gd<TreeItem>) {
        if let Some(icon) = self.to_gd().get_theme_icon("Edit".into()) {
            item.add_button(0, icon);
            item.set_button_tooltip_text(0, 0, "Edit tag".into());
        }

        if let Some(icon) = self.to_gd().get_theme_icon("Remove".into()) {
            item.add_button(0, icon);
            item.set_button_tooltip_text(0, 1, "Remove tag".into());
        }
    }
}
