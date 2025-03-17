//
// Created by iamoc on 17/03/2025.
//

#include "TagManager.h"
#include <godot_cpp/classes/scene_tree.hpp>

using namespace octod::gameplay::tags;

static TagManager *singleton = nullptr;

TagManager *TagManager::get_singleton()
{
	if (singleton == nullptr)
	{
		singleton = memnew(TagManager);
		singleton->group_name = "octod_gameplay_tags";
	}

	return singleton;
}

bool TagManager::add_tag(Node *p_node, const String &p_tag) const
{
	ERR_FAIL_NULL_V_MSG(p_node, false, "TagManager::add_tag: node is null");
	ERR_FAIL_COND_V_MSG(p_tag.is_empty() || p_tag.similarity(" "), false, "TagManager::add_tag: tag is empty or whitespace");

	if (!p_node->is_in_group(group_name))
	{
		p_node->add_to_group(group_name);
		p_node->set_meta(group_name, PackedStringArray({p_tag}));
		return true;
	}

	const PackedStringArray &tags = get_tags(p_node);

	if (tags.find(p_tag) != -1)
	{
		return false;
	}

	p_node->set_meta(group_name, tags);

	return true;
}

unsigned int TagManager::add_tags(Node *p_node, const PackedStringArray &p_tags) const
{
	ERR_FAIL_NULL_V_MSG(p_node, 0, "TagManager::add_tags: node is null");
	ERR_FAIL_COND_V_MSG(p_tags.size() == 0, 0, "TagManager::add_tags: tags are empty");

	unsigned int added = 0;

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (add_tag(p_node, p_tags[i]))
		{
			added += 1;
		}
	}

	return added;
}

TypedArray<Node> TagManager::get_tagged_nodes() const
{
	return get_tree()->get_nodes_in_group(group_name);
}

PackedStringArray TagManager::get_tags(const Node *p_node) const
{
	ERR_FAIL_NULL_V_MSG(p_node, {}, "TagManager::get_tags: node is null");

	if (!p_node->is_in_group(group_name))
	{
		return {};
	}

	return PackedStringArray(p_node->get_meta(group_name));
}

bool TagManager::has_tag(const Node *p_node, const String &p_tag) const
{
	ERR_FAIL_NULL_V_MSG(p_node, false, "TagManager::has_tag: node is null");
	return get_tags(p_node).find(p_tag) != -1;
}

bool TagManager::has_all_tags(const Node *p_node, const PackedStringArray &p_tags) const
{
	ERR_FAIL_NULL_V_MSG(p_node, false, "TagManager::has_all_tags: node is null");

	const PackedStringArray &own_tags = get_tags(p_node);

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (own_tags.find(p_tags[i]) == -1)
		{
			return false;
		}
	}

	return true;
}

bool TagManager::has_none_tags(const Node *p_node, const PackedStringArray &p_tags) const
{
	const PackedStringArray &own_tags = get_tags(p_node);

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (own_tags.find(p_tags[i]) != -1)
		{
			return false;
		}
	}

	return true;
}

bool TagManager::has_some_tags(const Node *p_node, const PackedStringArray &p_tags) const
{
	const PackedStringArray &own_tags = get_tags(p_node);

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (own_tags.find(p_tags[i]) != -1)
		{
			return true;
		}
	}

	return false;
}

bool TagManager::is_in_path(const Node *p_node, const String &p_path) const
{
	ERR_FAIL_NULL_V_MSG(p_node, false, "TagManager::is_in_path: node is null");

	const PackedStringArray &own_tags = get_tags(p_node);

	for (int i = 0; i < own_tags.size(); i++)
	{
		if (own_tags[i] == p_path)
		{
			return true;
		}
	}

	return false;
}

bool TagManager::remove_tag(Node *p_node, const String &p_tag) const
{
	ERR_FAIL_NULL_V_MSG(p_node, false, "TagManager::remove_tag: node is null");
	ERR_FAIL_COND_V_MSG(p_tag.is_empty(), false, "TagManager::remove_tag: tag is empty");

	PackedStringArray tags = get_tags(p_node);

	if (tags.find(p_tag) != -1)
	{
		return false;
	}

	tags.remove_at(tags.find(p_tag));

	if (tags.size() == 0)
	{
		p_node->remove_from_group(group_name);
		p_node->set_meta(group_name, nullptr);
	}

	return true;
}

unsigned int TagManager::remove_tags(Node *p_node, const PackedStringArray &p_tags) const
{
	ERR_FAIL_NULL_V_MSG(p_node, 0, "TagManager::remove_tags: node is null");
	ERR_FAIL_COND_V_MSG(p_tags.size() == 0, 0, "TagManager::remove_tags: tags are empty");

	unsigned int removed_count = 0;

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (remove_tag(p_node, p_tags[i]))
		{
			removed_count += 1;
		}
	}

	return removed_count;
}

void TagManager::set_tags(Node *p_node, const PackedStringArray &p_tags) const
{
	ERR_FAIL_NULL_MSG(p_node, "TagManager::set_tags: node is null");
	ERR_FAIL_COND_MSG(p_tags.size() == 0, "TagManager::set_tags: tags are empty");

	if (p_node->is_in_group(group_name))
	{
		PackedStringArray own_tags = get_tags(p_node);

		for (int i = 0; i < p_tags.size(); i++)
		{
			if (own_tags.find(p_tags[i]) == -1)
			{
				own_tags.append(p_tags[i]);
			}
		}

		p_node->set_meta(group_name, own_tags);
	} else
	{
		p_node->add_to_group(group_name);
		p_node->set_meta(group_name, p_tags);
	}
}

void TagManager::_bind_methods()
{
	/// binds methods to godot
	ClassDB::bind_method(D_METHOD("add_tag", "node", "tag"), &TagManager::add_tag);
	ClassDB::bind_method(D_METHOD("add_tags", "node", "tags"), &TagManager::add_tags);
	ClassDB::bind_method(D_METHOD("get_tagged_nodes"), &TagManager::get_tagged_nodes);
	ClassDB::bind_method(D_METHOD("get_tags", "node"), &TagManager::get_tags);
	ClassDB::bind_method(D_METHOD("has_tag", "node", "tag"), &TagManager::has_tag);
	ClassDB::bind_method(D_METHOD("has_all_tags", "node", "tags"), &TagManager::has_all_tags);
	ClassDB::bind_method(D_METHOD("has_none_tags", "node", "tags"), &TagManager::has_none_tags);
	ClassDB::bind_method(D_METHOD("is_in_path", "node", "path"), &TagManager::is_in_path);
	ClassDB::bind_method(D_METHOD("remove_tag", "node", "tag"), &TagManager::remove_tag);
	ClassDB::bind_method(D_METHOD("remove_tags", "node", "tags"), &TagManager::remove_tags);
}

const String &TagManager::get_group_name() const
{
	return group_name;
}

void TagManager::set_group_name(const String &p_group_name)
{
	ERR_FAIL_COND_MSG(p_group_name.is_empty() || p_group_name.similarity(" "), "TagManager::set_group_name: group name is empty or whitespace");

	group_name = p_group_name;
}
