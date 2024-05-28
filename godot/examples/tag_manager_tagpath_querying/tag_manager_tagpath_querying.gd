@tool

extends Control


@onready var tag_tree: TagTree = %TagTree
@onready var tag_path_selection_container = $InventoryContainer/VBoxContainer/TagPathSelectionContainer


func _get_path(tag: String) -> String:
	var chunks = tag.split(".")
	
	chunks.remove_at(chunks.size() - 1)
	
	return ".".join(PackedStringArray(chunks))


func _get_paths(tag_dictionary: TagDictionary) -> Array[String]:
	var out: Array[String] = []
	
	for tag in tag_dictionary.tags:
		var path = _get_path(tag)
		
		if !out.has(path):
			out.append(path)
	
	out.sort()
	
	return out


func _ready():
	var tag_dictionary = TagDictionary.new()
	
	for child in %ItemList.get_children():
		child.shared_tag_dictionary = tag_dictionary
		child.register_tag()

	tag_tree.hide_root = true
	tag_tree.set_selectable(true)
	tag_tree.set_tag_dictionary(tag_dictionary)
	tag_tree.set_selected_tags(tag_dictionary.tags)
	tag_tree.render_tree()
	tag_tree.item_selected.connect(_on_tag_select)
	
	for path in _get_paths(tag_dictionary):
		var button = Button.new()
		var tags = tag_dictionary.get_tags_from_path(path)

		button.text = "Hide all " + path + " items"
		
		button.pressed.connect(func ():
			for node in TagManager.get_nodes_in_tag_path(self, path):
				node.visible = false

			tag_tree.deselect_many_tags(tags)
		)
		
		tag_path_selection_container.add_child(button)
	

func _on_tag_select() -> void:
	var selected_tags = tag_tree.get_selected_tags()
	
	for child in %ItemList.get_children():
		if child is Control:
			child.visible = TagManager.has_some_tags(selected_tags, child)
