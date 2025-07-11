use std::path::{Path, PathBuf};

use rocket::http::ContentType;
use tera::Tera;

pub(crate) static TERA_EXT: &str = "tera";

static TERA_ESCAPED_EXTENSIONS: [&str; 6] = [
    ".html.tera",
    ".htm.tera",
    ".xml.tera",
    ".html",
    ".htm",
    ".xml",
];

pub(crate) struct TemplateInfo {
    pub(crate) name: String,
    /// The complete path, including `template_dir`, to this template, if any.
    pub(crate) path: Option<PathBuf>,
    /// The extension before the engine extension in the template, if any.
    pub(crate) data_type: ContentType,
}

pub(crate) fn tera_with_escape_settings() -> Tera {
    let mut tera = Tera::default();
    tera.autoescape_on(TERA_ESCAPED_EXTENSIONS.to_vec());
    tera
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

pub(crate) fn split_path(root: &Path, path: &Path) -> (String, Option<String>) {
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
