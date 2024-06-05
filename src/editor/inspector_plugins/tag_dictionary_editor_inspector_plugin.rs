use godot::engine::box_container::AlignmentMode;
use godot::engine::control::{LayoutPreset, SizeFlags};
use godot::engine::file_access::ModeFlags;
use godot::engine::{
    Button, EditorInspectorPlugin, FileAccess, HBoxContainer, IEditorInspectorPlugin,
    ResourceSaver, VBoxContainer,
};
use godot::prelude::*;

use crate::editor::ui::tag_tree::TagTree;
use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct TagDictionaryEditorInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
    tag_dictionary: Option<Gd<TagDictionary>>,
}

#[godot_api]
impl TagDictionaryEditorInspectorPlugin {
    #[func]
    pub fn handle_tag_dictionary_changed(tag_dictionary: Gd<TagDictionary>) {
        ResourceSaver::singleton().save(tag_dictionary.to_variant().to());
    }

    #[func]
    pub fn handle_tag_dictionary_export(&self) {
        if let Some(td) = self.tag_dictionary.clone() {
            let tags = td.bind().get_tags();
            let mut output = String::from("");

            for tag in tags.as_slice() {
                output.push_str(&format!("{}\n", tag));
            }

            if let Some(mut f) = FileAccess::open(
                td.get_path().to_string().replace("tres", "csv").into(),
                ModeFlags::READ_WRITE,
            ) {
                f.store_string(GString::from(output));
                f.close();
            }
        }
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

        let mut export_button = Button::new_alloc();

        export_button.connect(
            "pressed".into(),
            Callable::from_object_method(&self.to_gd(), "handle_tag_dictionary_export"),
        );
        export_button.set_text("Export to CVS".into());

        let mut footer_container = HBoxContainer::new_alloc();

        footer_container.add_child(export_button.to_variant().to());
        footer_container.set_alignment(AlignmentMode::END);

        let mut tag_tree = TagTree::new_alloc();
        let mut tag_dictionary = _object
            .try_cast::<TagDictionary>()
            .expect("Failed to cast to TagDictionary");
        let mut callable_args = VariantArray::new();

        self.tag_dictionary = Some(tag_dictionary.clone());

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
        let mut vbox_container = VBoxContainer::new_alloc();

        vbox_container.add_child(tag_tree.to_variant().to());
        vbox_container.add_spacer(false);
        vbox_container.add_child(footer_container.to_variant().to());
        vbox_container.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);

        self.to_gd()
            .add_custom_control(vbox_container.to_variant().to());

        true
    }
}
