use std::{collections::HashMap, str::FromStr};
use lsp_types::Range;
use crate::{documentparams::{ChangeDocument, OpenDocument}, error, DocumentError};

#[derive(Debug, PartialEq, Eq)]
pub struct Document{
    pub version: i32,
    pub content: HashMap<usize, String>
}

impl Document{
    pub fn new() -> Self{
        Self{
            version: 1,
            content: HashMap::<usize, String>::new()
        }
    }

    fn delete_rows(&mut self, start_line: usize, end_line: usize){
        let nr_of_lines = end_line - start_line;

        //Assumption that hashmap has all its rows filled,
        //as a document can't be missing rows
        for line_nr in start_line..self.content.len()
        {
            match self.content.get(&(line_nr+nr_of_lines))
            {
                Some(text) => self.content.insert(line_nr, text.to_string()),
                None => None
            };
        }

        for line_nr in 0..nr_of_lines {
            self.content.remove(&(self.content.len()-1-line_nr));
        }
    }

    fn edit_line(&mut self, line: usize, start_char: usize, end_char: usize, text: &str) -> Result<(), DocumentError>{
        if start_char > end_char
        {
            return Err(DocumentError::EndBeforeStart);
        }

        match self.content.get_mut(&line)
        {
            Some(current_text) => current_text.replace_range(start_char..end_char, text),
            None => return Err(DocumentError::LineOutOfRange(line, self.content.len()))
        };

        Ok(())
    }

    fn edit_multiple_lines(&mut self, range: Range, text: &str) -> Result<(), DocumentError>
    {
        let mut iter = text.lines().enumerate().peekable();

        let _ = match iter.next()
        {
            Some((_, start_line)) => self.edit_line(range.start.line as usize, range.start.character as usize, start_line.len(), start_line),
            None => return Err(error::DocumentError::Other("Iterator failed to iterate over lines".to_string()))
        };

        while let Some((line, line_text)) = iter.next() {
            if iter.peek().is_none() {
                self.edit_line(range.end.line as usize, 0, range.end.character as usize, text)?;
                break;
            }

            self.content.insert(line, line_text.to_string());
        }

        Ok(())
    }
}

impl TDocument for Document
{
    fn open_document(&mut self, doc: OpenDocument){
        self.content = Document::from(doc.text).content;
    }

    fn change_document(&mut self, doc: ChangeDocument) -> Result<(), DocumentError>{
        for change in doc.content_changes.iter()
        {
            let range = change.range;

            let start_line : usize = u32::try_into(range.start.line).unwrap();
            let end_line : usize = u32::try_into(range.end.line).unwrap();
            let start_char : usize = range.start.character.try_into().unwrap();
            let end_char : usize = range.end.character.try_into().unwrap();

            if start_line > self.content.len()
            {
                return Err(DocumentError::LineOutOfRange(start_line, self.content.len()));
            }

            if start_line != end_line && change.text == *""{
                self.delete_rows(start_line, end_line);
            }
            else if start_line == end_line{
                self.edit_line(start_line, start_char, end_char, &change.text)?
            }
            else{
                self.edit_multiple_lines(range, &change.text)?
            }
        }
        Ok(())
    }
}

pub trait TDocument{
    fn change_document(&mut self, params: ChangeDocument) -> Result<(), DocumentError>;
    fn open_document(&mut self, params: OpenDocument);
}

impl FromStr for Document{
    type Err = DocumentError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl From<&str> for Document
{
    fn from(value: &str) -> Self {
        Self{
            version: 1,
            content: value.lines()
                .enumerate()
                .map(|l| (l.0, l.1.to_string()))
                .collect::<HashMap<_, _>>()
        }
    }
}

impl From<String> for Document
{
    fn from(value: String) -> Self {
        Self{
            version: 1,
            content: value.lines()
                .enumerate()
                .map(|l| (l.0, l.1.to_string()))
                .collect::<HashMap<_, _>>()
        }
    }
}

#[cfg(test)]
mod tests
{
    use lsp_types::Position;

    use crate::documentparams::ChangeEvent;

    use super::*;

    #[test]
    fn did_change_document_oneline_test()
    {
        let mut lib = Document::new();
        lib.content.insert(0, String::from("this is the first line"));
        lib.content.insert(1, String::from("don't fool the second line twice"));
        lib.content.insert(2, String::from("third line is written by a third party"));

        let params = get_change_params(0, 12, 0, 22, "this is a new text that is inserted");
        
        let _ = lib.change_document(params);

        assert_eq!(lib.content.get(&0usize).unwrap(), "this is the this is a new text that is inserted");
    }

    #[test]
    fn did_change_document_delete_test()
    {
        let mut lib = Document::new();
        lib.content.insert(0, String::from("this is the first line"));
        lib.content.insert(1, String::from("don't fool the second line twice"));
        lib.content.insert(2, String::from("third line is written by a third party"));

        let params = get_change_params(1, 0, 2, 0, "");

        let _ = lib.change_document(params);

        assert_eq!(lib.content.len(), 2);
    }

    #[test]
    fn did_open_document_test()
    {
        let params = get_open_params();

        let mut lib = Document::new();

        let _ = lib.open_document(params);

        assert_eq!(lib.content.len(), 3);
    }

    fn get_change_params(start_line: u32, start_char: u32, end_line: u32, end_char: u32, text: &str) -> ChangeDocument{
        ChangeDocument{
            content_changes: vec![ChangeEvent{
                range: Range{
                    start: Position{
                        line: start_line,
                        character: start_char
                    },
                    end: Position{
                        line: end_line,
                        character: end_char
                    }
                },
                text: text.to_string()
            }]
        }
    }

    fn get_open_params() -> OpenDocument{
        OpenDocument{
            text: "this is the first line
                    don't fool the second line twice
                    third line is written by a third party".to_string(),
        }
    }
}
