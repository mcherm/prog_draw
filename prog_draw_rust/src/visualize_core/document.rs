//
// Support for document objects.
//

use crate::geometry::Coord;
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
pub const SVG_MARGIN: Coord = 0.0;



#[derive(Debug)]
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


        let core_tree_group = Group::item_transformed(&self.core_tree, Some((shift_dist * -1.0, 0.0)), None);
        let surround_tree_group = Group::item_transformed(&self.surround_tree, Some((shift_dist, 0.0)), None);
        let trifoil_group = Group::item_transformed(&trifoil::Trifoil, Some((0.0, -250.0)), Some(0.5));

        let content: [&dyn SvgPositioned; 4] = [
            &trifoil_group,
            &core_tree_group,
            &surround_tree_group,
            &CenterDot,
        ];
        let svg = Svg::new(Group::from(content), SVG_MARGIN);

        let mut tag_writer = TagWriterImpl::new(output);
        svg.render(&mut tag_writer)?;
        tag_writer.close()?;
        Ok(())
    }

    /// Toggles the collapsed state of a node. Leaf and Root nodes are unaffected.
    #[allow(dead_code)]
    pub fn toggle_collapse(&mut self, node_id: usize) {
        let should_layout_core_tree = self.core_tree.toggle_collapse(node_id);
        let should_layout_surround_tree = self.surround_tree.toggle_collapse(node_id);

        // FIXME: It would be better if the document maintained a needs_layout flag and
        //   performed the layout before returning svg.
        if should_layout_core_tree {
            self.core_tree.layout();
        }
        if should_layout_surround_tree {
            self.surround_tree.layout();
        }
    }

}

