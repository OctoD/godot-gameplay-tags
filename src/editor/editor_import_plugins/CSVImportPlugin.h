//
// Created by lasagnaking on 6/11/25.
//

#ifndef OCTOD_GAMEPLAY_TAGS_EDITOR_EDITOR_IMPORT_PLUGINS_CSV_IMPORT_PLUGIN_H
#define OCTOD_GAMEPLAY_TAGS_EDITOR_EDITOR_IMPORT_PLUGINS_CSV_IMPORT_PLUGIN_H

#include <godot_cpp/classes/editor_import_plugin.hpp>

using namespace godot;

namespace octod::gameplay::tags::editor::editor_import_plugins
{
	class CSVImportPlugin : public EditorImportPlugin
	{
		GDCLASS(CSVImportPlugin, EditorImportPlugin)

	public:
		/// @brief Returns the import options
		TypedArray<Dictionary> _get_import_options(const String &p_path, int32_t p_preset_index) const override;

		int _get_import_order() const override;

		PackedStringArray _get_recognized_extensions() const override;

		bool _get_option_visibility(const String &p_path, const StringName &p_option_name, const Dictionary &p_options) const override;

		String _get_preset_name(int p_idx) const override;

		float _get_priority() const override;

		String _get_resource_type() const override;

		String _get_save_extension() const override;

		String _get_visible_name() const override;

		Error _import(const String &p_source_file, const String &p_save_path, const Dictionary &p_options, const TypedArray<String> &p_platform_variants, TypedArray<String>& p_gen_files) const override;
	protected:
		/// @brief Binds methods to Godot
		static void _bind_methods();
	};
} // octod

#endif //OCTOD_GAMEPLAY_TAGS_EDITOR_EDITOR_IMPORT_PLUGINS_CSV_IMPORT_PLUGIN_H