//
// Support for document objects.
//

use prog_draw::data_tree::DTNode;
use prog_draw::geometry::Coord;
use prog_draw::svg_writer::Renderable;
use prog_draw::svg_writer::{TagWriterImpl, TagWriter, TagWriterError};
use prog_draw::svg_render::{Group, Svg, SvgPositioned};
use prog_draw::geometry::Point;
use crate::trifoil;
use crate::capability_db::CapabilitiesDB;
use crate::capability_tree::{CapabilityData, CapabilityNodeTree, read_trees_from_capdb};
use crate::center_dot::CenterDot;
use crate::surrounds::SurroundItems;
use crate::connecting_lines::ConnectingLines;


pub const TEXT_ITEM_PADDING: Coord = 2.0;
pub const BASELINE_RISE: Coord = 2.0;
pub const NODE_ITEM_ROUND_CORNER: Coord = 3.0;
pub const CENTER_DOT_RADIUS: Coord = 40.0;
pub const COLLAPSE_DOT_RADIUS: Coord = 3.0;
pub const CONNECT_DOT_RADIUS: Coord = 2.0;
pub const ITEM_SPACING: Coord = 8.0; // min vertical space between adjacent boxes
pub const LAYER_SPACING: Coord = 16.0; // min horizontal space between layers in tree
pub const SPACING_TO_SURROUNDS: Coord = 3.0 * LAYER_SPACING;
pub const SVG_MARGIN: Coord = 4.0;
pub const TRIFOIL_SCALE: Coord = 0.5;
pub const TRIFOIL_MARGIN: Coord = 80.0;



#[derive(Debug)]
pub struct TwoTreeViewDocument {
    pub capdb: CapabilitiesDB,
    pub core_tree: CapabilityNodeTree,
    pub surround_tree: CapabilityNodeTree,
    pub surrounds: SurroundItems,
    pub connecting_lines: ConnectingLines,
}


// FIXME: This part should be in the std library, right?
struct WritableString {
    s: String,
}

impl std::io::Write for WritableString {
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
    pub fn new(capdb: CapabilitiesDB) -> Self {
        // --- get data objects ---
        let [core_tree, surround_tree] = read_trees_from_capdb(&capdb);
        let surrounds = SurroundItems::new(&capdb);
        let connecting_lines = Default::default();

        // --- create document ---
        let mut doc = TwoTreeViewDocument{capdb, core_tree, surround_tree, surrounds, connecting_lines};

        // --- perform layout ---
        doc.update_layout(true, true);

        // --- return it ---
        return doc
    }

    /// Return the contents of this document as an SVG string.
    pub fn get_svg_str(&self) -> Result<String,TagWriterError> {
        let mut output: WritableString = WritableString {s:String::new()};
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
        let surrounds_group = Group::item_transformed(&self.surrounds, Some((shift_dist, 0.0)), None);
        let connecting_lines_group = Group::item_transformed(&self.connecting_lines, Some((shift_dist, 0.0)), None);
        let trifoil_group = Group::item_transformed(&trifoil::Trifoil, Some(self.trifoil_position()), Some(TRIFOIL_SCALE));

        let content: [&dyn SvgPositioned; 6] = [
            &trifoil_group,
            &connecting_lines_group,
            &core_tree_group,
            &surround_tree_group,
            &CenterDot,
            &surrounds_group,
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
        self.update_layout(should_layout_core_tree, should_layout_surround_tree);
    }


    /// Toggles the collapsed state of the whole document to a named well-known state. If
    /// a name is passed in that isn't known, this will panic.
    ///
    /// NOTE: It uses a string instead of an enum because it was designed to interact
    ///   with JavaScript.
    pub fn refold(&mut self, named_fold: &str) {
        match named_fold {
            "LEVEL_2" => {
                fn apply_to_tree(node: &mut DTNode<CapabilityData>, depth: usize) {
                    if depth < 2 {
                        node.collapsed = false;
                        for child in node.children.iter_mut() {
                            apply_to_tree(child, depth + 1);
                        }
                    } else if depth == 2 {
                        node.collapsed = true;
                    }
                }
                apply_to_tree(&mut self.core_tree.tree, 0);
                apply_to_tree(&mut self.surround_tree.tree, 0);
            },
            "ALL_OPEN" => {
                fn apply_to_tree(node: &mut DTNode<CapabilityData>) {
                    node.collapsed = false;
                    for child in node.children.iter_mut() {
                        apply_to_tree(child);
                    }
                }
                apply_to_tree(&mut self.core_tree.tree);
                apply_to_tree(&mut self.surround_tree.tree);
            },
            _ => panic!("The name '{}' is not a known refold state.", named_fold)
        }
        self.update_layout(true, true);
    }


    fn update_layout(&mut self, should_layout_core_tree: bool, should_layout_surround_tree: bool) {
        // FIXME: It would be better if the document maintained a needs_layout flag and
        //   performed the layout before returning svg.
        if should_layout_core_tree {
            self.core_tree.layout();
        }
        if should_layout_surround_tree {
            self.surround_tree.layout();
            self.surrounds.layout(&self.surround_tree);
            self.connecting_lines = ConnectingLines::new(self);
        }
    }

    /// This finds a good place to put the key. It returns an (x,y) offset from the center
    /// that would be good to move it to.
    fn trifoil_position(&self) -> Point {
        let trifoil_bbox = trifoil::Trifoil.get_bbox().scaled_about_center(TRIFOIL_SCALE);
        let left_top = self.core_tree.get_bbox().top();
        let right_top = self.surround_tree.get_bbox().top();
        let best_top = left_top.max(right_top);
        let y_position = best_top + -trifoil_bbox.bottom() - TRIFOIL_MARGIN;
        let x_position = if left_top <= right_top {
            -trifoil_bbox.left() + TRIFOIL_MARGIN
        } else {
            trifoil_bbox.left() - TRIFOIL_MARGIN
        };
        (x_position, y_position)
    }
}
