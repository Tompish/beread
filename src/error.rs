
#[derive(Debug)]
pub enum DocumentError{
    EndBeforeStart,
    LineOutOfRange(usize, usize),
    Deprecated(String),
    Other(String)
}

impl std::fmt::Display for DocumentError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::EndBeforeStart => write!(f, "Start character is after end character"),
            Self::LineOutOfRange(line_row, max_rows) => write!(f, "Can't edit line row {}. Document only has {} rows", line_row, max_rows),
            Self::Deprecated(mes) => write!(f, "{}", mes),
            _ => write!(f, "Unknown error")
        }
    }
}

impl std::error::Error for DocumentError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Debug)]
pub enum TableOfContentsError{
    DocumentError(DocumentError),
    ChangeOnUnopen
}

impl std::fmt::Display for TableOfContentsError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::DocumentError(document_error) => write!(f, "{}", document_error),
            Self::ChangeOnUnopen => write!(f, "Tried to change a document that has not been opened")
        }
    }
}

impl std::error::Error for TableOfContentsError{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self{
            Self::DocumentError(document_error) => Some(document_error),
            _ => None
        }
    }
}

impl From<DocumentError> for TableOfContentsError{
    fn from(value: DocumentError) -> Self {
        Self::DocumentError(value)
    }
}
