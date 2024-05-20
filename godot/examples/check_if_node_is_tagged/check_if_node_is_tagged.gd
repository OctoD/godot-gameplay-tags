extends Node


func _ready():
	check()
	TagManager.add_tag("lorem", self)
	check()
	TagManager.add_tag("lorem", self)
	check()
	TagManager.add_tag("ipsum", self)
	check()


func check() -> void:
	print("has_none_tags -> ", TagManager.has_none_tags(PackedStringArray(["lorem", "ipsum"]), self), " tags are ", TagManager.get_tags(self))
	print("has_some_tags -> ", TagManager.has_some_tags(PackedStringArray(["lorem", "ipsum"]), self), " tags are ", TagManager.get_tags(self))
	print("has_all_tags  -> ", TagManager.has_all_tags(PackedStringArray(["lorem", "ipsum"]), self), " tags are ", TagManager.get_tags(self))
