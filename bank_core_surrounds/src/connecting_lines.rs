//
// ConnectingLines are the lines between capabilities and the surround that is supposed to
// implement it. This module creates one.
//

use std::collections::VecDeque;
use prog_draw::data_tree::{DTNode, LAYOUT_DIRECTION, TreeLayoutDirection};
use prog_draw::geometry::{Coord, Point, Rect};
use prog_draw::svg_render::SvgPositioned;
use prog_draw::svg_writer::{Renderable, TagWriter, TagWriterError, Attributes};
use crate::capability_tree::CapabilityData;
use crate::used_by::get_color_strs;
use crate::document::{TwoTreeViewDocument, CONNECT_DOT_RADIUS};


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
    /// Create a ConnectingLines. It is passed a document in which the surround_tree and
    /// the surrounds have already been laid out.
    pub fn new(doc: &TwoTreeViewDocument) -> Self {
        // we will fill this in
        let mut lines: Vec<Line> = Vec::new();

        // Before we try to get bounding boxes, need to set the tree direction
        LAYOUT_DIRECTION.with(|it| it.set(Some(TreeLayoutDirection::Right)));

        // we'll use iteration instead of recursion, so here's our stack
        let mut node_stack: VecDeque<&DTNode<CapabilityData>> = VecDeque::new();
        node_stack.push_back(&doc.surround_tree.tree);
        while !node_stack.is_empty() {
            let node = node_stack.pop_front().unwrap();
            if node.children.is_empty() || node.collapsed {
                // --- it's "leaf" (as visible on the screen now) ---
                for (surround_name, used_by_set) in doc.capdb.get_related_surrounds(&node.data.id) {
                    let color = get_color_strs(&used_by_set).0;
                    let cap_bbox = node.get_bbox();
                    let start: Point = (cap_bbox.right(), cap_bbox.center_y());
                    let surround = doc.surrounds.get_by_name(surround_name).expect("Surround not found.");
                    let sur_bbox = surround.get_bbox();
                    let end: Point = (sur_bbox.left(), sur_bbox.center_y());
                    let line: Line = Line{start, end, color};
                    lines.push(line)
                }
            } else{
                // --- not a "leaf" so "recurse" ---
                for child in node.children.iter() {
                    node_stack.push_back(child)
                }
            }
        }

        // Now that we're done, restore the tree direction
        LAYOUT_DIRECTION.with(|it| it.set(None));

        // --- return the answer ---
        Self{lines}
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
