//
// Created by lasagnaking on 6/11/25.
//

#include "CSVImportPlugin.h"
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/resource_saver.hpp>

#include "../../TagDictionary.h"

using namespace octod::gameplay::tags::editor::editor_import_plugins;

TypedArray<Dictionary> CSVImportPlugin::_get_import_options(const String &p_path, int32_t p_preset_index) const
{
	return TypedArray<Dictionary>();
}

int CSVImportPlugin::_get_import_order() const
{
	return 0;
}

PackedStringArray CSVImportPlugin::_get_recognized_extensions() const
{
	PackedStringArray result;
	result.push_back("csv");
	return result;
}

bool CSVImportPlugin::_get_option_visibility(const String &p_path, const StringName &p_option_name, const Dictionary &p_options) const
{
	return true;
}

String CSVImportPlugin::_get_preset_name(int p_idx) const
{
	return "ggt_importer_csv";
}

float CSVImportPlugin::_get_priority() const
{
	return 1000.0;
}

String CSVImportPlugin::_get_resource_type() const
{
	return TagDictionary::get_class_static();
}

String CSVImportPlugin::_get_save_extension() const
{
	return "";
}

String CSVImportPlugin::_get_visible_name() const
{
	return "Gameplay Tags Importer";
}

Error CSVImportPlugin::_import(
		const String &p_source_file,
		const String &p_save_path,
		const Dictionary &p_options,
		const TypedArray<String> &p_platform_variants,
		TypedArray<String>& p_gen_files
		) const
{
	Ref<TagDictionary> tag_dictionary;
	const String resource_save_path = p_source_file + String(".tres");

	tag_dictionary.instantiate();
	tag_dictionary->take_over_path(resource_save_path);

	const Ref<FileAccess> file_access = FileAccess::open(p_source_file, FileAccess::READ);

	if (file_access.is_null()) {
		return ERR_CANT_OPEN;
	}

	PackedStringArray lines = file_access->get_as_text().split("\n");
	unsigned int lines_imported = 0;

	for (int i = 0; i < lines.size(); i++) {
		String tag = lines[i].replace(",", ".").replace("..", "");

		if (tag.ends_with(".")) {
			tag = tag.substr(0, tag.length() - 1);
		}

		if (!tag.is_empty() && tag_dictionary->add_tag(tag)) {
			lines_imported++;
		}
	}

	if (lines_imported > 0) {
		p_gen_files.append(resource_save_path);
		return ResourceSaver::get_singleton()->save(tag_dictionary, resource_save_path);
	}

	return OK;
}

void CSVImportPlugin::_bind_methods()
{
}