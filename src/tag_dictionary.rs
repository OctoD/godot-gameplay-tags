use godot::prelude::*;

pub const SPLIT_CHAR: char = '.';

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
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn add_tags(&mut self, tags: PackedStringArray) {
        let mut count = 0;

        for tag in tags.as_slice() {
            if !self.tags.contains(tag.clone()) {
                self.tags.push(tag.clone());
                count += 1;
            }
        }

        if count > 0 {
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn get_tree(&self) -> Dictionary {
        let root = Dictionary::new();
        let mut tags = self.tags.clone();

        tags.sort();

        for tag in tags.as_slice() {
            let cloned = tag.clone().to_string();
            let chunks = cloned.split(SPLIT_CHAR);
            let mut current = root.clone();

            for chunk in chunks {
                // if current has the key, pass, otherwise current is a new dictionary
                match current.get(chunk) {
					Some(_dict) => {
						current = _dict.to();
					}
					None => {
						let new_dict = Dictionary::new();
						current.set(chunk, new_dict.clone());
						current = new_dict;
					}
				}
            }
        }

        root
    }

    #[func]
    pub fn has_tag(&self, tag: GString) -> bool {
        self.tags.contains(tag)
    }

    #[func]
    pub fn has_some_tags(&self, tags: PackedStringArray) -> bool {
        tags.as_slice()
            .iter()
            .any(|tag| self.tags.contains(tag.clone()))
    }

    #[func]
    pub fn has_none_tags(&self, tags: PackedStringArray) -> bool {
        tags.as_slice()
            .iter()
            .all(|tag| !self.tags.contains(tag.clone()))
    }

    #[func]
    pub fn replace_tag(&mut self, old_tag: String, new_tag: String) {
        if let Some(index) = self.tags.as_slice().iter().position(|t| old_tag == t.to_string()) {
            self.tags.set(index, new_tag.into());
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn replace_tags(&mut self, old_tags: PackedStringArray, new_tags: PackedStringArray) {
        let mut count = 0;

        for old_tag in old_tags.as_slice() {
            if let Some(index) = self.tags.as_slice().iter().position(|t| old_tag == t) {
                self.tags.set(index, new_tags.get(index).to_string().into());
                count += 1;
            }
        }

        if count > 0 {
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn remove_tag(&mut self, tag: GString) {
        if let Some(index) = self.tags.as_slice().iter().position(|t| tag.eq(t)) {
            self.tags.remove(index);
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn update_path(&mut self, old_path: String, new_path: String) {
        let mut count = 0;

        for tag in self.tags.clone().as_slice() {
            if tag.to_string().starts_with(&old_path) {
                let new_tag = tag.to_string().replace(&old_path, &new_path);
                self.replace_tag(tag.into(), new_tag.into());
                count += 1;
            }
        }

        if count > 0 {
            self.base_mut().emit_changed();
        }
    }
}
