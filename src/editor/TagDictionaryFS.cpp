//
// Created by Lasagnaking on 19/03/2025.
//

#include "TagDictionaryFS.h"
#include <godot_cpp/classes/dir_access.hpp>
#include <godot_cpp/classes/resource_loader.hpp>

using namespace octod::gameplay::tags::editor;

static TagDictionaryFS *singleton = nullptr;

TagDictionaryFS * TagDictionaryFS::get_singleton()
{
	if (singleton == nullptr)
	{
		singleton = memnew(TagDictionaryFS);
	}

	return singleton;
}

TypedArray<octod::gameplay::tags::TagDictionary> TagDictionaryFS::get_dictionaries() const
{
	return dictionaries;
}

void TagDictionaryFS::scan_file_system(const String &p_from_directory)
{
	PackedStringArray resources = _read_directory_recursive(p_from_directory);
	ResourceLoader *resource_loader = ResourceLoader::get_singleton();

	for (int i = 0; i < resources.size(); i++)
	{
		if (const String& res_path = resources[i]; res_path.ends_with(".res") || res_path.ends_with(".tres"))
		{
			if (Ref<Resource> resource = resource_loader->load(res_path); resource.is_valid())
			{
				if (const TagDictionary *maybe_tag_dictionary = cast_to<TagDictionary>(resource.ptr()); maybe_tag_dictionary != nullptr)
				{
					dictionaries.push_back(maybe_tag_dictionary);
				}
			}
		}
	}
}

void TagDictionaryFS::_bind_methods()
{
}

PackedStringArray TagDictionaryFS::_read_directory_recursive(const String &p_directory) // NOLINT(*-no-recursion)
{
	PackedStringArray output;

	const Ref<DirAccess> dir_access = DirAccess::open(p_directory);

	ERR_FAIL_COND_V_MSG(dir_access->list_dir_begin() != OK, output, "Error while reading directory: " + p_directory);

	String filename = dir_access->get_next();

	while (!filename.is_empty())
	{
		if (dir_access->current_is_dir())
		{
			if (!filename.begins_with("."))
			{
				PackedStringArray sub_dir_files = _read_directory_recursive(vformat("{%s}/{%s}", p_directory, filename));

				for (int i = 0; i < sub_dir_files.size(); i++)
				{
					output.push_back(sub_dir_files[i]);
				}
			}
		} else
		{
			output.push_back(vformat("{%s}/{%s}", p_directory, filename));
		}

		filename = dir_access->get_next();
	}

	return output;
}
