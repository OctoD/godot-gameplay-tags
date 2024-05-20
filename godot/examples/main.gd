extends VBoxContainer


var example_scenes: Array[String] = [
	"res://examples/check_if_node_is_tagged/check_if_node_is_tagged.tscn",
	"res://examples/get_tag_dictionary_tree_dictionary/get_tag_dictionary_tree_dictionary.tscn",
	"res://examples/list_all_tag_dictionaries/list_all_tag_dictionaries.tscn",
	"res://examples/tree_tags_querying/tree_tags_querying.tscn",
]

@onready var back_to_menu_button: Button = %BackToMenuButton
@onready var main_menu = preload("res://examples/main_menu.tscn")
@onready var render_target = %RunningExample


func _clear_scene() -> void:
	for child in render_target.get_children():
		child.queue_free()


func _ready() -> void:
	_render_main_menu()
	back_to_menu_button.pressed.connect(on_back_button_pressed)
	back_to_menu_button.visible = false


func _render_main_menu() -> void:
	var menu = main_menu.instantiate()

	menu.change_scene_request.connect(on_change_scene_requested)
	menu.examples = example_scenes
	
	_clear_scene()
	
	render_target.add_child(menu)


func on_back_button_pressed() -> void:
	_render_main_menu()
	back_to_menu_button.visible = false


func on_change_scene_requested(scene_path: String) -> void:
	print("changing scene to " + scene_path)
	
	var scene = load(scene_path)
	
	assert(scene is PackedScene, "omg I am a banana!")
	
	_clear_scene()
	
	if scene is PackedScene:
		render_target.add_child(scene.instantiate())
		back_to_menu_button.visible = true
