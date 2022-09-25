//
// ConnectingLines are the lines between capabilities and the surround that is supposed to
// implement it. This module creates one.
//

use prog_draw::geometry::{Coord, Point, Rect};
use prog_draw::svg_render::SvgPositioned;
use prog_draw::svg_writer::{Renderable, TagWriter, TagWriterError, Attributes};
use crate::used_by::{get_color_strs, UsedBySet};
use crate::document::CONNECT_DOT_RADIUS;


const LINE_CTRL_OFFSET: Coord = 10.0;



#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
    color: &'static str
}

#[derive(Debug)]
pub struct ConnectingLines {
    lines: Vec<Line>,
}


impl Default for ConnectingLines {
    fn default() -> Self {
        ConnectingLines{lines: Default::default()}
    }
}


impl ConnectingLines {
    /// Call this to clear out the existing lines (before adding new ones).
    pub fn clear(&mut self) {
        self.lines.clear();
    }

    /// Call this to add in a new line. It's promised that we're in a mode where we can safely
    /// get bbox data for the DTNode.
    pub fn add_line(&mut self, capability_pos: Point, surround_pos: Point, used_by_set: &UsedBySet) {
        let start = capability_pos;
        let end = surround_pos;
        let color = get_color_strs(&used_by_set).0;
        let line: Line = Line{start, end, color};
        self.lines.push(line);
    }
}



fn make_line_path(left: Point, right: Point) -> String {
    let left_ctrl_x = left.0 + LINE_CTRL_OFFSET;
    let right_ctrl_x = right.0 - LINE_CTRL_OFFSET;
    format_args!(
        "M {} {} C {} {}, {} {}, {} {}",
        left.0, left.1,
        left_ctrl_x, left.1,
        right_ctrl_x, right.1,
        right.0, right.1
    ).to_string()
}


impl Renderable for Line {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        tag_writer.single_tag("path", Attributes::from([
            ("d", &*make_line_path(self.start, self.end)),
            ("fill", "none"),
            ("stroke", self.color),
            ("stroke-width", "2.0"),
        ]))?;
        tag_writer.single_tag("circle", Attributes::from([
            ("cx", format!("{}", self.start.0).as_str()),
            ("cy", format!("{}", self.start.1).as_str()),
            ("r", &*CONNECT_DOT_RADIUS.to_string()),
            ("fill", "#FFFFFF"),
            ("stroke", "#000000"),
            ("stroke-width", "1.0"),
        ]))?;
        tag_writer.single_tag("circle", Attributes::from([
            ("cx", format!("{}", self.end.0).as_str()),
            ("cy", format!("{}", self.end.1).as_str()),
            ("r", &*CONNECT_DOT_RADIUS.to_string()),
            ("fill", "#FFFFFF"),
            ("stroke", "#000000"),
            ("stroke-width", "1.0"),
        ]))?;
        Ok(())
    }
}


impl Renderable for ConnectingLines {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        for line in self.lines.iter() {
            line.render(tag_writer)?;
        }
        Ok(())
    }
}


impl SvgPositioned for Line {
    fn get_bbox(&self) -> Rect {
        assert!(self.start.0 < self.end.0);
        if self.end.1 >= self.start.0 {
            Rect::new_ltrb(self.start.0, self.start.1, self.end.0, self.end.1)
        } else {
            Rect::new_ltrb(self.start.0, self.end.1, self.end.0, self.start.1)
        }
    }
}


impl SvgPositioned for ConnectingLines {
    fn get_bbox(&self) -> Rect {
        self.lines.iter()
            .map(|x| x.get_bbox())
            .reduce(|r1, r2| r1.cover(&r2))
            .unwrap_or(Rect::new_cwh((0.0, 0.0), 0.0, 0.0)) // FIXME: No way to say "don't cover anything" so this is what we'll do with no lines
    }
}
