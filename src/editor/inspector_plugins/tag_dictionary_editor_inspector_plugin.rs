use godot::engine::control::{LayoutPreset, SizeFlags};
use godot::engine::{EditorInspectorPlugin, HBoxContainer, IEditorInspectorPlugin, Label, Tree, TreeItem, VBoxContainer};
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
        if _name == "RefCounted".into() {
            return false;
        }

        let title_label = Label::new_alloc();
        let mut container = VBoxContainer::new_alloc();
        let mut header = HBoxContainer::new_alloc();
        let mut tree = Tree::new_alloc();
        let tag_dictionary = _object
            .try_cast::<TagDictionary>()
            .expect("Failed to cast to TagDictionary");

        header.add_child(title_label.to_variant().to());

        container.add_child(header.to_variant().to());
        container.add_child(tree.to_variant().to());

        let mut tags = tag_dictionary.bind().get_tags();

        tags.sort();

        tree.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        tree.set_column_title(0, "Tag name".into());
        tree.set_custom_minimum_size(Vector2::new(0.0, 200.0));
        tree.set_hide_root(true);
        tree.set_v_size_flags(SizeFlags::EXPAND_FILL);
        
        // done this because of this https://github.com/godot-rust/gdext/issues/156
        let mut root: Gd<TreeItem> = tree.call("create_item".into(), &[]).to();

        for tag in tags.as_slice() {
            let mut item: Gd<TreeItem> = root.call("create_child".into(), &[]).to();

            item.set_text(0, tag.clone());
        }

        self.to_gd()
            .add_custom_control(container.to_variant().to());

        true
    }
}
