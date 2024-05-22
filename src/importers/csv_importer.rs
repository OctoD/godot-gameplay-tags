use godot::{engine::file_access::ModeFlags, prelude::*};

use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = RefCounted)]
struct CsvImporter {
    base: Base<RefCounted>,
    content: PackedStringArray,
}

#[godot_api]
impl CsvImporter {
    #[func]
    pub fn from_csv(&mut self, path: GString) {
        self.content = GFile::open(path, ModeFlags::READ)
            .expect("Failed to open file")
            .read_csv_line(",")
            .expect("Failed to read file");
    }

	#[func]
	pub fn to_csv(&self, path: GString) {
		GFile::open(path, ModeFlags::WRITE)
			.expect("Failed to open file")
			.write_csv_line(self.content.clone(), ",")
			.expect("Failed to write file");
	}

    #[func] 
    pub fn to_tag_dictionary(&self) -> Gd<TagDictionary> {
        let mut tag_dictionary = TagDictionary::new_gd();
        
        tag_dictionary.bind_mut().add_tags(self.content.clone());

        tag_dictionary
    }
}
