use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use rocket::http::ContentType;
use tera::{Context, Tera};

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "templates"]
#[prefix = "templates/"]
struct Templates;

pub(crate) struct Templater {
    tera: Tera,
    content_types: HashMap<String, ContentType>,
}

pub(crate) struct TemplateInfo {
    pub(crate) name: String,
    /// The complete path, including `template_dir`, to this template, if any.
    pub(crate) path: Option<PathBuf>,
    /// The extension before the engine extension in the template, if any.
    pub(crate) data_type: ContentType,
}

impl Templater {
    pub fn render<C>(&self, template_name: &str, context: C) -> (ContentType, String)
    where
        C: Into<Context>,
    {
        (
            self.content_types.get(template_name).unwrap().to_owned(),
            self.tera.render(template_name, &context.into()).unwrap(),
        )
    }
}

static TERA_EXT: &str = "tera";

pub(crate) fn load_templates() -> Vec<TemplateInfo> {
    //let mut templates: Vec<TemplateInfo> = Vec::new();
    let root = std::path::Path::new("templates");
    let template_iters = Templates::iter();
    let templates = template_iters
        .map(|p| {
            let path = Path::new(p.as_ref());
            // let template = Templates::get(&p).unwrap();
            let (template, data_type_str) = split_path(&root, path);
            let data_type = data_type_str
                .as_ref()
                .and_then(|ext| ContentType::from_extension(ext))
                .unwrap_or(ContentType::Text);
            TemplateInfo {
                name: template,
                path: Some(path.to_path_buf()),
                data_type,
            }
        })
        .collect::<Vec<TemplateInfo>>();
    templates
}

pub(crate) fn init_template_provider() -> Templater {
    let templates = load_templates();

    let mut tera = Tera::default();
    let ext = [
        ".html.tera",
        ".htm.tera",
        ".xml.tera",
        ".html",
        ".htm",
        ".xml",
    ];
    tera.autoescape_on(ext.to_vec());

    let typed_templates = HashMap::from_iter(
        templates
            .iter()
            .map(|ti| (ti.name.to_owned(), ti.data_type.clone())),
    );

    #[cfg(not(debug_assertions))]
    {
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
    }
    #[cfg(debug_assertions)]
    {
        let named_templates = templates
            .iter()
            .filter_map(|ti| Some((ti.name.as_str(), ti.path.as_ref()?)))
            .map(|(k, p)| (k, p.as_path()));

        let files = named_templates.map(|(name, path)| (path, Some(name)));

        // Finally try to tell Tera about all of the templates.
        tera.add_template_files(files).unwrap();
    }
    Templater {
        tera: tera,
        content_types: typed_templates,
    }
}

/// Removes the file path's extension or does nothing if there is none.
fn remove_extension(path: &Path) -> PathBuf {
    let stem = match path.file_stem() {
        Some(stem) => stem,
        None => return path.to_path_buf(),
    };

    match path.parent() {
        Some(parent) => parent.join(stem),
        None => PathBuf::from(stem),
    }
}

fn split_path(root: &Path, path: &Path) -> (String, Option<String>) {
    let rel_path = path.strip_prefix(root).unwrap().to_path_buf();
    let path_no_ext = remove_extension(&rel_path);
    let data_type = path_no_ext.extension();
    let mut name = remove_extension(&path_no_ext)
        .to_string_lossy()
        .into_owned();

    // Ensure template name consistency on Windows systems
    if cfg!(windows) {
        name = name.replace('\\', "/");
    }

    (name, data_type.map(|d| d.to_string_lossy().into_owned()))
}
