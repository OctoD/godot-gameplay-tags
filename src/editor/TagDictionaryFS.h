//
// Created by Lasagnaking on 19/03/2025.
//

#ifndef OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_FS_H
#define OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_FS_H

#include <godot_cpp/classes/ref_counted.hpp>

#include "../TagDictionary.h"

using namespace godot;

namespace octod::gameplay::tags::editor
{
	/// @brief Searches the file system for tag dictionaries.
	class TagDictionaryFS final : public RefCounted
	{
		GDCLASS(TagDictionaryFS, RefCounted)

	public:
		/// @brief Returns the singleton instance.
		static TagDictionaryFS *get_singleton();

		/// @brief Returns the tag dictionaries.
		[[nodiscard]] TypedArray<TagDictionary> get_dictionaries() const;

		/// @brief Scans the file system for tag dictionaries.
		void scan_file_system(const String &p_from_directory);

	protected:
		/// @brief Binds methods to godot.
		static void _bind_methods();

		/// @brief Reads a directory recursively and returns its contents.
		static PackedStringArray _read_directory_recursive(const String &p_directory);

		/// @brief An array of tag dictionaries.
		TypedArray<TagDictionary> dictionaries;
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_FS_H
