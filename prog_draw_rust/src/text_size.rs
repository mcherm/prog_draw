///
/// A module for determining the size of text in various fonts. Doing so is actually fairly
/// tricky and OS-specific, so this has a wrapper trait with multiple implementations.
///

pub trait TextSizer {
    /// Given a string of text, a font_family (name of a font or list of different fonts
    /// to try in order), and a font size this attempts to return the width and height of the
    /// rendered text (as a single line; no wrapping), although it may also return an error.
    fn text_size(&self, text: &str, font_family: &str, font_size: f32) -> Result<(f32, f32), TextSizeError>;
}

#[derive(Debug)]
pub struct TextSizeError;

/// A default implementation for TextSizer that always fails.
struct AlwaysFailTextSizer;

impl TextSizer for AlwaysFailTextSizer {
    fn text_size(&self, _text: &str, _font_family: &str, _font_size: f32) -> Result<(f32, f32), TextSizeError> {
        Err(TextSizeError)
    }
}

static mut SYSTEM_TEXT_SIZER: &dyn TextSizer = &AlwaysFailTextSizer;

pub fn get_system_text_sizer() -> &'static dyn TextSizer {
    unsafe {
        SYSTEM_TEXT_SIZER
    }
}

pub unsafe fn set_system_text_sizer(text_sizer: &'static dyn TextSizer) {
    SYSTEM_TEXT_SIZER = text_sizer;
}
