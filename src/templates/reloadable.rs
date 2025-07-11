use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{
        Arc, RwLock,
        mpsc::{Receiver, channel},
    },
};

use rocket::http::ContentType;
use tera::{Context, Tera};

use crate::templates::base::{TERA_EXT, TemplateInfo, split_path, tera_with_escape_settings};

use notify::{Event, RecommendedWatcher, RecursiveMode, Result, Watcher, recommended_watcher};

pub(crate) type Templater = ReloadableTemplater;

struct TemplateInner {
    watcher: RecommendedWatcher,
    event_channel: Receiver<Result<Event>>,
    tera: Tera,
    content_types: HashMap<String, ContentType>,
}

pub(crate) struct ReloadableTemplater {
    inner: Arc<RwLock<TemplateInner>>,
}

unsafe impl Send for ReloadableTemplater {}
unsafe impl Sync for ReloadableTemplater {}

impl TemplateInner {
    pub fn reload_if_needed(&mut self) {
        let mut changes = false;
        while let Ok(_) = self.event_channel.try_recv() {
            changes = true;
        }
        if changes {
            println!("Template changes detected, reloading...");
            let (tera, template_types) = load_inner_props();
            self.tera = tera;
            self.content_types = template_types;
            println!("Templates reloaded!");
        }
    }
}

impl ReloadableTemplater {
    pub(crate) fn render<C>(&self, template_name: &str, context: C) -> (ContentType, String)
    where
        C: Into<Context>,
    {
        let mut inner_w = self.inner.write().unwrap();
        inner_w.reload_if_needed();
        drop(inner_w);
        let inner = self.inner.read().unwrap();
        (
            inner.content_types.get(template_name).unwrap().to_owned(),
            inner.tera.render(template_name, &context.into()).unwrap(),
        )
    }
}

fn load_templates() -> Vec<TemplateInfo> {
    let mut templates: Vec<TemplateInfo> = Vec::new();
    let root = std::path::Path::new("templates");
    for entry in walkdir::WalkDir::new(&root).follow_links(true) {
        let entry = match entry {
            Ok(entry) if is_file_with_ext(&entry, TERA_EXT) => entry,
            Ok(_) | Err(_) => continue,
        };

        let (template, data_type_str) = split_path(&root, entry.path());

        let data_type = data_type_str
            .as_ref()
            .and_then(|ext| ContentType::from_extension(ext))
            .unwrap_or(ContentType::Text);

        templates.push(TemplateInfo {
            name: template,
            path: Some(entry.into_path()),
            data_type,
        });
    }
    templates
}

fn load_inner_props() -> (Tera, HashMap<String, ContentType>) {
    let templates = load_templates();

    let mut tera = tera_with_escape_settings();

    let typed_templates = HashMap::from_iter(
        templates
            .iter()
            .map(|ti| (ti.name.to_owned(), ti.data_type.clone())),
    );

    let named_templates = templates
        .iter()
        .filter_map(|ti| Some((ti.name.as_str(), ti.path.as_ref()?)))
        .map(|(k, p)| (k, p.as_path()));

    let files = named_templates.map(|(name, path)| (path, Some(name)));

    // Finally try to tell Tera about all of the templates.
    tera.add_template_files(files).unwrap();

    (tera, typed_templates)
}

pub(crate) fn init_template_provider() -> Templater {
    let (tera, typed_templates) = load_inner_props();
    let root = std::path::Path::new("templates");
    let (tx, rx) = channel();

    let watcher = recommended_watcher(tx)
        .and_then(|mut watcher| {
            watcher.watch(&root.canonicalize()?, RecursiveMode::Recursive)?;
            Ok(watcher)
        })
        .unwrap();

    let t_inner = TemplateInner {
        watcher: watcher,
        event_channel: rx,
        tera: tera,
        content_types: typed_templates,
    };

    let m = RwLock::new(t_inner);
    let arc = Arc::new(m);

    ReloadableTemplater { inner: arc }
}

fn is_file_with_ext(entry: &walkdir::DirEntry, ext: &str) -> bool {
    let is_file = entry.file_type().is_file();
    let has_ext = entry.path().extension().map_or(false, |e| e == ext);
    is_file && has_ext
}
