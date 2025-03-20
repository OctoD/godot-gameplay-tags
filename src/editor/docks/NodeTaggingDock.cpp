//
// Created by iamoc on 20/03/2025.
//

#include "NodeTaggingDock.h"

#include <godot_cpp/classes/editor_interface.hpp>
#include <godot_cpp/classes/editor_selection.hpp>

#include "../TagDictionaryFS.h"
#include "../../TagManager.h"


using namespace octod::gameplay::tags::editor::docks;

void NodeTaggingDock::_ready()
{
	VBoxContainer::_ready();
}

void NodeTaggingDock::_bind_methods()
{
}

void NodeTaggingDock::on_tags_added(const PackedStringArray &p_tags_added)
{
	const TagManager *tag_manager = TagManager::get_singleton();

	if (EditorSelection *editor_selection = EditorInterface::get_singleton()->get_selection())
	{
		const TypedArray<Node> selected_nodes = editor_selection->get_selected_nodes();

		for (int i = 0; i < selected_nodes.size(); i++)
		{
			tag_manager->add_tags(cast_to<Node>(selected_nodes[i]), p_tags_added);
		}
	}
}

void NodeTaggingDock::on_tags_removed(const PackedStringArray &p_tags_removed)
{
	const TagManager *tag_manager = TagManager::get_singleton();

	if (EditorSelection *editor_selection = EditorInterface::get_singleton()->get_selection())
	{
		const TypedArray<Node> selected_nodes = editor_selection->get_selected_nodes();

		for (int i = 0; i < selected_nodes.size(); i++)
		{
			tag_manager->remove_tags(cast_to<Node>(selected_nodes[i]), p_tags_removed);
		}
	}
}

void NodeTaggingDock::on_visibility_changed() const
{
	if (!is_visible())
	{
		return;
	}

	for (int i = 0; i < get_child_count(); i++)
	{
		const Ref child = get_child(i);
		child->queue_free();
	}
}

void NodeTaggingDock::render_tag_trees()
{
	for (int i = 0; i < get_child_count(); i++)
	{
		cast_to<Node>(get_child(i))->queue_free();
	}

	TagDictionaryFS::get_singleton()->scan_file_system("res://");

	const auto dictionaries = TagDictionaryFS::get_singleton()->get_dictionaries();

	for (int i = 0; i < dictionaries.size(); i++)
	{
		/// todo: port tag tree node
	}
}
