use godot::engine::{EditorInspectorPlugin, IEditorInspectorPlugin, Tree};
use godot::prelude::*;

use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct TagDictionaryEditorInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for TagDictionaryEditorInspectorPlugin {
    fn can_handle(&self, object: Gd<Object>) -> bool {
        object.is_class(TagDictionary::class_name().to_gstring())
    }

    fn parse_property(
        &mut self,
        _object: Gd<Object>,
        _type_: VariantType,
        _name: GString,
        _hint_type: godot::engine::global::PropertyHint,
        _hint_string: GString,
        _usage_flags: godot::engine::global::PropertyUsageFlags,
        _wide: bool,
    ) -> bool {
        let mut tree = Tree::new_alloc();
        let tag_dictionary = _object
            .try_cast::<TagDictionary>()
            .expect("Failed to cast to TagDictionary");

        self.to_gd()
            .add_custom_control(tree.to_variant().to::<Gd<godot::engine::Control>>());

        let tags = tag_dictionary.bind().get_tags();

        // let mut root = tree.create_item().expect("Root item not created");

        // for tag in tags.as_slice() {
        //     let mut item = root.create_child().expect("Child item not created");

        //     item.set_text(0, tag.clone());
        // }

        true
    }
}
