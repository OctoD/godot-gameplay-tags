### HUGE DISCLAIMER
### DO NOT EVER DO SOMETHING LIKE THIS
### This code is just for the sake of the example, doing this on a real game will hurt your feelings and will make your cat to cry

@tool

extends TextureRect


@export var shared_tag_dictionary: TagDictionary


var armor: Array[String] = [
	"helm",
	"ring",
	"boots",
	"chestplate",
	"amulet",
	"shield"
]


var weapon: Array[String] = [
	"axe",
	"knife",
	"sword",
	"wand",
]

var item_prefix: Array[String] = [
	"shiny",
	"rusty",
	"broken",
	"enchanted",
	"mysterious",
	"ancient",
	"heavy",
	"light",
	"sharp",
	"blunt",
	"magical",
	"normal",
	"common",
	"uncommon",
	"rare",
	"epic",
	"legendary",
	"mythical",
	"godly",
	"divine",
	"unholy",
	"evil",
	"good",
	"neutral",
	"chaotic",
	"lawful",
	"fire",
	"water",
	"earth",
	"air",
	"lightning",
	"ice",
	"poison",
	"acid",
	"dark",
	"light",
	"void",
	"spirit",
	"mind",
	"body",
	"heart",
	"soul",
	"blood",
	"bone",
	"metal",
	"wood",
	"stone",
	"glass",
	"cloth",
	"leather",
	"chain",
	"plate",
	"scale",
	"feather",
	"fur",
	"hair",
	"claw",
	"fang",
	"horn",
	"tusk",
	"eye",
	"tooth",
	"tongue",
	"tail",
	"wing",
	"hand",
	"foot",
	"head",
	"neck",
	"shoulder",
	"chest",
	"back",
	"arm",
	"leg",
	"waist",
	"hip",
	"thigh",
	"calf",
	"ankle",
	"elbow",
	"wrist",
	"finger",
	"toe",
	"ear",
	"nose",
	"mouth",
	"beard",
	"hair",
	"scar",
	"tattoo",
	"mark",
	"symbol",
	"rune",
	"sigil",
	"seal",
	"charm",
	"amulet",
	"pendant",
	"necklace",
	"ring",
	"bracelet",
	"anklet",
	"belt",
	"bag",
	"pouch",
	"backpack",
	"scroll",
	"book",
	"tome",
	"grimoire",
	"codex",
	"tablet"
]

var item_suffix: Array[String] = [
	"of power",
	"of might",
	"of strength",
	"of endurance",
	"of agility",
	"of dexterity",
	"of speed",
	"of haste",
	"of quickness",
	"of swiftness",
	"of accuracy",
	"of precision",
	"of skill",
	"of mastery",
	"of expertise",
	"of proficiency",
	"of knowledge",
	"of wisdom",
	"of intelligence",
	"of insight",
	"of perception",
	"of awareness",
	"of consciousness",
	"of enlightenment",
	"of clarity",
	"of focus",
	"of concentration",
	"of meditation",
	"of contemplation",
	"of reflection",
	"of thought",
	"of memory",
	"of recall",
	"of recollection",
	"of remembrance",
	"of forgetfulness",
	"of oblivion",
	"of amnesia",
	"of dementia",
	"of madness",
	"of insanity",
	"of lunacy",
	"of delirium",
	"of confusion",
	"of chaos",
	"of disorder",
	"of entropy",
	"of decay",
	"of corruption",
	"of pollution",
	"of contamination",
	"of infection",
	"of disease",
	"of plague",
	"of pestilence",
	"of famine",
	"of drought",
	"of flood",
	"of storm",
	"of hurricane",
	"of tornado",
	"of earthquake",
	"of volcano",
	"of eruption",
	"of explosion",
	"of implosion",
	"of collapse",
	"of ruin",
	"of destruction",
	"of annihilation",
	"of obliteration",
	"of extermination",
]

var item_name = ""
var tag = ""


func _ready():
	var prefix = _get_item_prefix()
	var itemtype = _get_item_type()
	var suffix = _get_item_suffix()
	
	item_name = prefix +" "+ itemtype +" "+ suffix

	%Label.text = item_name
	self.tooltip_text = item_name

	tag = "item"

	if armor.has(itemtype):
		tag += "." + "armor"
	else:
		tag += "." + "weapon"

	tag += "." + itemtype

	TagManager.remove_tags(TagManager.get_tags(self), self)
	TagManager.add_tag(tag, self)
	print("setting item ", item_name, " tag to ", tag)
	register_tag()


func _get_item_prefix() -> String:
	return  item_prefix[randi_range(0, item_prefix.size() - 1)]


func _get_item_type() -> String:
	var types = armor.duplicate()

	types.append_array(weapon)

	return types[randi_range(0, types.size() - 1)]


func _get_item_suffix() -> String:
	return item_suffix[randi_range(0, item_suffix.size() - 1)]


func register_tag() -> void:
	if shared_tag_dictionary and tag:
		shared_tag_dictionary.add_tag(tag)
