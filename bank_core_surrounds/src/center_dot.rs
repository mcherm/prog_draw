use prog_draw::svg_writer::{Attributes, Renderable, TagWriter, TagWriterError};
use prog_draw::svg_render::SvgPositioned;
use prog_draw::geometry::Rect;
use crate::document::CENTER_DOT_RADIUS;


pub struct CenterDot;

impl Renderable for CenterDot {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        tag_writer.single_tag("circle", Attributes::from([
            ("cx", "0"),
            ("cy", "0"),
            ("r", &*CENTER_DOT_RADIUS.to_string()),
        ]))?;
        Ok(())
    }
}

impl SvgPositioned for CenterDot {
    fn get_bbox(&self) -> Rect {
        Rect::new_cwh((0.0,0.0), 2.0 * CENTER_DOT_RADIUS, 2.0 * CENTER_DOT_RADIUS)
    }
}


