extends VBoxContainer

const PATH = "res://"

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var fs = TagDictionaryFs.new()

	fs.scan_fs(PATH)
	
	print("Got ", fs.get_dictionaries().size(), " TagDictionary resources")
	
	for dict in fs.get_dictionaries():
		var tree = TagTree.new()
		
		tree.hide_root = true
		tree.set_tag_dictionary(dict)
		
		add_child(tree)
		
