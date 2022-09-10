
use std::collections::HashMap;
use std::cell::Cell;
use itertools::Itertools;
use crate::visualize_core::document::{BASELINE_RISE, COLLAPSE_DOT_RADIUS, NODE_ITEM_ROUND_CORNER, TEXT_ITEM_PADDING};
use super::super::data_tree::{DTNode, DTNodeBuild, DTNodeBuild::{AddData, EndChildren, StartChildren}, InvalidGrowth, LAYOUT_DIRECTION, TreeLayoutDirection};
use super::super::svg_render::{geometry::{Coord, Rect}, SvgPositioned};
use super::super::svg_writer::{Attributes, Renderable, TagWriter, TagWriterError};
use super::super::text_size::text_size;
use super::super::tidy_tree::{NULL_ID, TidyTree};
use super::lob_usage::{get_color_strs, LobUsage};
use super::fold_up;



#[derive(Debug, Eq, PartialEq)]
#[derive(Clone)] // FIXME: It shouldn't need to be cloned; I can fix that later
pub enum NodeLocationStyle {
    RootNode,
    BranchNode,
    LeafNode,
}

#[derive(Debug)]
#[derive(Clone)] // FIXME: It shouldn't need to be cloned; I can fix that later
pub struct CapabilityNode {
    id: usize,
    text: String,
    lob_usage: Option<LobUsage>,
    collapsed: bool,
    location: (f64, f64),
    node_loc_style: NodeLocationStyle,
}

#[derive(Clone)] // FIXME: It shouldn't need to be cloned; I can fix that later
pub struct CapabilityNodeTree {
    tree: DTNode<CapabilityNode>,
    layout_direction: TreeLayoutDirection,
    tree_collapse_policy: TreeCollapsePolicy,
}


/// Specifies which of several options is used for supporting collapsing of the tree.
#[derive(Debug, Copy, Clone)]
pub enum TreeCollapsePolicy {
    Nothing,
    JavaScriptReplace,
}

thread_local!{
    /// This threadlocal variable is set before rendering to say which what rules should be
    /// used for sketching the tree.
    static TREE_COLLAPSE_POLICY: Cell<TreeCollapsePolicy> = Cell::new(Default::default());
}




impl CapabilityNode {
    pub fn new(text: &str, lob_usage: Option<LobUsage>, id_source: &mut usize, collapsed: bool) -> Self {
        let id = *id_source;
        let text = text.to_string();
        let location = (0.0, 0.0); // default location until it gets repositioned
        let node_loc_style = NodeLocationStyle::BranchNode; // everything is assumed to be a branch until proven otherwise
        *id_source += 1;
        CapabilityNode{id, text, lob_usage, collapsed, location, node_loc_style}
    }

    /// Returns the (width, height) of the text string.
    fn text_size(&self) -> (Coord, Coord) {
        let maybe_size = text_size(&self.text, "Arial", 14.0);
        if maybe_size.is_err() {
            panic!("Sizing for Arial font isn't working.");
        }
        let (width_int, height_int) = maybe_size.unwrap();
        (width_int as Coord, height_int as Coord)
    }
}


impl Renderable for CapabilityNode {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        // --- Decide the dimensions of everything ---
        let (loc_x, loc_y) = self.location;
        let (text_width, text_height) = self.text_size();
        let text_left = loc_x + match LAYOUT_DIRECTION.with(|it| it.get()) {
            Some(TreeLayoutDirection::Right) => TEXT_ITEM_PADDING,
            Some(TreeLayoutDirection::Left) => (TEXT_ITEM_PADDING + text_width) * -1.0,
            None => panic!("No layout direction set."),
        };
        let text_top = loc_y - text_height / 2.0;
        let text_baseline = text_top + text_height - BASELINE_RISE;
        let box_left = text_left - TEXT_ITEM_PADDING;
        let box_top = text_top - TEXT_ITEM_PADDING;
        let box_width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let box_height = text_height + 2.0 * TEXT_ITEM_PADDING;

