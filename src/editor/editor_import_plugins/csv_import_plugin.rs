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

    fn get_priority(&self) -> f32 {
        1_000.0
    }

    fn get_resource_type(&self) -> GString {
        "TagDictionary".into()
    }

    fn get_save_extension(&self) -> GString {
        "".into()
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
        mut gen_files: Array<GString>,
    ) -> Error {
        let mut tag_dictionary = TagDictionary::new_gd();
        let resource_save_path = source_file.to_string() + ".tres";

        tag_dictionary.take_over_path(resource_save_path.clone().into());

        if let Some(file_contents) = FileAccess::open(source_file.clone(), ModeFlags::READ) {
            let text_content = file_contents.get_as_text();
            let text_content_str = text_content.to_string();
            let lines = text_content_str.lines();
            let mut imported_line_count = 0;

            if lines.clone().count() == 0 {
                return Error::OK;
            }

            lines.for_each(|line| {
                let binding = line.replace(",", ".").replace("..", "");
                let mut tag = binding;

                if tag.ends_with(".") {
                    tag = tag[0..tag.len() - 1].to_string();
                }

                if !tag.is_empty() && tag_dictionary.bind_mut().add_tag(tag.into()) {
                    imported_line_count += 1;
                }
            });

            if imported_line_count > 0 {
                gen_files.push(resource_save_path.into());
                return ResourceSaver::singleton().save(tag_dictionary.upcast());
            }

            return Error::OK;
        }

        Error::ERR_CANT_OPEN
    }
}
