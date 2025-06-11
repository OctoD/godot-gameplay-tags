//
// Created by lasagnaking on 6/11/25.
//

#include "TagDictionaryEditorInspectorPlugin.h"
#include <godot_cpp/classes/file_access.hpp>
#include <godot_cpp/classes/resource_saver.hpp>

using namespace octod::gameplay::tags::editor::inspector_plugins;

bool TagDictionaryEditorInspectorPlugin::_can_handle(Object *p_object) const
{
	return EditorInspectorPlugin::_can_handle(p_object);
}

bool TagDictionaryEditorInspectorPlugin::_parse_property(Object *p_object, Variant::Type p_type, const String &p_name, PropertyHint p_hint_type, const String &p_hint_string, BitField<PropertyUsageFlags> p_usage_flags, bool p_wide)
{
	return EditorInspectorPlugin::_parse_property(p_object, p_type, p_name, p_hint_type, p_hint_string, p_usage_flags, p_wide);
}

void TagDictionaryEditorInspectorPlugin::_bind_methods()
{
	/// binds methods
	ClassDB::bind_method(D_METHOD("handle_tag_dictionary_changed"), &TagDictionaryEditorInspectorPlugin::handle_tag_dictionary_changed);
	ClassDB::bind_method(D_METHOD("handle_tag_dictionary_export"), &TagDictionaryEditorInspectorPlugin::handle_tag_dictionary_export);
}

void TagDictionaryEditorInspectorPlugin::handle_tag_dictionary_changed() const
{
	if (tag_dictionary.is_valid()) {
		ResourceSaver::get_singleton()->save(tag_dictionary);
	}
}

void TagDictionaryEditorInspectorPlugin::handle_tag_dictionary_export() const
{
	if (tag_dictionary.is_null()) {
		return;
	}

	PackedStringArray tags = tag_dictionary->get_tags();
	String output = "";

	for (int i = 0; i < tags.size(); i++) {
		output += tags[i] + "\n";
	}

	const String filename = tag_dictionary->get_path().replace("tres", "csv");
	const Ref<FileAccess> file = FileAccess::open(filename, FileAccess::WRITE);

	ERR_FAIL_COND_MSG(file.is_valid(), "Failed to open file for writing: " + filename);

	if (file.is_valid()) {
		file->store_string(output);
		file->close();
		print_line("Exported tags to " + tag_dictionary->get_path().replace("tres", "csv"));
	}
}