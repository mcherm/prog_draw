use lazy_static::lazy_static;

use rusttype::{point, Font, Scale};

lazy_static! {
    static ref ARIAL_FONT: Font<'static> = {
        let font_path = "/System/Library/Fonts/Supplemental/Arial.ttf";
        let data = std::fs::read(&font_path).unwrap();
        let font = Font::try_from_vec(data).unwrap();
        font
    };
}

#[derive(Debug)]
pub struct FontError;

/// This is passed a single string and returns the (width, height) of a box that
/// bounds that string when rendered. The size depends, of course, on the font
/// it will be rendered in. At the moment, it will *only* support one specific
/// font and only on MacOS when that font happens to be installed. Those limitations
/// could be lifted later. The size also depends on the size at which the font is
/// rendered.
pub fn text_size(text: &str, font_name: &str, font_size: f32) -> Result<(usize, usize), FontError> {
    if font_name != "Arial" {
        return Err(FontError);
    }
    let font = &ARIAL_FONT;

    let pixel_height = font_size.ceil() as usize;
    let scale = Scale {
        x: font_size,
        y: font_size,
    };

    // The origin of a line of text is at the baseline (roughly where
    // non-descending letters sit). We don't want to clip the text, so we shift
    // it down with an offset when laying it out. v_metrics.ascent is the
    // distance between the baseline and the highest edge of any glyph in
    // the font. That's enough to guarantee that there's no clipping.
    let v_metrics = font.v_metrics(scale);
    let offset = point(0.0, v_metrics.ascent);

    // Glyphs to draw for "RustType". Feel free to try other strings.
    let glyphs: Vec<_> = font.layout(text, scale, offset).collect();

    let pixel_width = glyphs
        .iter()
        .rev()
        .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
        .next()
        .unwrap_or(0.0)
        .ceil() as usize;

    Ok((pixel_width, pixel_height))
 }


#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn invoke_text_size() {
        let text = "Hello, World";
        let font_name = "Arial";
        let font_size = 12.4;

        match text_size(text, font_name, font_size) {
            Ok((width, height)) => {
                assert_eq!(width, 121);
                assert_eq!(height, 13);
            },
            Err(_) => assert!(false),
        }
    }


    #[test]
    fn text_size_with_invalid_font() {
        let text = "Hello, world";
        let font_name = "InvalidFont";
        let font_size = 12.4;

        match text_size(text, font_name, font_size) {
            Ok(_) => assert!(false),
            Err(_) => {}, // expected
        }
    }
}
