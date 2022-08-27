use std::fs::File;
use std::fmt;
use std::io::Write;
use std::error::Error;


#[derive(Debug)]
pub enum TagWriterError {
    IoError(std::io::Error),
    NotEnoughEndTags,
    NotEnoughBeginTags,
}
impl From<std::io::Error> for TagWriterError {
    fn from(error: std::io::Error) -> Self {
        TagWriterError::IoError(error)
    }
}
impl fmt::Display for TagWriterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TagWriterError::IoError(err)   => write!(f, "{}", err),
            TagWriterError::NotEnoughEndTags => write!(f, "Not enough end tags."),
            TagWriterError::NotEnoughBeginTags => write!(f, "Not enough begin tags."),
        }
    }
}

impl Error for TagWriterError {
}


pub struct Attributes {
    fields: Vec<(String, String)>,
}

impl Attributes {
    pub fn new() -> Self {
        Attributes { fields: Vec::new() }
    }

    fn write(&self, output: &mut File) -> Result<(), TagWriterError> {
        for (key, val) in &self.fields {
            // FIXME: For now, this lacks proper escaping for field values
            write!(*output, " {}=\"{}\"", key, val)?;
        }
        Ok(())
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for Attributes
    where
        K: ToString,
        V: ToString,
{
    fn from(arr: [(K,V); N]) -> Self {
        let mut fields: Vec<(String,String)> = Vec::with_capacity(N);
        for (k,v) in arr {
            // FIXME: Maybe I should check for duplicate keys
            fields.push((k.to_string(), v.to_string()));
        }
        Attributes{fields}
    }
}



pub struct TagWriter {
    output: File, // FIXME: Should be more generic like std::io::Write, but I'm not yet skilled enough to do that
    indent_level: usize,
    indent_str: String,
}


/// Given some text that should be in the body of a document (outside any tags), this returns
/// the escaped version of it.
pub fn xml_escape_body_text(s: &str) -> String {
    let partial_escaped = str::replace(s, "&", "&amp;");
    let escaped = str::replace(&partial_escaped, "<", "&lt;");
    escaped
}


impl TagWriter {
    pub fn new(output: File) -> TagWriter {
        TagWriter{output, indent_level: 0, indent_str: "  ".to_string()}
    }

    fn i_space(&self) -> String {
        self.indent_str.repeat(self.indent_level)
    }

    /// Begin a tag; all content will be on the next line and will be indented.
    pub fn begin_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(&mut self.output)?;
        write!(self.output, ">\n")?;
        self.indent_level += 1;
        Ok(())
    }

    /// End a tag begun with begin_tag().
    pub fn end_tag(&mut self, tag: &str) -> Result<(), TagWriterError> {
        if self.indent_level == 0 {
            Err(TagWriterError::NotEnoughBeginTags)
        } else {
            self.indent_level -= 1;
            Ok(write!(self.output, "{}</{}>\n", self.i_space(), tag)?)
        }
    }

    /// Directly output text into the document. Rarely used.
    #[allow(dead_code)] // Maybe even remove this if it isn't used!
    pub fn text(&mut self, text: &str) -> Result<(), TagWriterError> {
        write!(self.output, "{}", xml_escape_body_text(text))?;
        Ok(())
    }

    /// Generate a single-line tag without separate begin/end tags.
    pub fn single_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(&mut self.output)?;
        write!(self.output, "/>\n")?;
        Ok(())
    }

    /// Creates a single-line tag with text inside the tag.
    pub fn tag_with_text(&mut self, tag: &str, attr: Attributes, text: &str) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(&mut self.output)?;
        write!(self.output, ">{}</{}>\n", xml_escape_body_text(text), tag)?;
        Ok(())
    }

    /// Complete the SVG document. Will return an error if there are any unclosed tags.
    pub fn close(&mut self) -> Result<(), TagWriterError> {
        if self.indent_level > 0 {
            Err(TagWriterError::NotEnoughEndTags)
        } else {
            Ok(())
        }
    }
}

/// A trait for anything which can be rendered to SVG.
pub trait Renderable {
    fn render(&self, tag_writer: &mut TagWriter) -> Result<(), TagWriterError>;
}
