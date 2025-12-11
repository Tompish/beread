use std::collections::HashMap;
use lsp_types::{DidChangeTextDocumentParams, DidOpenTextDocumentParams, Uri};
use crate::error::TableOfContentsError;
use crate::{Document, TDocument};
use crate::documentparams::ChangeDocument;


pub struct TableOfContents{
    pub documents: HashMap<Uri, Document>
}

impl TableOfContents
{
    pub fn new() -> TableOfContents{
        TableOfContents{
            documents: HashMap::<Uri, Document>::new()
        }
    }

    pub fn did_change_document(&mut self, params: DidChangeTextDocumentParams) -> Result<(), TableOfContentsError>{
        match self.documents.get_mut(&params.text_document.uri)
        {
            Some(document) => {
                document.version = params.text_document.version;
                let document_param: ChangeDocument = params.try_into()?;
                match document.change_document(document_param)
                {
                    Ok(_) => Ok(()),
                    Err(doc_err) => Err(TableOfContentsError::DocumentError(doc_err))
                }
            },
            None => Err(TableOfContentsError::ChangeOnUnopen)
        }
    }

    pub fn did_open_document(&mut self, params: DidOpenTextDocumentParams) -> Result<(), TableOfContentsError>{
        
        let doc: Document = params.text_document.text.into();
        match self.documents.insert(params.text_document.uri, doc)
        {
            _ => Ok(()),
        }
    }

    pub fn get_document(&self, uri: &Uri) -> Option<&HashMap<usize, String>>{
        match self.documents.get(&uri)
        {
            Some(document) => Some(&document.content),
            _ => None
        }
    }

    //Returns deleted document, if there was a match
    pub fn delete_document(&mut self, uri: Uri) -> Option<Document>{
        self.documents.remove(&uri)
    }
}

#[cfg(test)]
mod tests{
    
    use lsp_types::{Position, Range, TextDocumentContentChangeEvent, TextDocumentItem, VersionedTextDocumentIdentifier};
    use super::*;

    #[test]
    fn did_open_document_test()
    {
        let params = get_open_params();
        let file_uri: Uri = "path/myfile.rs".parse().unwrap();
        let mut table = TableOfContents::new();

        let _ = table.did_open_document(params);

        assert_eq!(table.documents.len(), 1);

        let doc = &table.documents[&file_uri];
        let second_row = &doc.content[&1usize];
        assert_eq!(second_row, "don't fool the second line twice");
    }

    #[test]
    fn change_unopened_doc_test(){
        let params = get_change_params(1, 0, 2, 0, "this is\na text");
        let mut table = TableOfContents::new();

        let res = table.did_change_document(params)
            .is_err_and(|e| { 
                match e { 
                    TableOfContentsError::ChangeOnUnopen => true,
                    _ => false
                }
            });
        assert!(res);
    }

    #[test]
    fn change_opened_doc_test(){
        let open_params = get_open_params();
        let uri = open_params.text_document.uri.clone();
        let change_params = get_change_params(1, 0, 1, 26, "new text");

        let mut table = TableOfContents::new();

        let _ = table.did_open_document(open_params);
        let _ = table.did_change_document(change_params);

        let second_row = &table.documents[&uri].content[&1usize];
        assert_eq!(second_row, "new text twice");
    }

    fn get_change_params(start_line: u32, start_char: u32, end_line: u32, end_char: u32, text: &str) -> DidChangeTextDocumentParams{
        DidChangeTextDocumentParams{
            text_document: VersionedTextDocumentIdentifier{
                uri: "path/myfile.rs".parse().unwrap(),
                version: 2
            },
            content_changes: vec![TextDocumentContentChangeEvent{
                range: Some(Range{
                    start: Position{
                        line: start_line,
                        character: start_char
                    },
                    end: Position{
                        line: end_line,
                        character: end_char
                    }
                }),
                range_length: None,
                text: text.to_string()
            }]
        }
    }

    fn get_open_params() -> DidOpenTextDocumentParams{
        DidOpenTextDocumentParams{
            text_document: TextDocumentItem{
                language_id: "Rust".to_string(),
                uri: "path/myfile.rs".parse().unwrap(),
                version: 1,
                text: 
                    "this is the first line\ndon't fool the second line twice\nthird line is written by a third party".to_string(),
            },
        }
    }
}
