//
// Created by iamoc on 17/03/2025.
//

#ifndef OCTOD_GAMEPLAY_TAGS_REGISTER_TYPES_H
#define OCTOD_GAMEPLAY_TAGS_REGISTER_TYPES_H

#include <godot_cpp/core/class_db.hpp>

using namespace godot;

namespace octod::gameplay::tags
{
	/// @brief Registers the types.
	void register_types(ModuleInitializationLevel p_level);

	/// @brief Unregisters the types.
	void unregister_types(ModuleInitializationLevel p_level);
}

#endif //OCTOD_GAMEPLAY_TAGS_REGISTER_TYPES_H
