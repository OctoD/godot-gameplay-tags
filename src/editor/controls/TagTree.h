//
// Created by iamoc on 20/03/2025.
//

#ifndef OCTOD_GAMEPLAY_TAGS_EDITOR_CONTROLS_TAG_TREE_H
#define OCTOD_GAMEPLAY_TAGS_EDITOR_CONTROLS_TAG_TREE_H

#include <godot_cpp/classes/tree.hpp>

namespace octod::gameplay::tags
{
	class TagDictionary;
}

using namespace godot;

namespace octod::gameplay::tags::controls::TagTree
{
	class TagTree final : public Tree
	{
		GDCLASS(TagTree, Tree)

	public:
		/// @brief Called when the node enters
		void _ready() override;

		/// @brief Deselects many tags at once
		void deselect_many_tags(const PackedStringArray &p_tags);

		/// @brief Deselects one tag
		void deselect_tag(const String &p_tag);

		/// @brief Determines if the tree is editable.
		/// @returns True if editable, false otherwise.
		[[nodiscard]] bool get_editable() const;

		/// @brief Determines if the tree is selectable.
		/// @returns True if selectable, false otherwise.
		[[nodiscard]] bool get_selectable() const;

		/// @brief Returns the selected tags.
		/// @returns An array of selected tags.
		[[nodiscard]] PackedStringArray get_selected_tags() const;

		/// @brief Returns the tag dictionary if any.
		/// @returns A pointer to the current tag dictionary.
		[[nodiscard]] TagDictionary *get_tag_dictionary() const;

		/// @brief Checks if a path is selected.
		/// @param p_tag_path The path to check.
		/// @returns True if selected, false otherwise.
		[[nodiscard]] bool is_path_selected(const String &p_tag_path) const;

		/// @brief Selects many tags.
		/// @param p_tags the tags to select.
		void select_many_tags(const PackedStringArray &p_tags);

		/// @brief Selects one tag.
		/// @param p_tag the tag to select.
		void select_tag(const String &p_tag);

		/// @brief Enables or disables the tree to be modified.
		/// @param p_editable True to make it editable, false otherwise.
		void set_editable(bool p_editable);

		/// @brief Enables or disables the tree to be selected.
		/// @param p_selectable True to make it selectable, false otherwise.
		void set_selectable(bool p_selectable);

		/// @brief Sets the tag dictionary.
		/// @param p_tag_dictionary the tag dictionary to set.
		void set_tag_dictionary(TagDictionary *p_tag_dictionary);

	protected:
		/// @brief Binds methods to Godot
		static void _bind_methods();

		/// @brief Determines if the tags can be edited
		bool editable;
		/// @brief Determines if the tags can be selected
		bool selectable;
		/// @brief The selected tags
		PackedStringArray selected_tags;
		/// @brief The edited tag dictionary
		TagDictionary *tag_dictionary;

		/// @brief bound to signal.
		void _on_button_clicked(const Ref<TreeItem> &p_item, int p_column_id, int p_id, int p_mouse_button_index);

		/// @brief bound to signal.
		void _on_item_selected();

		/// @brief bound to signal.
		void _on_tag_edited();

		/// @brief Renders the dictionary's contents.
		void render_dictionary();

		/// @brief Renders the tree.
		void render_tree();

		/// @brief Sets one tree item check-able.
		void set_tree_item_checkable(const Ref<TreeItem> &p_tree_item, const String &p_current_path, const String &p_key);

		/// @brief Sets one tree item's editable icon.
		void set_tree_item_editable_icon(Ref<TreeItem> &p_tree_item);
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_EDITOR_CONTROLS_TAG_TREE_H
