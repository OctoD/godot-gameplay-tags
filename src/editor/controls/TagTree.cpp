//
// Created by iamoc on 20/03/2025.
//

#include "TagTree.h"

#include "../../TagDictionary.h"
#include <godot_cpp/classes/display_server.hpp>

using namespace octod::gameplay::tags;
using namespace octod::gameplay::tags::controls::TagTree;

static const char *TAG_PATH_META = "ggt_tag_dictionary_path";

void TagTree::_ready()
{
	Tree::_ready();

	connect("button_clicked", Callable::create(this, "_on_button_clicked"));
	connect("item_selected", Callable::create(this, "_on_item_selected"));
	connect("item_edited", Callable::create(this, "_on_tag_edited"));

	set_v_scroll_enabled(true);
	set_v_size_flags(SIZE_EXPAND_FILL);

	render_tree();
}

void TagTree::deselect_many_tags(const PackedStringArray &p_tags)
{
	for (int i = 0; i < p_tags.size(); i++)
	{
		if (const int64_t index = selected_tags.find(p_tags[i]); index >= 0)
		{
			selected_tags.remove_at(index);
		}
	}
}

void TagTree::deselect_tag(const String &p_tag)
{
	if (const int64_t index = selected_tags.find(p_tag); index >= 0)
	{
		selected_tags.remove_at(index);
	}
}

bool TagTree::get_editable() const
{
	return editable;
}

bool TagTree::get_selectable() const
{
	return selectable;
}

PackedStringArray TagTree::get_selected_tags() const
{
	return selected_tags;
}

TagDictionary *TagTree::get_tag_dictionary() const
{
	return tag_dictionary;
}

bool TagTree::is_path_selected(const String &p_tag_path) const
{
	for (int i = 0; i < selected_tags.size(); i++)
	{
		if (selected_tags[i].begins_with(p_tag_path))
		{
			return true;
		}
	}

	return false;
}

void TagTree::select_many_tags(const PackedStringArray &p_tags)
{
	for (int i = 0; i < p_tags.size(); i++)
	{
		if (const int64_t index = selected_tags.find(p_tags[i]); index < 0)
		{
			selected_tags.push_back(p_tags[i]);
		}
	}
}

void TagTree::select_tag(const String &p_tag)
{
	if (const int64_t index = selected_tags.find(p_tag); index < 0)
	{
		selected_tags.push_back(p_tag);
	}
}

void TagTree::set_editable(bool p_editable)
{
	editable = p_editable;
}

void TagTree::set_selectable(bool p_selectable)
{
	selectable = p_selectable;
}

void TagTree::set_tag_dictionary(TagDictionary *p_tag_dictionary)
{
	tag_dictionary = p_tag_dictionary;
}

void TagTree::_on_button_clicked(const Ref<TreeItem> &p_item, int p_column_id, int p_id, int p_mouse_button_index)
{
	const String &tag_path = get_meta(TAG_PATH_META, "");

	if (p_id == 1)
	{
		tag_dictionary->remove_path(tag_path);
		emit_signal("tag_path_removed", tag_path);
		render_tree();
	} else
	{
		const int child_count = get_child_count();
		const String &new_tag_path = "new_tag_" + itos(child_count);

		String new_tag_name;

		if (tag_path.is_empty())
		{
			new_tag_name = new_tag_path;
		} else
		{
			new_tag_name = tag_path + String(TagDictionary::SPLIT_CHAR) + new_tag_path;
		}

		if (tag_dictionary->add_tag(new_tag_name))
		{
			render_tree();
		}
	}
}

void TagTree::_on_item_selected()
{
	TreeItem *maybe_selected = get_selected();

	if (!maybe_selected || !tag_dictionary)
	{
		return;
	}

	const String &tag_path = maybe_selected->get_meta(TAG_PATH_META, "");
	DisplayServer::get_singleton()->clipboard_set(tag_path);

	if (selectable)
	{
		PackedStringArray added_tags;
		PackedStringArray found_tags = tag_dictionary->get_tags_from_path(tag_path);
		PackedStringArray removed_tags;

		bool is_checked = maybe_selected->is_checked(0);

		for (int64_t i = 0; i < found_tags.size(); i++)
		{
			if (is_checked)
			{
				if (const int64_t index = selected_tags.find(found_tags[i]); index >= 0)
				{
					selected_tags.remove_at(index);
					removed_tags.push_back(found_tags[i]);
				}
			} else
			{
				if (const int64_t index = selected_tags.find(found_tags[i]); index >= 0)
				{
					added_tags.push_back(found_tags[i]);
					selected_tags.push_back(found_tags[i]);
				}
			}
		}

		if (added_tags.size() > 0)
		{
			emit_signal("tag_added", added_tags);
		}

		if (removed_tags.size() > 0)
		{
			emit_signal("tag_removed", removed_tags);
		}
	}
}

