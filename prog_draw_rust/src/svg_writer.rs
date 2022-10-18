
use std::fmt;
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

    fn write(&self, output: &mut dyn std::io::Write) -> Result<(), TagWriterError> {
        for (key, val) in &self.fields {
            // FIXME: For now, this lacks proper escaping for field values
            write!(*output, " {}=\"{}\"", key, val)?;
        }
        Ok(())
    }

    /// This consumes the Attributes but returns a new one with the additional key/value pair.
    pub fn with_field<K: ToString, V: ToString>(mut self, key: K, value: V) -> Self {
        // FIXME: Maybe I should check for duplicate keys
        self.fields.push((key.to_string(), value.to_string()));
        Attributes{fields: self.fields}
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



pub struct TagWriterImpl<'a> {
    output: &'a mut dyn std::io::Write,
    indent_level: usize,
    indent_str: String,
}


pub trait TagWriter {
    fn begin_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError>;
    fn end_tag(&mut self, tag: &str) -> Result<(), TagWriterError>;
    fn text(&mut self, text: &str) -> Result<(), TagWriterError>;
    fn raw_svg(&mut self, svg: &str) -> Result<(), TagWriterError>;
    fn single_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError>;
    fn tag_with_text(&mut self, tag: &str, attr: Attributes, text: &str) -> Result<(), TagWriterError>;
    fn close(&mut self) -> Result<(), TagWriterError>;
}


/// Given some text that should be in the body of a document (outside any tags), this returns
/// the escaped version of it.
pub fn xml_escape_body_text(s: &str) -> String {
    let partial_escaped = str::replace(s, "&", "&amp;");
    let escaped = str::replace(&partial_escaped, "<", "&lt;");
    escaped
}


impl<'a> TagWriterImpl<'a> {
    pub fn new(output: &'a mut dyn std::io::Write) -> Self {
        Self{output, indent_level: 0, indent_str: "  ".to_string()}
    }

    fn i_space(&self) -> String {
        self.indent_str.repeat(self.indent_level)
    }

}


impl<'a> TagWriter for TagWriterImpl<'a> {

    /// Begin a tag; all content will be on the next line and will be indented.
    fn begin_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(self.output)?;
        write!(self.output, ">\n")?;
        self.indent_level += 1;
        Ok(())
    }

    /// End a tag begun with begin_tag().
    fn end_tag(&mut self, tag: &str) -> Result<(), TagWriterError> {
        if self.indent_level == 0 {
            Err(TagWriterError::NotEnoughBeginTags)
        } else {
            self.indent_level -= 1;
            Ok(write!(self.output, "{}</{}>\n", self.i_space(), tag)?)
        }
    }

    /// Directly output body text into the document (escaping it first). Rarely used.
    #[allow(dead_code)] // Maybe even remove this if it isn't used!
    fn text(&mut self, text: &str) -> Result<(), TagWriterError> {
        write!(self.output, "{}", xml_escape_body_text(text))?;
        Ok(())
    }

    /// Directly output SVG content into the document. Used in specialized cases only.
    fn raw_svg(&mut self, svg: &str) -> Result<(), TagWriterError> {
        write!(self.output, "{}", svg)?;
        Ok(())
    }

    /// Generate a single-line tag without separate begin/end tags.
    fn single_tag(&mut self, tag: &str, attr: Attributes) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(self.output)?;
        write!(self.output, "/>\n")?;
        Ok(())
    }

    /// Creates a single-line tag with text inside the tag.
    fn tag_with_text(&mut self, tag: &str, attr: Attributes, text: &str) -> Result<(), TagWriterError> {
        write!(self.output, "{}<{}", self.i_space(), tag)?;
        attr.write(self.output)?;
        write!(self.output, ">{}</{}>\n", xml_escape_body_text(text), tag)?;
        Ok(())
    }

    /// Complete the SVG document. Will return an error if there are any unclosed tags.
    fn close(&mut self) -> Result<(), TagWriterError> {
        if self.indent_level > 0 {
            Err(TagWriterError::NotEnoughEndTags)
        } else {
            Ok(())
        }
    }
}


/// A trait for anything which can be rendered to SVG.
pub trait Renderable {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError>;
}
