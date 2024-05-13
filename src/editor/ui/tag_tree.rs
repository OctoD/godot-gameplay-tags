use godot::{
    engine::{Tree, TreeItem},
    prelude::*,
};

use crate::tag_dictionary::{TagDictionary, SPLIT_CHAR};

#[derive(GodotClass)]
#[class(tool, init, base = Tree)]
pub struct TagTree {
    base: Base<Tree>,
    tag_dictionary: Option<Gd<TagDictionary>>,
}

#[godot_api]
impl TagTree {
    #[func]
    pub fn get_tag_dictionary(&self) -> Option<Gd<TagDictionary>> {
        self.tag_dictionary.clone()
    }

    #[func]
    pub fn set_tag_dictionary(&mut self, tag_dictionary: Option<Gd<TagDictionary>>) {
        self.tag_dictionary = tag_dictionary;
        self.render_tree();
    }

    pub fn render_dictionary(
        &self,
        dictionary: Dictionary,
        mut parent: Gd<TreeItem>,
        path: GString,
    ) {
        let keys = dictionary.keys_array();

        for key in keys.iter_shared() {
            let keystring = key.to_string();

            if let Some(variant) = dictionary.get(keystring.clone()) {
                match Dictionary::try_from_variant(&variant) {
                    Ok(dict) => {
                        let new_path = if path.is_empty() {
                            keystring.clone()
                        } else {
                            format!("{}{}{}", path, SPLIT_CHAR, keystring.clone())
                        };
						godot::engine::utilities::print(new_path.to_variant(), &[]);
                        let mut item = parent.call("create_child".into(), &[]).to::<Gd<TreeItem>>();
						item.set_text(0, keystring.clone().to_godot());
                        self.render_dictionary(dict, item, new_path.clone().into());
                    }
                    Err(convert_error) => {
                        godot::engine::utilities::printerr(
                            convert_error.to_string().to_variant(),
                            &[],
                        );
                    }
                }
            }
        }
    }

    pub fn render_tree(&self) {
        let root = self
            .to_gd()
            .call("create_item".into(), &[])
            .to::<Gd<TreeItem>>();

        if let Some(tag_dictionary) = self.get_tag_dictionary() {
            self.render_dictionary(tag_dictionary.bind().get_tree(), root, "".into())
        }
    }
}
