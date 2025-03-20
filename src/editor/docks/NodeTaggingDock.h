//
// Created by iamoc on 20/03/2025.
//

#ifndef OCTOD_GAMEPLAY_TAGS_EDITOR_DOCKS_NODE_TAGGING_DOCK_H
#define OCTOD_GAMEPLAY_TAGS_EDITOR_DOCKS_NODE_TAGGING_DOCK_H

#include <godot_cpp/classes/v_box_container.hpp>

using namespace godot;

namespace octod::gameplay::tags::editor::docks
{
	class NodeTaggingDock : public VBoxContainer
	{
		GDCLASS(NodeTaggingDock, VBoxContainer)

	public:
		/// @brief Called when the node enters the scene tree for the first time.
		void _ready() override;

	protected:
		/// @brief Binds methods to Godot.
		static void _bind_methods();

		/// @brief Called when the tags are added to the node.
		static void on_tags_added(const PackedStringArray &p_tags_added);

		/// @brief Called when the tags are removed from the node.
		static void on_tags_removed(const PackedStringArray &p_tags_removed);

		/// @brief Called when the visibility of the node changes.
		void on_visibility_changed() const;

		/// @brief Called when the node is selected.
		void render_tag_trees();
	};
}

#endif //OCTOD_GAMEPLAY_TAGS_EDITOR_DOCKS_NODE_TAGGING_DOCK_H
