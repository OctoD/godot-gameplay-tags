use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base = Resource)]
pub struct TagDictionary {
	base: Base<Resource>,
	#[export]
	tags: PackedStringArray,
}

#[godot_api]
impl TagDictionary {
	#[func]
	pub fn add_tag(&mut self, tag: GString) {
		if !self.tags.contains(tag.clone()) {
			self.tags.push(tag);
		}
	}

	#[func]
	pub fn has_tag(&self, tag: GString) -> bool {
		self.tags.contains(tag)
	}

	#[func]
	pub fn has_some_tags(&self, tags: PackedStringArray) -> bool {
		tags.as_slice().iter().any(|tag| self.tags.contains(tag.clone()))
	}

	#[func]
	pub fn has_none_tags(&self, tags: PackedStringArray) -> bool {
		tags.as_slice().iter().all(|tag| !self.tags.contains(tag.clone()))
	}

	#[func]
	pub fn remove_tag(&mut self, tag: GString) {
		if let Some(index) = self.tags.as_slice().iter().position(|t| tag.eq(t)) {
			self.tags.remove(index);
		}
	}
}