        // --- decide on decoration & color ---
        let class: &str = match self.node_loc_style {
            NodeLocationStyle::RootNode => "root",
            NodeLocationStyle::BranchNode => if self.collapsed {"leaf"} else {"branch"},
            NodeLocationStyle::LeafNode => "leaf",
        };
        let (box_color, text_color) = if self.collapsed {
            // FIXME: Here we OUGHT to recurse over descendants to accumulate the total lob_usage.
            //   But we can't, because there's no way to walk the tree from here. Maybe
            //   someday I can fix it with ideas from this:
            //   https://eli.thegreenplace.net/2021/rust-data-structures-with-circular-references/
            get_color_strs(self.lob_usage)
        } else {
            get_color_strs(self.lob_usage)
        };

        // --- Decide how we're handling collapsed things ---
        struct JSReplaceData {
            control_cx: Coord,
            control_cy: Coord,
            fill: String,
            onclick: String,
        }
        let right_left = match LAYOUT_DIRECTION.with(|it| it.get()) {
            Some(TreeLayoutDirection::Left) => -1.0,
            _ => 1.0
        };
        let jsreplace_data = match (&self.node_loc_style, TREE_COLLAPSE_POLICY.with(|it| it.get())) {
            (NodeLocationStyle::BranchNode, TreeCollapsePolicy::JavaScriptReplace) => Some(JSReplaceData{
                control_cx: loc_x + box_width * right_left,
                control_cy: loc_y,
                fill: (if self.collapsed {"#000000"} else {"#FFFFFF"}).to_string(),
                onclick: format_args!(
                    "alert('{} {}')",
                    if self.collapsed {"Open"} else {"Close"},
                    self.id
                ).to_string(),
            }),
            (_, _) => None, // show nothing if that's how we want to render it
        };

        // --- draw it ---
        if self.node_loc_style != NodeLocationStyle::RootNode {
            tag_writer.single_tag("rect", Attributes::from([
                ("x", &*box_left.to_string()),
                ("y", &*box_top.to_string()),
                ("width", &*box_width.to_string()),
                ("height", &*box_height.to_string()),
                ("rx", &*NODE_ITEM_ROUND_CORNER.to_string()),
                ("fill", box_color),
                ("stroke", "black"),
                ("stroke-width", &*1.to_string()),
                ("class", class)
            ]))?;
            tag_writer.tag_with_text(
                "text",
                Attributes::from([
                    ("x", &*text_left.to_string()),
                    ("y", &*text_baseline.to_string()),
                    ("font-family", "Arial"),
                    ("fill", text_color),
                    ("style", "font-style: normal; font-size: 12.4px"), // FIXME: size for 14 and set this to 12.4 seems to work. WHY?
                    ("class", class),
                ]),
                &self.text
            )?;
            match jsreplace_data {
                None => {},
                Some(jsreplace_data) => {
                    tag_writer.single_tag("circle", Attributes::from([
                        ("cx", &*jsreplace_data.control_cx.to_string()),
                        ("cy", &*jsreplace_data.control_cy.to_string()),
                        ("r", &*COLLAPSE_DOT_RADIUS.to_string()),
                        ("fill", &jsreplace_data.fill),
                        ("stroke", "#000000"),
                        ("stroke-width", "1.0"),
                        ("onclick", &jsreplace_data.onclick),
                    ]))?;
                },
            }
        }
        Ok(())
    }
}


impl SvgPositioned for CapabilityNode {
    // Gives the bounding box for the node including text AND the box around it. Remember, if
    // the node isn't correctly positioned yet, its location will be (0,0). Also know that
    // self.location is the center-left of the box it occupies.
    fn get_bbox(&self) -> Rect {
        let center = self.location;
        let (text_width, text_height) = self.text_size();
        let width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let height = text_height + 2.0 * TEXT_ITEM_PADDING;
        let left = center.0 - match LAYOUT_DIRECTION.with(|it| it.get()) {
            Some(TreeLayoutDirection::Right) => 0.0,
            Some(TreeLayoutDirection::Left) => width,
            None => panic!("No layout direction set."),
        };
        let top = center.1 - height / 2.0;
        Rect::new_ltwh(left, top, width, height)
    }
}


