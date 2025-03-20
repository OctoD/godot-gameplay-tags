//
// Created by iamoc on 17/03/2025.
//

// ReSharper disable CppClassCanBeFinal
#ifndef OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_H
#define OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_H

#include <godot_cpp/classes/resource.hpp>

using namespace godot;

namespace octod::gameplay::tags
{
	/// @brief A dictionary of tags.
	class TagDictionary : public Resource
	{
		GDCLASS(TagDictionary, Resource)

	public:
		static const char *SPLIT_CHAR;

		/// @brief Adds a tag to the dictionary.
		/// @returns True if the tag was added, false otherwise.
		bool add_tag(const String &p_tag);

		/// @brief Adds multiple tags to the dictionary.
		/// @param p_tags The tags to add.
		/// @returns The number of tags added.
		unsigned int add_tags(const PackedStringArray &p_tags);

		/// @brief Returns the number of tags in the dictionary.
		/// @returns The number of tags in the dictionary.
		[[nodiscard]] unsigned int count() const;

		/// @brief Finds all tags that match the predicate.
		/// @param p_predicate The predicate to match.
		/// @returns The tags that match the predicate.
		[[nodiscard]] PackedStringArray find(const Callable &p_predicate) const;

		/// @brief Returns the tags associated with a path.
		/// @param p_path The path to get the tags from.
		/// @returns The tags associated with the path.
		[[nodiscard]] PackedStringArray get_tags_from_path(const String &p_path) const;

		/// @brief Returns the dictionary of tags.
		/// @returns The dictionary of tags.
		[[nodiscard]] Dictionary get_tree() const;

		/// @brief Checks if a path has tags.
		/// @param p_path The path to check.
		/// @returns True if the path has tags, false otherwise.
		[[nodiscard]] bool has_path(const String &p_path) const;

		/// @brief Checks if a tag is in the dictionary.
		/// @param p_tag The tag to check.
		/// @returns True if the tag is in the dictionary, false otherwise.
		[[nodiscard]] bool has_tag(const String &p_tag) const;

		/// @brief Checks if the dictionary has some of the tags.
		/// @param p_tags The tags to check.
		/// @returns True if the dictionary has some of the tags, false otherwise.
		[[nodiscard]] bool has_some_tags(const PackedStringArray &p_tags) const;

		/// @brief Checks if the dictionary has none of the tags.
		/// @param p_tags The tags to check.
		/// @returns True if the dictionary has none of the tags, false otherwise.
		[[nodiscard]] bool has_none_tags(const PackedStringArray &p_tags) const;

		/// @brief Checks if none of the tags match the predicate.
		/// @param p_predicate The predicate to match.
		/// @returns True if none of the tags match the predicate, false otherwise.
		[[nodiscard]] bool none(const Callable &p_predicate) const;

		/// @brief Replaces a tag with another tag.
		/// @param p_old_tag The tag to replace.
		/// @param p_new_tag The tag to replace with.
		/// @returns True if the tag was replaced, false otherwise.
		bool replace_tag(const String &p_old_tag, const String &p_new_tag);

		/// @brief Replaces multiple tags with other tags.
		/// @param p_old_tags The tags to replace.
		/// @param p_new_tags The tags to replace with.
		/// @returns The number of tags replaced.
		unsigned int replace_tags(const PackedStringArray &p_old_tags, const PackedStringArray &p_new_tags);

		/// @brief Removes a tag from the dictionary.
		/// @param p_tag The tag to remove.
		/// @returns True if the tag was removed, false otherwise.
		bool remove_tag(const String &p_tag);

		/// @brief Removes a path from the dictionary.
		/// @param p_path The path to remove.
		/// @returns The number of tags removed.
		/// @note This will remove all tags associated with the path.
		unsigned int remove_path(const String &p_path);

		/// @brief Checks if any of the tags match the predicate.
		/// @param p_predicate The predicate to match.
		/// @returns True if any of the tags match the predicate, false otherwise.
		[[nodiscard]] bool some(const Callable &p_predicate) const;

		/// @brief Updates a path with a new path.
		/// @param p_old_path The path to update.
		/// @param p_new_path The new path.
		/// @returns The number of tags updated.
		unsigned int update_path(const String &p_old_path, const String &p_new_path);

	protected:
		/// @brief Binds the methods to Godot.
		static void _bind_methods();

		/// @brief The tags in the dictionary.
		PackedStringArray tags;
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_TAG_DICTIONARY_H
