use super::super::svg_writer::{Attributes, Context, Renderable, TagWriter, TagWriterError};
use super::super::svg_render::{SvgPositioned, geometry::Rect};
use super::CENTER_DOT_RADIUS;


pub struct CenterDot;

impl Renderable for CenterDot {
    fn render(&self, tag_writer: &mut TagWriter, _context: &mut Context) -> Result<(), TagWriterError> {
        tag_writer.single_tag("circle", Attributes::from([
            ("cx", "0"),
            ("cy", "0"),
            ("r", &*CENTER_DOT_RADIUS.to_string()),
        ]))?;
        Ok(())
    }
}

impl SvgPositioned for CenterDot {
    fn get_bbox(&self, _context: &mut Context) -> Rect {
        Rect::new_cwh((0.0,0.0), 2.0 * CENTER_DOT_RADIUS, 2.0 * CENTER_DOT_RADIUS)
    }
}


