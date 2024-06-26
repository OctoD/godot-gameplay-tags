use godot::{engine::Engine, prelude::*};
use std::str::FromStr;

pub const SINGLETON_NAME: &str = "TagManager";

#[derive(GodotClass)]
#[class(init, base = Object)]
pub struct TagManager {
    base: Base<Object>,
}

#[godot_api]
impl TagManager {
    fn _add_to_group(&self, mut target: Gd<Node>) {
        target.upcast_mut::<Node>().call(
            "add_to_group".into(),
            &[
                self.get_group_name().to_variant(),
                Engine::singleton().is_editor_hint().to_variant(),
            ],
        );
    }

    fn _remove_from_group(&self, mut target: Gd<Node>) {
        target.upcast_mut::<Node>().call(
            "remove_from_group".into(),
            &[self.get_group_name().to_variant()],
        );
    }

    pub fn _get_tags(&self, target: &Gd<Node>) -> PackedStringArray {
        if target.has_meta(self.get_meta_name()) {
            return PackedStringArray::from_variant(&target.get_meta(self.get_meta_name()));
        }

        return PackedStringArray::new();
    }

    /// Adds a tag to a target node.
    #[func]
    pub fn add_tag(&self, tag: GString, mut target: Gd<Node>) {
        let mut packed_string_array = self._get_tags(&target);

        if !packed_string_array.contains(&tag) {
            packed_string_array.push(tag);
            self._add_to_group(target.clone());
            target.set_meta(self.get_meta_name(), packed_string_array.to_variant());
        }
    }

    /// Adds multiple tags to a target node.
    #[func]
    pub fn add_tags(&self, tags: PackedStringArray, mut target: Gd<Node>) {
        let mut packed_string_array = self._get_tags(&target);

        if tags.len() == 0 {
            return;
        }

        for tag in tags.as_slice() {
            if !packed_string_array.contains(&tag) {
                packed_string_array.push(tag.clone());
            }
        }

        self._add_to_group(target.clone());
        target.set_meta(self.get_meta_name(), packed_string_array.to_variant());
    }

    /// Returns the group name used to store tagged nodes.
    #[func]
    pub fn get_group_name(&self) -> StringName {
        StringName::from_str("octod_ggs_tagged_node_group").expect("Failed to create StringName")
    }

    /// Returns the meta name used to store tags.
    #[func]
    pub fn get_meta_name(&self) -> StringName {
        StringName::from_str("octod_ggs_tagged_node").expect("Failed to create StringName")
    }

    /// Gets all the nodes in with a tag path
    #[func]
    pub fn get_nodes_in_tag_path(&self, target: Gd<Node>, tag_path: String) -> Array<Gd<Node>> {
        let mut output: Array<Gd<Node>> = Array::new();

        if let Some(mut tree) = target.get_tree() {
            let nodes: Array<Gd<Node>> = tree.get_nodes_in_group(self.get_group_name());

            for node in nodes.iter_shared() {
                if self.is_in_path(node.clone(), tag_path.clone()) {
                    output.push(node);
                }
            }
        }

        return output;
    }

    /// Returns all tagged nodes descending from a target node.
    #[func]
    pub fn get_tagged_nodes(&self, target: Gd<Node>) -> Array<Gd<Node>> {
        if let Some(mut tree) = target.get_tree() {
            return tree.get_nodes_in_group(self.get_group_name());
        }

        return Array::new();
    }

    /// Returns all tags of a target node.
    #[func]
    pub fn get_tags(&self, target: Gd<Node>) -> PackedStringArray {
        return self._get_tags(&target);
    }

    #[func]
    pub fn is_in_path(&self, target: Gd<Node>, path: String) -> bool {
        let tags = self.get_tags(target);

        for tag in tags.as_slice().iter() {
            if tag.to_string().contains(&path.to_string()) {
                return true;
            }
        }

        return false;
    }

    /// Checks if a target node has a specific tag.
    #[func]
    fn has_tag(&self, tag: GString, target: Gd<Node>) -> bool {
        return self._get_tags(&target).contains(&tag);
    }

    /// Checks if a target node has all of the specified tags.
    #[func]
    pub fn has_all_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
        return self
            ._get_tags(&target)
            .as_slice()
            .iter()
            .all(|tag| tags.contains(&tag));
    }

    /// Checks if a target node has any of the specified tags.
    #[func]
    pub fn has_some_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
        self._get_tags(&target)
            .as_slice()
            .iter()
            .any(|tag| tags.contains(&tag))
    }

    /// Checks if a target node has none of the specified tags.
    #[func]
    pub fn has_none_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
        self._get_tags(&target)
            .as_slice()
            .iter()
            .all(|tag| !tags.contains(&tag))
    }

    /// Removes a tag from a target node. If the node has no more tags, it will be removed from the tagged nodes group.
    #[func]
    pub fn remove_tag(&self, tag: GString, mut target: Gd<Node>) {
        let mut packed_string_array = self._get_tags(&target);

        if let Some(index) = packed_string_array
            .as_slice()
            .iter()
            .position(|t| tag.eq(t))
        {
            packed_string_array.remove(index);
            target.set_meta(self.get_meta_name(), packed_string_array.to_variant());

            if packed_string_array.len() == 0 {
                target.remove_meta(self.get_meta_name());
                self._remove_from_group(target.clone());
            }
        }
    }

    /// Removes multiple tags from a target node. If the node has no more tags, it will be removed from the tagged nodes group.
    #[func]
    pub fn remove_tags(&self, tags: PackedStringArray, mut target: Gd<Node>) {
        let mut packed_string_array = self._get_tags(&target);

        for tag in tags.as_slice() {
            if let Some(index) = packed_string_array
                .as_slice()
                .iter()
                .position(|t| tag.eq(t))
            {
                packed_string_array.remove(index);
            }
        }

        if packed_string_array.len() == 0 {
            self._remove_from_group(target.clone());
            target.remove_meta(self.get_meta_name());
        } else {
            target.set_meta(self.get_meta_name(), packed_string_array.to_variant());
        }
    }

    /// Sets the tags of a target node. If the node has no more tags, it will be removed from the tagged nodes group.
    #[func]
    pub fn set_tags(&self, tags: PackedStringArray, mut target: Gd<Node>) {
        if tags.len() == 0 {
            self._remove_from_group(target.clone());
            target.remove_meta(self.get_meta_name());
        } else {
            self._add_to_group(target.clone());
            target.set_meta(self.get_meta_name(), tags.to_variant());
        }
    }
}
