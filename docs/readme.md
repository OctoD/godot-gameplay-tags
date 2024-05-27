Godot Gameplay Tags
===================

This addon is a simple way to add tags to your nodes and query them later.

## How does it work

This addon handles the `meta` property gracefully to manage tags in your nodes. This way, you can assign tags to your nodes and query them later. 

Each tagged node is added to a specific group, so you can use the Godot's built-in group system to query the nodes (but this is facilitated by the `TagManager` singleton).

## How to use this addon

You can use two different approaches to handle tags to your nodes:

- Create one or more TagDictionary resources and assign the tags to your nodes.
- Handle the tags directly to your nodes with scripts.
- Do both.

## Creating a TagDictionary

To create a TagDictionary, you have to create a new resource in the Godot editor. You can do this by right-clicking on the FileSystem dock and selecting `New Resource` -> `TagDictionary`.

Once created, you can add tags to the dictionary by clicking on the `+` (Add Tag) button in the `TagDictionary` category.

To assign one or more tags, select one or more nodes and in the inspector click the `Node tags` tab, then select all the tags you want to assign to the nodes.

## Handling tags with scripts

To add, update, remove or check if a node is tagged, you can use the `TagManager` singleton.

This singleton, has several methods to help you manage the tags of your nodes:

| Method | Description |
|--------|-------------|
| `add_tag(node: Node, tag: StringName) -> void` | Adds a single tag to a node. |
| `add_tags(node: Node, tags: PackedStringArray) -> void` | Adds multiple tags to a node. |
| `get_tagged_nodes(target: Node) -> Array[Node]` | Gets all the nodes that have tags assigned under the target node. |
| `get_tags(target: Node) -> PackedStringArray` | Gets all the tags assigned to a node. |
| `has_all_tags(tags: PackedStringArray, target: Node) -> bool` | Checks if a node has all the tags assigned. |
| `has_some_tags(tags: PackedStringArray, target: Node) -> bool` | Checks if a node has at least one of the tags assigned. |
| `has_none_tags(tags: PackedStringArray, target: Node) -> bool` | Checks if a node has none of the tags assigned. |
| `remove_tag(tag: GString, target: Node)` | Removes a single tag from a node. |
| `remove_tags(tags: PackedStringArray, target: Node)` | Removes multiple tags from a node. |
| `set_tags(tags: PackedStringArray, target: Node)` | Sets the tags of a node, removing the previous ones. |
