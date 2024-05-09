use godot::engine::{EditorInspectorPlugin, IEditorInspectorPlugin, Tree};
use godot::prelude::*;

use crate::tag_dictionary::TagDictionary;

#[derive(GodotClass)]
#[class(tool, init, base = EditorInspectorPlugin)]
pub struct TagDictionaryEditorInspectorPlugin {
    base: Base<EditorInspectorPlugin>,
    #[var]
    tag_dictionary: Option<Gd<TagDictionary>>,
    #[var]
    tree: Option<Gd<Tree>>,
}

impl TagDictionaryEditorInspectorPlugin {
    fn render(&mut self, dict: &Gd<TagDictionary>, mut tree: Gd<Tree>) {
        tree.clear();

        if let Some(mut root) = tree.create_item() {
            for tag in dict.bind().get_tags().as_slice() {
                if let Some(mut item) = root.create_child() {
                    item.set_text(0, tag.clone());
                }
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
        self.tag_dictionary = _object.try_cast::<TagDictionary>().map_or(None, Some);
        self.tree = Some(Tree::new_alloc());

        self.to_gd().add_custom_control(
            self.tree
                .to_variant()
                .to::<Gd<godot::engine::Control>>(),
        );

        if self.tag_dictionary.as_ref().and(self.tree.as_ref()).is_none() {
            return false;
        }

        let callable = Callable::from_fn("tagdictionary_changed", |_: &[&Variant]| {
            Ok(Variant::nil())
        });

        self.tag_dictionary.as_mut().unwrap().connect(StringName::from("changed"), callable);

        true
    }
}
