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
    pub fn remove_tag(&mut self, tag: GString) {
        if let Some(index) = self.tags.as_slice().iter().position(|t| tag.eq(t)) {
            self.tags.remove(index);
        }
    }
}
