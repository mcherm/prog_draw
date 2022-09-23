//
// Support for document objects.
//

use prog_draw::geometry::Coord;
use prog_draw::svg_writer::Renderable;
use prog_draw::svg_writer::{TagWriterImpl, TagWriter, TagWriterError};
use prog_draw::svg_render::{Group, Svg, SvgPositioned};
use crate::{capability_db, trifoil};
use crate::capability_tree::{CapabilityData, CapabilityNodeTree, read_trees_from_capdb};
use crate::center_dot::CenterDot;


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
    pub fn new(capdb: capability_db::CapabilitiesDB) -> Self {
        // --- get trees ---
        let [mut core_tree, mut surround_tree] = read_trees_from_capdb(capdb);

        // --- perform layout ---
        core_tree.layout();
        surround_tree.layout();

        // --- return document ---
        TwoTreeViewDocument{core_tree, surround_tree}
    }

    /// Return the contents of this document as an SVG string.
    pub fn get_svg_str(&self) -> Result<String,TagWriterError> {
        let mut output: MyString = MyString{s:String::new()};
        self.output_to(&mut output)?;
        Ok(output.s)
    }

    /// Returns the CapabilityData with that node_id if it exists; None if not.
    pub fn get_node_data(&self, id: &str) -> Option<&CapabilityData> {
        // NOTE: The tricky bit is that it could be in either tree (and we don't care which it's in)
        match self.core_tree.find_data_by_id(id) {
            Some(data) => Some(data),
            None => self.surround_tree.find_data_by_id(id),
        }
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
    pub fn toggle_collapse(&mut self, node_id: &str) {
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