impl CapabilityNodeTree {
    pub fn new(layout_direction: TreeLayoutDirection, tree_collapse_policy: TreeCollapsePolicy) -> Self {
        let mut id_source = 0;
        let tree = DTNode::new(CapabilityNode::new("", None, &mut id_source, false));
        CapabilityNodeTree {tree, layout_direction, tree_collapse_policy}
    }

    /// Adds nodes to the tree.
    pub fn grow_tree(&mut self, items: impl IntoIterator<Item=DTNodeBuild<CapabilityNode>>) -> Result<(),InvalidGrowth> {
        self.tree.grow_tree(items)
    }

    /// Performs layout of the nodes.
    pub fn layout(&mut self) {
        // --- use tidy-tree to lay it out ---
        let mut tidy = TidyTree::with_tidy_layout(16.0, 8.0);
        LAYOUT_DIRECTION.with(|it| it.set(Some(self.layout_direction)));
        add_to_tidy(&mut tidy, &self.tree, NULL_ID);
        tidy.layout();
        LAYOUT_DIRECTION.with(|it| it.set(None));
        let locations: HashMap<usize, (f64, f64)> = tidy.get_pos().iter()
            .tuples::<(_,_,_)>() // break into groups of 3
            .map(|(id,x,y)| (*id as usize, match self.layout_direction {
                TreeLayoutDirection::Right => (*x, *y),
                TreeLayoutDirection::Left => (*x, *y * -1.0),
            })) // convert to ID and (x,y)
            .collect(); // and collect into a hashmap

        // set the location field in each one.
        populate_locations(&mut self.tree, &locations);

        // set the node_loc_style
        set_node_loc_style(&mut self.tree);
    }
}

impl Renderable for CapabilityNodeTree {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        tag_writer.begin_tag("g", Attributes::new())?;
        let style_text = r#"
          text.leaf {
            pointer-events: none;
          }
        "#;
        tag_writer.tag_with_text("style", Attributes::new(), style_text)?;
        LAYOUT_DIRECTION.with(|it| it.set(Some(self.layout_direction)));
        TREE_COLLAPSE_POLICY.with(|it| it.set(self.tree_collapse_policy));
        self.tree.render(tag_writer)?;
        TREE_COLLAPSE_POLICY.with(|it| it.set(Default::default()));
        LAYOUT_DIRECTION.with(|it| it.set(None));
        tag_writer.end_tag("g")?;
        Ok(())
    }
}

impl SvgPositioned for CapabilityNodeTree {
    fn get_bbox(&self) -> Rect {
        LAYOUT_DIRECTION.with(|it| it.set(Some(self.layout_direction)));
        let answer = self.tree.get_bbox();
        LAYOUT_DIRECTION.with(|it| it.set(None));
        answer
    }
}


impl Default for TreeCollapsePolicy {
    fn default() -> Self {
        TreeCollapsePolicy::Nothing
    }
}



