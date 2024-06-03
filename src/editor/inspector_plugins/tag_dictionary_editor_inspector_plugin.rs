use godot::engine::control::{LayoutPreset, SizeFlags};
use godot::engine::{EditorInspectorPlugin, IEditorInspectorPlugin, ResourceSaver};
use godot::prelude::*;

use crate::editor::ui::tag_tree::TagTree;
use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct TagDictionaryEditorInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl TagDictionaryEditorInspectorPlugin {
    #[func]
    pub fn handle_tag_dictionary_changed(tag_dictionary: Gd<TagDictionary>) {
        ResourceSaver::singleton().save(tag_dictionary.to_variant().to());
    }
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
        if _name.to_string().to_lowercase() == "script" {
            return false;
        }

        let mut tag_tree = TagTree::new_alloc();
        let mut tag_dictionary = _object
            .try_cast::<TagDictionary>()
            .expect("Failed to cast to TagDictionary");
        let mut callable_args = VariantArray::new();

        callable_args.push(tag_dictionary.clone().to_variant());

        let callable = Callable::from_object_method(&self.to_gd(), "handle_tag_dictionary_changed")
            .bindv(callable_args);

        if !tag_dictionary.is_connected("changed".into(), callable.clone()) {
            tag_dictionary.connect("changed".into(), callable);
        }

        tag_tree.bind_mut().set_tag_dictionary(Some(tag_dictionary));

        tag_tree.set_hide_root(false);
        tag_tree.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        tag_tree.set_column_title(0, "Tag name".into());
        tag_tree.set_custom_minimum_size(Vector2::new(0.0, 200.0));
        tag_tree.bind_mut().set_editable(true);
        tag_tree.set_v_size_flags(SizeFlags::EXPAND_FILL);

        // done this because of this https://github.com/godot-rust/gdext/issues/156
        self.to_gd().add_custom_control(tag_tree.to_variant().to());

        true
    }
}
