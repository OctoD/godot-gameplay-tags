use godot::prelude::*;
use godot::engine::Tree;

#[derive(GodotClass)]
#[class(init, tool, base = Tree)]
pub struct TagDictionaryEditor {
	base: Base<Tree>,
}

#[godot_api]
impl TagDictionaryEditor {

}
