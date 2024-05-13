use godot::engine::control::{LayoutPreset, SizeFlags};
use godot::engine::{
    EditorInspectorPlugin, HBoxContainer, IEditorInspectorPlugin, Label, VBoxContainer,
};
use godot::prelude::*;

use crate::editor::ui::tag_tree::TagTree;
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
        godot::engine::utilities::print(_name.to_variant(), &[]);

        if _name.to_string().to_lowercase() == "script" {
            return false;
        }

        let mut title_label = Label::new_alloc();
        let mut container = VBoxContainer::new_alloc();
        let mut header = HBoxContainer::new_alloc();
        let mut tag_tree = TagTree::new_alloc();
        let tag_dictionary = _object
            .try_cast::<TagDictionary>()
            .expect("Failed to cast to TagDictionary");

        title_label.set_text(tag_dictionary.get_name().into());

        header.add_child(title_label.to_variant().to());
        header.set_h_size_flags(SizeFlags::EXPAND_FILL);
        header.set_anchors_preset(LayoutPreset::TOP_WIDE);

        container.add_child(header.to_variant().to());
        container.add_child(tag_tree.to_variant().to());

        tag_tree.bind_mut().set_tag_dictionary(Some(tag_dictionary));

        tag_tree.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        tag_tree.set_column_title(0, "Tag name".into());
        tag_tree.set_custom_minimum_size(Vector2::new(0.0, 200.0));
        tag_tree.set_hide_root(true);
        tag_tree.set_v_size_flags(SizeFlags::EXPAND_FILL);

        // done this because of this https://github.com/godot-rust/gdext/issues/156
        self.to_gd().add_custom_control(container.to_variant().to());

        true
    }
}
