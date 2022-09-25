//
// Support for document objects.
//

use std::collections::{HashMap, VecDeque};
use prog_draw::data_tree::{DTNode, LAYOUT_DIRECTION, TreeLayoutDirection};
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
use crate::used_by::UsedBySet;


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
    #[allow(dead_code)] // this IS used, but from javascript
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
    #[allow(dead_code)] // this IS used, but from javascript
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
    #[allow(dead_code)] // this IS used, but from javascript
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
            self.regenerate_connecting_lines();
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


    /// This both re-creates the connecting lines and repositions the items in the SurroundItems.
    /// It must be called AFTER the surround tree has been laid out correctly.
    fn regenerate_connecting_lines(&mut self) {
        // Before we try to get bounding boxes, need to set the tree direction
        let existing_direction = LAYOUT_DIRECTION.with(|it| it.get());
        LAYOUT_DIRECTION.with(|it| it.set(Some(TreeLayoutDirection::Right)));

        // This is the list of (requirement, surround) pairs. We'll use it for layout,
        // then for making lines.
        // FIXME: Can probably just store the LOCATION of the DTNode. Which would be better
        struct Connection<'a>(&'a DTNode<CapabilityData>, &'a str, UsedBySet);

        let mut connections: Vec<Connection> = Vec::new();

        // we'll use iteration instead of recursion, so here's our stack
        let mut node_stack: VecDeque<&DTNode<CapabilityData>> = VecDeque::new();
        node_stack.push_back(&self.surround_tree.tree);
        while !node_stack.is_empty() {
            let node = node_stack.pop_front().unwrap();
            if node.children.is_empty() || node.collapsed {
                // --- it's "leaf" (as visible on the screen now) ---
                for (surround_name, used_by_set) in self.capdb.get_related_surrounds(&node.data.id) {
                    match self.surrounds.get_by_name(surround_name) {
                        None => {
                            // FIXME: A better way to report this might be nice; this mostly just ignores bad data
                            println!("Could not find a surround named '{}' which is mentioned in {}. Skipped.", surround_name, node.data.id);
                        },
                        Some(surround_item) => {
                            connections.push(Connection(node, surround_item.id(), used_by_set));
                        }
                    }
                }
            } else{
                // --- not a "leaf" so "recurse" ---
                for child in node.children.iter() {
                    node_stack.push_back(child)
                }
            }
        }

        // Now that we have connections, we can find the list of connections for each unique surround
        let mut connections_by_surround_name: HashMap<&str, Vec<&Connection>> = HashMap::new();
        for connection in connections.iter() {
            let id = connection.1;
            match connections_by_surround_name.get_mut(id) {
                None => { // new surround
                    connections_by_surround_name.insert(id, vec![connection]);
                }
                Some(connection_list) => { // existing surround
                    connection_list.push(connection);
                }
            }
        }

        // Decide where to position the surrounds in the x direction
        let surround_x = self.surround_tree.get_bbox().right() + SPACING_TO_SURROUNDS;

        // Now we can position each surround vertically
        let mut desired_surround_positions: HashMap<String, Coord> = HashMap::new();
        for (id, connection_list) in connections_by_surround_name.iter() {
            assert!(connection_list.len() > 0);

            // Average the things it's connected to to find a y value
            let sum_of_y_values: Coord = connection_list.iter()
                .map(|connection| {
                    // FIXME: Getting the bbox to find the position is wasteful; we used that to build the bbox!
                    connection.0.data.get_bbox().center_y()
                })
                .sum();
            let average_y = sum_of_y_values / (connection_list.len() as Coord);
            desired_surround_positions.insert(id.to_string(), average_y);
        }

        // Now create the actual lines
        self.connecting_lines.clear();
        for connection in connections.iter() {
            let surround_y = *desired_surround_positions.get(connection.1).unwrap();
            self.connecting_lines.add_line(connection.0, (surround_x, surround_y), &connection.2);
        }

        // Now move the actual surrounds
        self.surrounds.set_x_position(surround_x);
        for (id, pos) in desired_surround_positions.iter() {
            self.surrounds.get_by_id_mut(id.as_str()).unwrap().reposition(*pos);
        }

        // Now that we're done, restore the tree direction
        LAYOUT_DIRECTION.with(|it| it.set(existing_direction));
    }
}
