extends VBoxContainer

const PATH = "res://examples/list_all_tag_dictionaries/"

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var fs = TagDictionaryFs.new()
	var index = 0
	
	fs.scan_fs(PATH)
	
	print("Got ", fs.get_dictionaries().size(), " TagDictionary resources")
	
	for dict in fs.get_dictionaries():
		var list_item = make_list_item(dict)
		
		if list_item:
			if index > 0:
				list_item.add_spacer(true)
			
			add_child(list_item)
			
			index = index + 1


func make_list_item(tag_dictionary: TagDictionary) -> VBoxContainer:
	var container = VBoxContainer.new()
	
	if tag_dictionary is TagDictionary:
		var resource_path_label = Label.new()
		var content_label = Label.new()
		
		resource_path_label.text = tag_dictionary.resource_path
		content_label.text = str(tag_dictionary.tags)
		
		container.add_child(resource_path_label)
		container.add_child(content_label)
	
	return container
