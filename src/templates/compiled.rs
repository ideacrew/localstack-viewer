use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::http::ContentType;
use tera::{Context, Tera};

use rust_embed::Embed;

use crate::templates::base::{TERA_EXT, TemplateInfo, split_path, tera_with_escape_settings};

#[derive(Embed)]
#[folder = "templates"]
#[prefix = "templates/"]
struct Templates;

pub(crate) type Templater = EmbeddedTemplater;

pub(crate) struct EmbeddedTemplater {
    tera: Tera,
    content_types: HashMap<String, ContentType>,
}

impl EmbeddedTemplater {
    pub(crate) fn render<C>(&self, template_name: &str, context: C) -> (ContentType, String)
    where
        C: Into<Context>,
    {
        (
            self.content_types.get(template_name).unwrap().to_owned(),
            self.tera.render(template_name, &context.into()).unwrap(),
        )
    }
}

fn load_templates() -> Vec<TemplateInfo> {
    //let mut templates: Vec<TemplateInfo> = Vec::new();
    let root = std::path::Path::new("templates");
    let template_iters = Templates::iter();
    let templates = template_iters
        .filter_map(|p| {
            let path = Path::new(p.as_ref());
            if !is_file_with_ext(&path, TERA_EXT) {
                return None;
            }
            let (template, data_type_str) = split_path(&root, path);
            let data_type = data_type_str
                .as_ref()
                .and_then(|ext| ContentType::from_extension(ext))
                .unwrap_or(ContentType::Text);
            Some(TemplateInfo {
                name: template,
                path: Some(path.to_path_buf()),
                data_type,
            })
        })
        .collect::<Vec<TemplateInfo>>();
    templates
}

pub(crate) fn init_template_provider() -> Templater {
    let templates = load_templates();

    let mut tera = tera_with_escape_settings();

    let typed_templates = HashMap::from_iter(
        templates
            .iter()
            .map(|ti| (ti.name.to_owned(), ti.data_type.clone())),
    );

    for ti in templates.iter() {
        let raw_data = Templates::get(&ti.path.as_ref().unwrap().to_str().unwrap()).unwrap();
        let template_data = String::from_utf8_lossy(raw_data.data.as_ref());
        let template = tera::Template::new(
            &ti.name,
            Some(
                ti.path
                    .as_ref()
                    .unwrap()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_owned(),
            ),
            &template_data,
        )
        .unwrap();
        tera.templates.insert(ti.name.to_owned(), template);
    }
    tera.build_inheritance_chains().unwrap();
    EmbeddedTemplater {
        tera: tera,
        content_types: typed_templates,
    }
}

fn is_file_with_ext(entry: &Path, ext: &str) -> bool {
    let is_file = entry.is_file();
    let has_ext = entry.extension().map_or(false, |e| e == ext);
    is_file && has_ext
}
