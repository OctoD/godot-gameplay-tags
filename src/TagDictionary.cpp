//
// Created by iamoc on 17/03/2025.
//

#include "TagDictionary.h"

using namespace octod::gameplay::tags;

const char * TagDictionary::SPLIT_CHAR = ".";

bool TagDictionary::add_tag(const String &p_tag)
{
	if (has_tag(p_tag))
	{
		return false;
	}

	tags.append(p_tag);
	emit_changed();

	return true;
}

unsigned int TagDictionary::add_tags(const PackedStringArray &p_tags)
{
	unsigned int count = 0;

	for (int i = 0; i < p_tags.size(); i++)
	{
		if (add_tag(p_tags[i]))
		{
			count++;
		}
	}

	if (count > 0)
	{
		emit_changed();
	}

	return count;
}

unsigned int TagDictionary::count() const
{
	return tags.size();
}

PackedStringArray TagDictionary::find(const Callable &p_predicate) const
{
	PackedStringArray result;

	for (int i = 0; i < tags.size(); i++)
	{
		if (p_predicate.call(tags[i]))
		{
			result.append(tags[i]);
		}
	}

	return result;
}

PackedStringArray TagDictionary::get_tags_from_path(const String &p_path) const
{
	PackedStringArray result;

	for (int i = 0; i < tags.size(); i++)
	{
		if (tags[i].begins_with(p_path))
		{
			result.append(tags[i]);
		}
	}

	return result;
}

Dictionary TagDictionary::get_tree() const
{
	Dictionary root;
	PackedStringArray sorted_tags = tags;
	sorted_tags.sort();

	for (int i = 0; i < sorted_tags.size(); i++)
	{
		const String &tag = sorted_tags[i];
		PackedStringArray chunks = tag.split(".");
		Dictionary current = root;

		for (int j = 0; j < chunks.size(); j++)
		{
			const String &chunk = chunks[j];

			if (!current.has(chunk))
			{
				const Dictionary new_node;
				current[chunk] = new_node;
			}

			current = current[chunk];
		}
	}

	return root;
}

bool TagDictionary::has_path(const String &p_path) const
{
	for (int i = 0; i < tags.size(); i++)
	{
		if (tags[i].begins_with(p_path))
		{
			return true;
		}
	}

	return false;
}

bool TagDictionary::has_tag(const String &p_tag) const
{
	return tags.find(p_tag) != -1;
}

bool TagDictionary::has_some_tags(const PackedStringArray &p_tags) const
{
	for (int i = 0; i < p_tags.size(); i++)
	{
		if (tags.find(p_tags[i]) != -1)
		{
			return true;
		}
	}

	return false;
}

bool TagDictionary::has_none_tags(const PackedStringArray &p_tags) const
{
	for (int i = 0; i < p_tags.size(); i++)
	{
		if (tags.find(p_tags[i]) != -1)
		{
			return false;
		}
	}

	return true;
}

bool TagDictionary::none(const Callable &p_predicate) const
{
	for (int i = 0; i < tags.size(); i++)
	{
		if (p_predicate.call(tags[i]))
		{
			return false;
		}
	}

	return true;
}

bool TagDictionary::replace_tag(const String &p_old_tag, const String &p_new_tag)
{
	for (int i = 0; i < tags.size(); i++)
	{
		if (tags[i] == p_old_tag)
		{
			tags[i] = p_new_tag;
			emit_changed();
			return true;
		}
	}

	return false;
}

unsigned int TagDictionary::replace_tags(const PackedStringArray &p_old_tags, const PackedStringArray &p_new_tags)
{
	unsigned int count = 0;

	for (int i = 0; i < p_old_tags.size(); i++)
	{
		if (replace_tag(p_old_tags[i], p_new_tags[i]))
		{
			count++;
		}
	}

	if (count > 0)
	{
		emit_changed();
	}

	return count;
}

bool TagDictionary::remove_tag(const String &p_tag)
{
	const unsigned int tag_index = tags.find(p_tag);

	if (tag_index == -1)
	{
		return false;
	}

	tags.remove_at(tag_index);
	emit_changed();

	return true;
}

unsigned int TagDictionary::remove_path(const String &p_path)
{
	unsigned int removed_tags_count = 0;

	for (int64_t i = tags.size() - 1; i >= 0; i--)
	{
		if (tags[i].begins_with(p_path))
		{
			tags.remove_at(i);
			removed_tags_count++;
		}
	}

	if (removed_tags_count > 0)
	{
		emit_changed();
	}

	return removed_tags_count;
}

bool TagDictionary::some(const Callable &p_predicate) const
{
	for (int64_t i = 0; i < tags.size(); i++)
	{
		if (p_predicate.call(tags[i]))
		{
			return true;
		}
	}

	return false;
}

unsigned int TagDictionary::update_path(const String &p_old_path, const String &p_new_path)
{
	unsigned int updated_count = 0;

	for (int i = 0; i < tags.size(); i++)
	{
		if (tags[i].begins_with(p_old_path))
		{
			tags[i] = p_new_path + tags[i].substr(p_old_path.length());
			updated_count += 1;
		}
	}

	return updated_count;
}

void TagDictionary::_bind_methods()
{
	/// binds methods to godot
	ClassDB::bind_method(D_METHOD("add_tag", "string"), &TagDictionary::add_tag);
	ClassDB::bind_method(D_METHOD("find", "callable"), &TagDictionary::find);
	ClassDB::bind_method(D_METHOD("get_tags_from_path", "string"), &TagDictionary::get_tags_from_path);
	ClassDB::bind_method(D_METHOD("get_tree"), &TagDictionary::get_tree);
	ClassDB::bind_method(D_METHOD("has_path", "string"), &TagDictionary::has_path);
	ClassDB::bind_method(D_METHOD("has_tag", "string"), &TagDictionary::has_tag);
	ClassDB::bind_method(D_METHOD("has_some_tags", "array"), &TagDictionary::has_some_tags);
	ClassDB::bind_method(D_METHOD("has_none_tags", "array"), &TagDictionary::has_none_tags);
	ClassDB::bind_method(D_METHOD("none", "callable"), &TagDictionary::none);
	ClassDB::bind_method(D_METHOD("replace_tag", "string", "string"), &TagDictionary::replace_tag);
	ClassDB::bind_method(D_METHOD("replace_tags", "array", "array"), &TagDictionary::replace_tags);
	ClassDB::bind_method(D_METHOD("remove_tag", "string"), &TagDictionary::remove_tag);
	ClassDB::bind_method(D_METHOD("remove_path", "string"), &TagDictionary::remove_path);
	ClassDB::bind_method(D_METHOD("some", "callable"), &TagDictionary::some);
	ClassDB::bind_method(D_METHOD("update_path", "string", "string"), &TagDictionary::update_path);

	/// binds properties to godot
	ADD_PROPERTY(PropertyInfo(Variant::ARRAY, "tags", PROPERTY_HINT_TYPE_STRING), "add_tag", "remove_tag");
}
