use godot::{
    engine::{file_access::ModeFlags, FileAccess, IEditorImportPlugin, ResourceSaver},
    global::Error,
    prelude::*,
};

use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, hidden, init, base = EditorImportPlugin)]
pub struct CsvImportPlugin {}

#[godot_api]
impl IEditorImportPlugin for CsvImportPlugin {
    fn get_import_options(&self, _path: GString, _preset_index: i32) -> Array<Dictionary> {
        Array::new() as Array<Dictionary>
    }

    fn get_import_order(&self) -> i32 {
        0
    }

    fn get_importer_name(&self) -> GString {
        "ggt_importer_csv".into()
    }

    fn get_recognized_extensions(&self) -> PackedStringArray {
        let mut out = PackedStringArray::new();
        out.push("csv".into());
        out
    }

    fn get_option_visibility(
        &self,
        _path: GString,
        _option_name: StringName,
        _options: Dictionary,
    ) -> bool {
        true
    }

    fn get_preset_count(&self) -> i32 {
        0
    }

    fn get_preset_name(&self, _preset_index: i32) -> GString {
        "".into()
    }

    fn get_resource_type(&self) -> GString {
        "TagDictionary".into()
    }

    fn get_save_extension(&self) -> GString {
        "csv".into()
    }

    fn get_visible_name(&self) -> GString {
        "Gameplay Tags Importer".into()
    }

    fn import(
        &self,
        source_file: GString,
        _save_path: GString,
        _options: Dictionary,
        _platform_variants: Array<GString>,
        _gen_files: Array<GString>,
    ) -> Error {
        let mut tag_dictionary = TagDictionary::new_gd();

        tag_dictionary.set_path(source_file.to_string().replace("csv", "tres").into());

        if let Some(file_contents) = FileAccess::open(source_file.clone(), ModeFlags::READ) {
            let text_content = file_contents.get_as_text();

            text_content.to_string().lines().for_each(|line| {
                let binding = line.replace(",", ".").replace("..", "");
                let mut tag = binding;

                if tag.ends_with(".") {
                    tag = tag[0..tag.len() - 1].to_string();
                }

                if !tag.is_empty() {
                    tag_dictionary.bind_mut().add_tag(tag.into());
                }
            });

            return ResourceSaver::singleton().save(tag_dictionary.upcast());
        }

        Error::ERR_CANT_OPEN
    }
}