#[allow(dead_code)]
fn dummy_data() -> CapabilityNodeTree {
    let core_0 = "Account Management";
    let core_0_0 = "Administer Accounts";
    let core_0_0_0 = "Enact a status on accounts";
    let core_0_0_1 = "Enact a status on transactions";
    let core_0_0_2 = "Link overdraft protection";
    let core_0_1 = "Capital One Legal Entity";
    let core_0_1_0 = "Assign DDA Account to Legal Entity";
    let core_0_1_1 = "Assign Loan Account to Legal Entity";
    let core_1 = "Account Servicing";
    let core_1_0 = "Perform Year End Processing";
    let core_1_1 = "Calculate Balances";
    let core_1_1_0 = "Aggregate available balance within customer";
    let core_1_1_1 = "Assign funds availability policy to accounts";
    let core_1_1_2 = "Maintain daily balances";

    let lob_usage = None;
    let mut id_source: usize = 1;
    let mut tree = CapabilityNodeTree::new(TreeLayoutDirection::Left, TreeCollapsePolicy::Nothing);
    let collapsed = false;
    tree.grow_tree([
        AddData(CapabilityNode::new(core_0, lob_usage, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityNode::new(core_0_0, lob_usage, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityNode::new(core_0_0_0, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_0_0_1, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_0_0_2, lob_usage, &mut id_source, collapsed)),
        EndChildren,
        AddData(CapabilityNode::new(core_0_1, lob_usage, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityNode::new(core_0_1_0, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_0_1_1, lob_usage, &mut id_source, collapsed)),
        EndChildren,
        EndChildren,
        AddData(CapabilityNode::new(core_1, lob_usage, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityNode::new(core_1_0, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_1_1, lob_usage, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityNode::new(core_1_1_0, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_1_1_1, lob_usage, &mut id_source, collapsed)),
        AddData(CapabilityNode::new(core_1_1_2, lob_usage, &mut id_source, collapsed)),
        EndChildren,
        EndChildren,
    ]).expect("The data insertion is unbalanced.");

    tree
}

/// Recursive function used in build_tidy_tree().
fn add_to_tidy(tidy: &mut TidyTree, dtnode: &DTNode<CapabilityNode>, parent_id: usize) {
    let data_bbox = dtnode.data.get_bbox();
    // note: width and height are swapped because we want to lay it out sideways not vertically
    tidy.add_node(dtnode.data.id, data_bbox.height(), data_bbox.width(), parent_id);
    for child in dtnode.children.iter() {
        add_to_tidy(tidy, child, dtnode.data.id);
    }
}

/// Recursive function used in build_tidy_tree().
fn populate_locations(dtnode: &mut DTNode<CapabilityNode>, locations: &HashMap<usize, (f64, f64)>) {
    match locations.get(&dtnode.data.id) {
        None => panic!("All locations should be set but aren't."),
        Some((x,y)) => dtnode.data.location = (*y, *x),
    }
    for child in dtnode.children.iter_mut() {
        populate_locations(child, locations);
    }
}


/// Sets the node_loc_style field in the entire tree.
fn set_node_loc_style(dtnode: &mut DTNode<CapabilityNode>) {
    dtnode.data.node_loc_style = NodeLocationStyle::RootNode;
    for child in dtnode.children.iter_mut() {
        set_node_loc_style_internal(child)
    }
}

/// Private internal subroutine of set_node_loc_style()
fn set_node_loc_style_internal(dtnode: &mut DTNode<CapabilityNode>) {
    if dtnode.children.is_empty() {
        dtnode.data.node_loc_style = NodeLocationStyle::LeafNode;
    } else {
        dtnode.data.node_loc_style = NodeLocationStyle::BranchNode;
    }
    for child in dtnode.children.iter_mut() {
        set_node_loc_style_internal(child)
    }
}


// FIXME: This panics if the format isn't as expected. Should be made more robust.
/// Returns the core tree and the surrounds tree
pub fn read_csv(input_filename: &str, fold_info: fold_up::FoldInfo) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    const LEVEL_COLS: [usize; 5] = [0,1,2,3,4];
    const FEATURE_PLACEMENT_COL: usize = 7;
    const LOB_USAGE_COLS: [usize;3] = [9,10,11];
    static EMPTY_STRING: String = String::new();

    // --- Variables we will track from row to row ---
    /// we'll track 3 fields for each tree we are building
    struct FieldsTrackedPerTree {
        commands: Vec<DTNodeBuild<CapabilityNode>>,
        id_source: usize,
        prev_node_names: Vec<String>, // entry for each branch node
        prev_item_text: String,
    }
    impl FieldsTrackedPerTree {
        fn new() -> Self {
            FieldsTrackedPerTree{commands: Vec::new(), id_source: 1, prev_node_names: Vec::new(), prev_item_text: String::new()}
        }
    }

    // --- Create two of them for the two trees ---
    let mut fields_core = FieldsTrackedPerTree::new();
    let mut fields_surround = FieldsTrackedPerTree::new();

    // --- Start reading the CSV ---
    let mut reader = csv::Reader::from_path(input_filename)?;
    for result in reader.records() {
        let record = result.unwrap();

        // --- get the lob_usage for this leaf ---
        let get_lob_usage = |i: usize| {
            let s = record.get(LOB_USAGE_COLS[i])
                .unwrap_or_else(|| panic!("Column {} missing for row {:?}", LOB_USAGE_COLS[0], record));
            match s {
                "Yes" => true,
                "Maybe" => true,
                "" => true,
                "No" => false,
                _ => panic!("Invalid LOB usage '{}' in row {:?}", s, record)
            }
        };
        let lob_usage_bools: [bool; 3] = [get_lob_usage(0), get_lob_usage(1), get_lob_usage(2)];
        let lob_usage = LobUsage::new(lob_usage_bools);

        // --- find which tree this leaf is on ---
        let feature_placement = record.get(FEATURE_PLACEMENT_COL).unwrap();
        let places_to_display = match feature_placement {
            "Core"     => vec![&mut fields_core],
            "Surround" => vec![&mut fields_surround],
            "Not Sure" => vec![&mut fields_core, &mut fields_surround],
            ""         => vec![&mut fields_core, &mut fields_surround],
            _ => panic!("Invalid feature placement of '{}' in row {:?}", feature_placement, record),
        };

        // --- Loop through the places we might display this (could be in core, surround, or both) ---
        for fields in places_to_display {

            // --- find the node_names ---
            let mut this_node_names: Vec<String> = (0..LEVEL_COLS.len())
                .map(|x| record.get(x).unwrap().to_string())
                .take_while(|x| x.len() > 0)
                .collect();
            let item_text = this_node_names.pop().unwrap(); // the last one isn't a node name, it's the item

            // --- close out nodes as needed ---
            let mut depth;
            if fields.prev_node_names.len() == 0 {
                depth = 0;
            } else {
                depth = fields.prev_node_names.len() - 1;
                loop {
                    let prev_name = fields.prev_node_names.get(depth).unwrap();
                    let this_name = this_node_names.get(depth).unwrap_or(&EMPTY_STRING);
                    if prev_name == this_name {
                        depth += 1;
                        break; // we can exit the loop; we found an identical ancestor node
                    } else {
                        fields.commands.push(EndChildren); // they're different, close out that depth
                    }
                    if depth == 0 {
                        break;
                    } else {
                        depth -= 1;
                    }
                }
            }

            // --- double-check that previous nodes are the same ---
            for deeper_depth in 0..depth {
                let prev_name = fields.prev_node_names.get(deeper_depth).unwrap();
                let this_name = this_node_names.get(deeper_depth).unwrap_or(&EMPTY_STRING);
                assert_eq!(prev_name, this_name);
            }

            // --- create new nodes as needed ---
            while depth < this_node_names.len() {
                let this_name = this_node_names.get(depth).unwrap();
                let collapsed = fold_info.is_fold_path(&this_node_names, depth + 1);
                fields.commands.push(AddData(CapabilityNode::new(this_name, None, &mut fields.id_source, collapsed)));
                fields.commands.push(StartChildren(collapsed));
                depth += 1;
            }

            // --- add THIS node ---
            let collapsed = false;
            fields.commands.push(AddData(CapabilityNode::new(&item_text, Some(lob_usage), &mut fields.id_source, collapsed)));

            // --- update prev_node_names ---
            fields.prev_node_names = this_node_names;
            fields.prev_item_text = item_text;
        }
    }

    // --- Create a tree from the commands ---
    let mut core_tree = CapabilityNodeTree::new(TreeLayoutDirection::Left, TreeCollapsePolicy::JavaScriptReplace);
    core_tree.grow_tree(fields_core.commands).expect("The data insertion is unbalanced for core.");
    let mut surround_tree = CapabilityNodeTree::new(TreeLayoutDirection::Right, TreeCollapsePolicy::JavaScriptReplace);
    surround_tree.grow_tree(fields_surround.commands).expect("The data insertion is unbalanced for surround.");

    // --- Return the result ---
    Ok([core_tree, surround_tree])
}
