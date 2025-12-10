mod formatter;
mod store;
pub mod types;

pub use formatter::apply_formatting;
pub use store::{load, save};
pub use types::{FormattingSettings};
