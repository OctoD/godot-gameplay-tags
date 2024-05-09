use godot::{engine::file_access::ModeFlags, prelude::*};

#[derive(GodotClass)]
#[class(tool, init, base = Resource)]
struct CsvImporter {
    base: Base<Resource>,
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
}
