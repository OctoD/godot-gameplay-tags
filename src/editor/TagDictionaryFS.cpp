//
// Created by Lasagnaking on 19/03/2025.
//

#include "TagDictionaryFS.h"
#include <godot_cpp/classes/dir_access.hpp>

using namespace octod::gameplay::tags::editor;

TypedArray<octod::gameplay::tags::TagDictionary> TagDictionaryFS::get_dictionaries() const
{
}

void TagDictionaryFS::scan_file_system(const String &p_from_directory)
{
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
