use std::str::FromStr;
use godot::prelude::*;

pub const SINGLETON_NAME: &str = "TagManager";

#[derive(GodotClass)]
#[class(init, base = Object)]
pub struct TagManager {
	base: Base<Object>,
}

#[godot_api]
impl TagManager {
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

		if !packed_string_array.contains(tag.clone()) {
			packed_string_array.push(tag);
			target.add_to_group(self.get_group_name());
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
			if !packed_string_array.contains(tag.clone()) {
				packed_string_array.push(tag.clone());
			}
		}
		
		target.add_to_group(self.get_group_name());
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

	/// Checks if a target node has a specific tag.
	#[func]
	fn has_tag(&self, tag: GString, target: Gd<Node>) -> bool {
		return self.
			_get_tags(&target)
			.contains(tag)
	}

	/// Checks if a target node has all of the specified tags.
	#[func]
	pub fn has_all_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
		return self
			._get_tags(&target)
			.as_slice()
			.iter()
			.all(|tag| tags.contains(tag.clone()))
	}

	/// Checks if a target node has any of the specified tags.
	#[func]
	pub fn has_some_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
		self
			._get_tags(&target)
			.as_slice()
			.iter()
			.any(|tag| tags.contains(tag.clone()))
	}

	/// Checks if a target node has none of the specified tags.
	#[func]
	pub fn has_none_tags(&self, tags: PackedStringArray, target: Gd<Node>) -> bool {
		self
			._get_tags(&target)
			.as_slice()
			.iter()
			.all(|tag| !tags.contains(tag.clone()))
	}

	/// Removes a tag from a target node. If the node has no more tags, it will be removed from the tagged nodes group.
	#[func]
	pub fn remove_tag(&self, tag: GString, mut target: Gd<Node>) {
		let mut packed_string_array = self._get_tags(&target);

		if let Some(index) = packed_string_array.as_slice().iter().position(|t| tag.eq(t)) {
			packed_string_array.remove(index);
			target.set_meta(self.get_meta_name(), packed_string_array.to_variant());

			if packed_string_array.len() == 0 {
				target.remove_meta(self.get_meta_name());
				target.remove_from_group(self.get_group_name());
			}
		}
	}

	/// Removes multiple tags from a target node. If the node has no more tags, it will be removed from the tagged nodes group.
	#[func]
	pub fn remove_tags(&self, tags: PackedStringArray, mut target: Gd<Node>) {
		let mut packed_string_array = self._get_tags(&target);

		for tag in tags.as_slice() {
			if let Some(index) = packed_string_array.as_slice().iter().position(|t| tag.eq(t)) {
				packed_string_array.remove(index);
			}
		}

		if packed_string_array.len() == 0 {
			target.remove_from_group(self.get_group_name());
			target.remove_meta(self.get_meta_name());
		} else {
			target.set_meta(self.get_meta_name(), packed_string_array.to_variant());
		}
	}

	/// Sets the tags of a target node. If the node has no more tags, it will be removed from the tagged nodes group.
	#[func]
	pub fn set_tags(&self, tags: PackedStringArray, mut target: Gd<Node>) {
		if tags.len() == 0 {
			target.remove_from_group(self.get_group_name());
			target.remove_meta(self.get_meta_name());
		} else {
			target.add_to_group(self.get_group_name());
			target.set_meta(self.get_meta_name(), tags.to_variant());
		}
	}
}
