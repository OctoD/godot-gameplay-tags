extends Node


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var dict = TagDictionary.new()
	
	dict.add_tag("items.armor.shield")
	dict.add_tag("items.weapons.1h.sword")
	dict.add_tag("items.weapons.1h.wand")
	dict.add_tag("items.armor.glove")
	dict.add_tag("items.weapons.2h.sword")
	dict.add_tag("items.weapons.2h.axe")
	dict.add_tag("items.armor.helm")

	for tag in dict.tags:
		print(tag)

	print_tag_tree(dict.get_tree())


func print_tag_tree(tree: Dictionary, indent_count = 0) -> void:
	for key in tree.keys():
		var indent = ""
		while indent.length() < indent_count:
			indent = indent + "-"
		print(indent + key)
		
		if tree.has(key):
			print_tag_tree(tree[key], indent_count + 1)
