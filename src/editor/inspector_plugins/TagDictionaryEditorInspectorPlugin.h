//
// Created by lasagnaking on 6/11/25.
//

#ifndef OCTOD_GAMEPLAY_TAGS_EDITOR_INSPECTOR_PLUGINS_TAG_DICTIONARY_EDITOR_INSPECTOR_PLUGIN_H
#define OCTOD_GAMEPLAY_TAGS_EDITOR_INSPECTOR_PLUGINS_TAG_DICTIONARY_EDITOR_INSPECTOR_PLUGIN_H

#include <godot_cpp/classes/editor_inspector_plugin.hpp>
#include "../../TagDictionary.h"

using namespace godot;

namespace octod::gameplay::tags::editor::inspector_plugins
{
	class TagDictionaryEditorInspectorPlugin : public EditorInspectorPlugin
	{
		GDCLASS(TagDictionaryEditorInspectorPlugin, EditorInspectorPlugin)

	public:
		bool _can_handle(Object *p_object) const override;

		bool _parse_property(Object *p_object, Variant::Type p_type, const String &p_name, PropertyHint p_hint_type, const String &p_hint_string, BitField<PropertyUsageFlags> p_usage_flags, bool p_wide) override;

	protected:
		/// @brief Binds methods to Godot
		static void _bind_methods();

		/// @brief Reference to a TagDictionary
		Ref<TagDictionary> tag_dictionary;

		void handle_tag_dictionary_changed() const;

		void handle_tag_dictionary_export() const;
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_EDITOR_INSPECTOR_PLUGINS_TAG_DICTIONARY_EDITOR_INSPECTOR_PLUGIN_H