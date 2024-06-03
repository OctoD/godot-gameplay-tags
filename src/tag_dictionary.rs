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
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn add_tags(&mut self, tags: PackedStringArray) {
        let mut count = 0;

        for tag in tags.as_slice() {
            if !self.tags.contains(&tag) {
                self.tags.push(tag.clone());
                count += 1;
            }
        }

        if count > 0 {
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn count(&self) -> u64 {
        self.tags.len() as u64
    }

    #[func]
    pub fn find(&self, predicate: Callable) -> PackedStringArray {
        let mut result = PackedStringArray::new();

        for tag in self.tags.as_slice() {
            let mut args = VariantArray::new();

            args.push(tag.to_variant());

            if predicate.callv(args).booleanize() {
                result.push(tag.clone());
            }
        }

        result
    }

    #[func]
    pub fn get_tags_from_path(&self, path: GString) -> PackedStringArray {
        let mut out = PackedStringArray::new();

        for tag in self.tags.as_slice() {
            if tag.to_string().starts_with(path.to_string().as_str()) {
                out.push(tag.clone());
            }
        }

        out
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
    pub fn has_path(&self, path: String) -> bool {
        self.tags
            .as_slice()
            .iter()
            .any(|tag| tag.to_string().starts_with(&path))
    }

    #[func]
    pub fn has_tag(&self, tag: GString) -> bool {
        self.tags.contains(&tag)
    }

    #[func]
    pub fn has_some_tags(&self, tags: PackedStringArray) -> bool {
        tags.as_slice().iter().any(|tag| self.tags.contains(&tag))
    }

    #[func]
    pub fn has_none_tags(&self, tags: PackedStringArray) -> bool {
        tags.as_slice().iter().all(|tag| !self.tags.contains(&tag))
    }

    #[func]
    pub fn none(&self, predicate: Callable) -> bool {
        self.tags.as_slice().iter().all(|tag| {
            let mut args = VariantArray::new();

            args.push(tag.to_variant());

            !predicate.callv(args).booleanize()
        })
    }

    #[func]
    pub fn replace_tag(&mut self, old_tag: String, new_tag: String) {
        if let Some(index) = self
            .tags
            .as_slice()
            .iter()
            .position(|t| old_tag == t.to_string())
        {
            self.tags[index] = new_tag.into();
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn replace_tags(&mut self, old_tags: PackedStringArray, new_tags: PackedStringArray) {
        let mut count = 0;

        for old_tag in old_tags.as_slice() {
            if let Some(index) = self.tags.as_slice().iter().position(|t| old_tag == t) {
                if let Some(tag) = new_tags.get(index) {
                    self.tags[index] = tag;
                }
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
    pub fn remove_path(&mut self, path: String) {
        let mut count = 0;

        for tag in self.tags.clone().as_slice() {
            if tag.to_string().starts_with(&path) {
                self.remove_tag(tag.to_owned());
                count += 1;
            }
        }

        if count > 0 {
            self.base_mut().emit_changed();
        }
    }

    #[func]
    pub fn some(&self, predicate: Callable) -> bool {
        self.tags.as_slice().iter().any(|tag| {
            let mut args = VariantArray::new();

            args.push(tag.to_variant());

            predicate.callv(args).booleanize()
        })
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
