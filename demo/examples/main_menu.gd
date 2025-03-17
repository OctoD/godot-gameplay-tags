class_name MainMenu extends VBoxContainer

signal change_scene_request(resource_path: StringName)

var examples: Array[String] = []

@onready var buttons_container: VBoxContainer = %ButtonsContainer


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	%QuitToDesktopButton.pressed.connect(on_quit_to_desktop_button_pressed)
	
	for packed_scene_file_path in examples:
		var button = Button.new()

		button.pressed.connect(func (): 
			change_scene_request.emit(packed_scene_file_path)
		)
		button.text = packed_scene_file_path.get_file().replace(packed_scene_file_path.get_extension(), "").replace("_", " ")

		buttons_container.add_child(button)


func on_quit_to_desktop_button_pressed() -> void:
	get_tree().quit(0)
