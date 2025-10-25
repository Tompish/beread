use lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams};
pub use lsp_types::Range;

use crate::DocumentError;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OpenDocument{
    pub text: String
}

impl OpenDocument{
    pub fn new(s: &str) -> Self{
        Self{ text: s.to_string() }
    }
}

impl TryFrom<DidOpenTextDocumentParams> for OpenDocument{
    type Error = DocumentError;
    fn try_from(value: DidOpenTextDocumentParams) -> Result<Self, Self::Error> {
        Ok(
            Self{
            text: value.text_document.text.to_owned()
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChangeDocument{
    pub content_changes: Vec<ChangeEvent>
}

impl ChangeDocument{
    pub fn new() -> Self{
        Self{
            content_changes: Vec::<ChangeEvent>::new()
        }
    }
}

impl TryFrom<DidChangeTextDocumentParams> for ChangeDocument{
    type Error = DocumentError;

    fn try_from(params: DidChangeTextDocumentParams) -> Result<Self, Self::Error>{
            let content_changes = params.content_changes
                .into_iter()
                .map(|change| {
                    let range = change.range.ok_or(DocumentError::Deprecated("No range given. The deprecated range_length field is not supported".to_string()))?;
                    Ok(ChangeEvent::new(range, change.text))
                })
                .collect::<Result<Vec<_>, _>>()?;
        Ok(Self{ content_changes })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChangeEvent{
    pub range: Range,
    pub text: String,
}

impl ChangeEvent{
    pub fn new(range: Range, text: String) -> Self{
        Self{ range, text }
    }
}

