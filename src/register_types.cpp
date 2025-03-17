//
// Created by iamoc on 17/03/2025.
//

#include "register_types.h"

#include "TagManager.h"
#include <godot_cpp/classes/engine.hpp>

using namespace godot;

namespace octod::gameplay::tags
{
	void register_types(ModuleInitializationLevel p_level)
	{
		if (p_level == MODULE_INITIALIZATION_LEVEL_SCENE)
		{
			/// registers concrete classes
			ClassDB::register_class<TagManager>();

			/// registers singletons
			Engine::get_singleton()->register_singleton(TagManager::get_class_static(), TagManager::get_singleton());
		}
	}

	void unregister_types(ModuleInitializationLevel p_level)
	{
		if (p_level == MODULE_INITIALIZATION_LEVEL_SCENE)
		{
			/// unregisters singletons and destroy them
			if (Engine::get_singleton()->has_singleton(TagManager::get_class_static()))
			{
				Engine::get_singleton()->unregister_singleton(TagManager::get_class_static());
				memdelete(TagManager::get_singleton());
			}
		}
	}
}

extern "C"{
GDExtensionBool GDE_EXPORT godot_gameplay_tags_init(GDExtensionInterfaceGetProcAddress p_get_proc_address, GDExtensionClassLibraryPtr p_library, GDExtensionInitialization *r_initialization)
{
	GDExtensionBinding::InitObject init_obj(p_get_proc_address, p_library, r_initialization);

	init_obj.register_initializer(octod::gameplay::tags::register_types);
	init_obj.register_terminator(octod::gameplay::tags::unregister_types);
	init_obj.set_minimum_library_initialization_level(MODULE_INITIALIZATION_LEVEL_SCENE);

	return init_obj.init();
}
}
