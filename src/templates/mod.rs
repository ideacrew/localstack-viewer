mod shared;

#[cfg(debug_assertions)]
mod reloadable;

#[cfg(debug_assertions)]
pub(crate) use crate::templates::reloadable::{Templater, init_template_provider};

#[cfg(not(debug_assertions))]
mod compiled;

#[cfg(not(debug_assertions))]
pub(crate) use crate::templates::compiled::{Templater, init_template_provider};
