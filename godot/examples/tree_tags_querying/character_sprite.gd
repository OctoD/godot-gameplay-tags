extends CharacterBody2D

@export var character_name: String


func _ready() -> void:
	$Label.text = character_name
