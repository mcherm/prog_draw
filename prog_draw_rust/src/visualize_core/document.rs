//
// Support for document objects.
//

use crate::svg_render::geometry::Coord;
use crate::svg_writer::Renderable;
use crate::visualize_core::trifoil;
use super::super::svg_writer::{TagWriterImpl, TagWriter, TagWriterError};
use super::super::svg_render::{Group, Svg, SvgPositioned};
use super::capability_tree::CapabilityNodeTree;
use super::center_dot::CenterDot;


pub const TEXT_ITEM_PADDING: Coord = 2.0;
pub const BASELINE_RISE: Coord = 2.0;
pub const NODE_ITEM_ROUND_CORNER: Coord = 3.0;
pub const CENTER_DOT_RADIUS: Coord = 16.0;
pub const COLLAPSE_DOT_RADIUS: Coord = 3.0;
pub const SVG_MARGIN: Coord = 10.0;



pub struct TwoTreeViewDocument {
    core_tree: CapabilityNodeTree,
    surround_tree: CapabilityNodeTree,
}


// FIXME: This part shouldn't be needed
struct MyString {
    s: String,
}

impl std::io::Write for MyString {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let new_str = match std::str::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, e)),
        };
        self.s.push_str(new_str);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}


impl TwoTreeViewDocument {
    pub fn new(mut core_tree: CapabilityNodeTree, mut surround_tree: CapabilityNodeTree) -> Self {
        // --- perform layout ---
        core_tree.layout();
        surround_tree.layout();

        TwoTreeViewDocument{core_tree, surround_tree}
    }

    /// Return the contents of this document as an SVG string.
    #[allow(dead_code)] // FIXME: Later we will use this. Still working before that's ready to go
    pub fn get_svg_str(&self) -> Result<String,TagWriterError> {
        let mut output: MyString = MyString{s:String::new()};
        self.output_to(&mut output)?; // FIXME: Wrong args; here and elsewhere
        Ok(output.s)
    }

    pub fn output_to(&self, output: &mut dyn std::io::Write) -> Result<(),TagWriterError> {
        let shift_dist = CENTER_DOT_RADIUS - 2.0 * TEXT_ITEM_PADDING;

        let core_tree_group = Group::item_transformed(Box::new(self.core_tree.clone()), &format!("translate({},0)", shift_dist * -1.0));
        let surround_tree_group = Group::item_transformed(Box::new(self.surround_tree.clone()), &format!("translate({},0)", shift_dist));
        let trifoil_group = Group::item_transformed(Box::new(trifoil::Trifoil), "translate(0 -250) scale(0.5)");

        let content: [Box<dyn SvgPositioned>; 4] = [
            Box::new(trifoil_group),
            Box::new(core_tree_group),
            Box::new(surround_tree_group),
            Box::new(CenterDot),
        ];
        let svg = Svg::new(Group::from(content), SVG_MARGIN);

        let mut tag_writer = TagWriterImpl::new(output);
        svg.render(&mut tag_writer)?;
        tag_writer.close()?;
        Ok(())
    }

}