void TagTree::_on_tag_edited()
{
}

void TagTree::render_dictionary()
{
}

void TagTree::render_tree()
{
}

void TagTree::set_tree_item_checkable(const Ref<TreeItem> &p_tree_item, const String &p_current_path, const String &p_key)
{
}

void TagTree::set_tree_item_editable_icon(Ref<TreeItem> &p_tree_item)
{
}

void TagTree::_bind_methods()
{
	/// methods binding
	ClassDB::bind_method(D_METHOD("deselect_many_tags", "tags"), &TagTree::deselect_many_tags);
	ClassDB::bind_method(D_METHOD("deselect_tag", "tag"), &TagTree::deselect_tag);
	ClassDB::bind_method(D_METHOD("get_editable"), &TagTree::get_editable);
	ClassDB::bind_method(D_METHOD("get_selectable"), &TagTree::get_selectable);
	ClassDB::bind_method(D_METHOD("get_selected_tags"), &TagTree::get_selected_tags);
	ClassDB::bind_method(D_METHOD("get_tag_dictionary"), &TagTree::get_tag_dictionary);
	ClassDB::bind_method(D_METHOD("is_path_selected", "tag_path"), &TagTree::is_path_selected);
	ClassDB::bind_method(D_METHOD("select_many_tags", "tags"), &TagTree::select_many_tags);
	ClassDB::bind_method(D_METHOD("select_tag", "tag"), &TagTree::select_tag);
	ClassDB::bind_method(D_METHOD("set_editable", "editable"), &TagTree::set_editable);
	ClassDB::bind_method(D_METHOD("set_selectable", "selectable"), &TagTree::set_selectable);
	ClassDB::bind_method(D_METHOD("set_tag_dictionary", "tag_dictionary"), &TagTree::set_tag_dictionary);
	ClassDB::bind_method(D_METHOD("_on_button_clicked", "item", "column_id", "id", "mouse_button_index"), &TagTree::_on_button_clicked);
	ClassDB::bind_method(D_METHOD("_on_item_selected"), &TagTree::_on_item_selected);
	ClassDB::bind_method(D_METHOD("_on_tag_edited"), &TagTree::_on_tag_edited);
	ClassDB::bind_method(D_METHOD("render_dictionary"), &TagTree::render_dictionary);
	ClassDB::bind_method(D_METHOD("render_tree"), &TagTree::render_tree);
	ClassDB::bind_method(D_METHOD("set_tree_item_checkable", "tree_item", "current_path", "key"), &TagTree::set_tree_item_checkable);
	ClassDB::bind_method(D_METHOD("set_tree_item_editable_icon", "tree_item"), &TagTree::set_tree_item_editable_icon);

	/// properties binding
	ADD_PROPERTY(PropertyInfo(Variant::BOOL, "editable"), "set_editable", "get_editable");
	ADD_PROPERTY(PropertyInfo(Variant::BOOL, "selectable"), "set_selectable", "get_selectable");
	ADD_PROPERTY(PropertyInfo(Variant::OBJECT, "tag_dictionary", PROPERTY_HINT_RESOURCE_TYPE, "24:17/TagDictionary"), "set_tag_dictionary", "get_tag_dictionary");

	/// signals binding
	ADD_SIGNAL(MethodInfo("tag_path_edited", PropertyInfo(Variant::STRING, "tag_path"), PropertyInfo(Variant::STRING, "new_tag_path")));
	ADD_SIGNAL(MethodInfo("tag_path_removed", PropertyInfo(Variant::STRING, "tag_path")));
	ADD_SIGNAL(MethodInfo("tags_added", PropertyInfo(Variant::ARRAY, "tags", PROPERTY_HINT_TYPE_STRING)));
	ADD_SIGNAL(MethodInfo("tags_removed", PropertyInfo(Variant::ARRAY, "tags", PROPERTY_HINT_TYPE_STRING)));
}
