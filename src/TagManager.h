//
// Created by iamoc on 17/03/2025.
//

#ifndef OCTOD_GAMEPLAY_TAGS_TAG_MANAGER_H
#define OCTOD_GAMEPLAY_TAGS_TAG_MANAGER_H

#include <godot_cpp/classes/node.hpp>

using namespace godot;

namespace octod::gameplay::tags
{
	/// @brief Handles the tags of the nodes.
	class TagManager final : public Node
	{
		GDCLASS(TagManager, Node)

	public:
		/// @brief Returns the singleton instance of the TagManager.
		static TagManager *get_singleton();

		/// @brief Adds the tag to the node.
		/// @param p_node The node to which the tag will be added.
		/// @param p_tag The tag to add.
		bool add_tag(Node *p_node, const String &p_tag) const;

		/// @brief Adds the tags to the node.
		/// @param p_node The node to which the tags will be added.
		/// @param p_tags The tags to add.
		unsigned int add_tags(Node *p_node, const PackedStringArray &p_tags) const;

		/// @brief Returns all the tagged nodes descending from the node.
		/// @return All the tagged nodes descending from the node.
		[[nodiscard]] TypedArray<Node> get_tagged_nodes() const;

		/// @brief Returns the tags of the node.
		/// @param p_node The node from which the tags will be retrieved.
		/// @return The tags of the node.
		PackedStringArray get_tags(const Node *p_node) const;

		/// @brief Returns whether the node has the tag.
		/// @param p_node The node to check.
		/// @param p_tag The tag to check.
		/// @return Whether the node has the tag.
		bool has_tag(const Node *p_node, const String &p_tag) const;

		/// @brief Returns whether the node has all the tags.
		/// @param p_node The node to check.
		/// @param p_tags The tags to check.
		/// @return Whether the node has all the tags.
		bool has_all_tags(const Node *p_node, const PackedStringArray &p_tags) const;

		/// @brief Returns whether the node has any of the tags.
		/// @param p_node The node to check.
		/// @param p_tags The tags to check.
		/// @return Whether the node has any of the tags.
		bool has_none_tags(const Node *p_node, const PackedStringArray &p_tags) const;

		/// @brief Returns whether the node has some of the tags.
		/// @param p_node The node to check.
		/// @param p_tags The tags to check.
		/// @return Whether the node has some of the tags.
		bool has_some_tags(const Node *p_node, const PackedStringArray &p_tags) const;

		/// @brief Returns whether the node is in the path.
		/// @param p_node The node to check.
		/// @param p_path The path to check.
		/// @return Whether the node is in the path.
		bool is_in_path(const Node *p_node, const String &p_path) const;

		/// @brief Removes the tag from the node.
		/// @param p_node The node from which the tag will be removed.
		/// @param p_tag The tag to remove.
		bool remove_tag(Node *p_node, const String &p_tag) const;

		/// @brief Removes the tags from the node.
		/// @param p_node The node from which the tags will be removed.
		/// @param p_tags The tags to remove.
		unsigned int remove_tags(Node *p_node, const PackedStringArray &p_tags) const;

		/// @brief Sets the tags of the node.
		/// @param p_node The node to which the tags will be set.
		/// @param p_tags The tags to set.
		void set_tags(Node *p_node, const PackedStringArray &p_tags) const;

	protected:
		/// @brief Binds the methods of the TagManager class to the Godot engine.
		static void _bind_methods();

		/// @brief The group name to look for tagged nodes and to retrieve meta.
		String group_name;

		/// @brief Returns the group name.
		[[nodiscard]] const String& get_group_name() const;

		/// @brief Sets the group name.
		void set_group_name(const String &p_group_name);
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_TAG_MANAGER_H
