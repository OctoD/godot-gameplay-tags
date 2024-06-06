use godot::{
    engine::{DirAccess, ResourceLoader},
    prelude::*,
};

use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = RefCounted)]
pub struct TagDictionaryFs {
    base: Base<RefCounted>,
    dictionaries: Array<Gd<TagDictionary>>,
}

#[godot_api]
impl TagDictionaryFs {
    fn _read_dir_recursive(&self, dir: &str) -> PackedStringArray {
        let mut output = PackedStringArray::new();

        if let Some(mut dir_access) = DirAccess::open(dir.into_godot()) {
            dir_access.list_dir_begin();

            let mut filename = dir_access.get_next();

            while !filename.is_empty() {
                if dir_access.current_is_dir() {
                    if !filename.to_string().starts_with(".") {
                        let sub_dir = format!("{}/{}", dir, filename);
                        let sub_dir_files = self._read_dir_recursive(&sub_dir);

                        for file in sub_dir_files.as_slice() {
                            output.push(file.to_godot().clone());
                        }
                    }
                } else {
                    output.push(GString::from(format!("{}/{}", dir, filename)));
                }

                filename = dir_access.get_next();
            }
        }

        output
    }

    /// Returns the stored dictionaries.
    #[func]
    pub fn get_dictionaries(&self) -> Array<Gd<TagDictionary>> {
        self.dictionaries.clone()
    }

    #[func]
    pub fn scan_fs(&mut self, from_dir: GString) {
        let resources = self._read_dir_recursive(&from_dir.to_string());
        let mut loader = ResourceLoader::singleton();

        for resource_path in resources.as_slice() {
            if resource_path.to_string().ends_with(".res")
                || resource_path.to_string().ends_with(".tres")
            {
                if let Some(resource) = loader.load(resource_path.clone()) {
                    if let Ok(dictionary) = resource.try_cast::<TagDictionary>() {
                        self.dictionaries.push(dictionary);
                    }
                }
            }
        }
    }
}
