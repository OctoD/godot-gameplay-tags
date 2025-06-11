
project(godot-gameplay-tags)

set(godot-gameplay-tags_version 1.0.0)
set(godot-gameplay-tags_output_directory "${CMAKE_SOURCE_DIR}/demo/addons/godot-gameplay-tags/bin/${PLATFORM}")
set(godot-gameplay-tags_library_name "${PROJECT_NAME}.${PLATFORM}.template_${DEBUG_RELEASE}.${ARCH}.${EXT}")

# Automatically pick up files added to src
file(GLOB_RECURSE SOURCES CONFIGURE_DEPENDS
        "${CMAKE_CURRENT_SOURCE_DIR}/src/*.h"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/*.hpp"
        "${CMAKE_CURRENT_SOURCE_DIR}/src/*.cpp")

# Add a dynamic library called gameplay - this builds gameplay.dll
add_library(${PROJECT_NAME} SHARED ${SOURCES}
        register_types.cpp
        register_types.h
        TagManager.cpp
        TagManager.h
        TagDictionary.cpp
        TagDictionary.h
        editor/TagDictionaryFS.cpp
        editor/TagDictionaryFS.h
        editor/docks/NodeTaggingDock.cpp
        editor/docks/NodeTaggingDock.h
        editor/controls/TagTree.cpp
        editor/controls/TagTree.h
        editor/inspector_plugins/TagDictionaryEditorInspectorPlugin.cpp
        editor/inspector_plugins/TagDictionaryEditorInspectorPlugin.h
)

target_include_directories(${PROJECT_NAME} PRIVATE "src")
target_link_libraries(${PROJECT_NAME} PUBLIC godot::cpp)

source_group(TREE "${CMAKE_CURRENT_SOURCE_DIR}/src" PREFIX src FILES ${SOURCES})

# Copy the library to the demo addons directory after building it ${demo-gameplay-tags_output_directory}
# The library will be copied to the demo addons directory after building it

message(STATUS "Copying ${PROJECT_NAME} to ${godot-gameplay-tags_output_directory}/${LIBRARY_NAME}")

add_custom_command(TARGET ${PROJECT_NAME} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        $<TARGET_FILE:${PROJECT_NAME}>
        "${godot-gameplay-tags_output_directory}/${godot-gameplay-tags_library_name}")