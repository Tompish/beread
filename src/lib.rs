mod document;
mod documentparams;
mod tableofcontents;
mod error;

pub use document::{TDocument, Document};
pub use lsp_types::{Position, Range};
pub use error::DocumentError;
pub use tableofcontents::*;

