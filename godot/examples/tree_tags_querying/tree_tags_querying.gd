extends Node2D

const resource = preload("res://examples/tree_tags_querying/tree_tags_querying.tres")

@onready var h_box_container: HBoxContainer = $HBoxContainer

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var reset_button = Button.new()
	reset_button.text = "Reset"
	reset_button.pressed.connect(_on_reset_pressed)
	h_box_container.add_child(reset_button)
	
	for child in get_children():
		if child is CharacterBody2D:
			child.add_to_group("octod_ggs_tagged_node_group")
	
	for tag in resource.tags:
		var button = Button.new()
		button.pressed.connect(_on_button_clicked.bind(tag))
		button.text = tag
		h_box_container.add_child(button)


func _on_button_clicked(tag: String) -> void:
	for node in TagManager.get_tagged_nodes(self):
		(node as Node2D).visible = TagManager.has_tag(tag, node)


func _on_reset_pressed() -> void:
	for node in TagManager.get_tagged_nodes(self):
		(node as Node2D).visible = true
