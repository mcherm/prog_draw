
use std::collections::HashMap;
use std::cell::Cell;
use itertools::Itertools;
use prog_draw::data_tree::{
    DTNode, DTNodeBuild, InvalidGrowth,TreeLayoutDirection, LAYOUT_DIRECTION,
    DTNodeBuild::{AddData, EndChildren, StartChildren},
};
use prog_draw::svg_render::SvgPositioned;
use prog_draw::geometry::{Coord, Rect};
use prog_draw::svg_writer::{Attributes, Renderable, TagWriter, TagWriterError};
use prog_draw::text_size::get_system_text_sizer;
use prog_draw::tidy_tree::{NULL_ID, TidyTree};
use crate::used_by::{UsedBySet, get_color_strs, UsedBy};
use crate::document::{
    BASELINE_RISE, COLLAPSE_DOT_RADIUS, NODE_ITEM_ROUND_CORNER,
    TEXT_ITEM_PADDING, LAYER_SPACING, ITEM_SPACING
};
use crate::fold_up;
use crate::capability_db::CapabilitiesDB;



#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum NodeLocationStyle {
    RootNode,
    BranchNode,
    LeafNode,
}


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum CoreOrSurround {
    Core,
    Surround,
    NotSure,
    Blank,
    Mixed,
}

#[derive(Debug, Clone)]
pub struct CapabilityData {
    pub id: String,
    pub parent_id: String,
    pub text: String,
    pub used_by_set: UsedBySet,
    pub description: String,
    pub core_surround: CoreOrSurround,
    pub notes: String,
    pub collapsed: bool,
    location: (f64, f64),
    node_loc_style: NodeLocationStyle,
}

#[derive(Debug)]
pub struct CapabilityNodeTree {
    pub tree: DTNode<CapabilityData>,
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



impl From<&str> for CoreOrSurround {
    /// Parse a CoreOrSurround string or panic if it's invalid.
    fn from(s: &str) -> Self {
        match s {
            "Core" => CoreOrSurround::Core,
            "Surround" => CoreOrSurround::Surround,
            "Not Sure" => CoreOrSurround::NotSure,
            "" => CoreOrSurround::Blank,
            "Mixed" => CoreOrSurround::Mixed,
            _ => panic!("Invalid CoreOrSurround: '{}'", s),
        }
    }
}

impl From<CoreOrSurround> for &'static str {
    fn from(x: CoreOrSurround) -> &'static str {
        match x {
            CoreOrSurround::Core => "Core",
            CoreOrSurround::Surround => "Surround",
            CoreOrSurround::NotSure => "Not Sure",
            CoreOrSurround::Blank => "",
            CoreOrSurround::Mixed => "Mixed",
        }
    }
}


impl Default for CoreOrSurround {
    // Default to blank (mostly so it works with reading Serde from excel).
    fn default() -> Self {
        return CoreOrSurround::Blank
    }
}


impl CapabilityData {
    fn new(
        id_str: String,
        parent_id: String,
        text: &str,
        used_by_set: UsedBySet,
        description: String,
        core_surround: CoreOrSurround,
        notes: String,
        collapsed: bool
    ) -> Self {
        let text = text.to_string();
        let location = (0.0, 0.0); // default location until it gets repositioned
        let node_loc_style = NodeLocationStyle::BranchNode; // everything is assumed to be a branch until proven otherwise
        CapabilityData {
            id: id_str, parent_id, text,
            used_by_set, description, core_surround,
            notes, collapsed, location, node_loc_style
        }
    }

    pub fn new_new(id: &str, parent_id: &str, text: &str, used_by_set: UsedBySet, description: &str, core_surround: CoreOrSurround, notes: &str, collapsed: bool) -> Self {
        CapabilityData::new(
            id.to_string(), parent_id.to_string(), text, used_by_set,
            description.to_string(), core_surround, notes.to_string(), collapsed
        )
    }

    pub fn new_generating_id(text: &str, used_by_set: UsedBySet, id_source: &mut usize, collapsed: bool) -> Self {
        let id = *id_source;
        *id_source += 1;
        let id_str = format!("XX{}", id);
        let parent_id = "XXX".to_string();
        let description = "".to_string();
        let core_surround = CoreOrSurround::Blank;
        let notes = "".to_string();
        Self::new(id_str, parent_id, text, used_by_set, description, core_surround, notes, collapsed)
    }

    /// Returns the (width, height) of the text string.
    fn text_size(&self) -> (Coord, Coord) {
        match get_system_text_sizer().text_size(&self.text, "Arial", 14.0) {
            Err(_) => panic!("Sizing isn't working."),
            Ok((width,height)) => (width as Coord, height as Coord)
        }
    }
}


impl Renderable for CapabilityData {
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
        let (box_color, text_color) = get_color_strs(&self.used_by_set);

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
                onclick: format!("toggle_then_draw('{}')", self.id).to_string(),
            }),
            (_, _) => None,
        };

        assert_eq!(box_width, self.get_bbox().width()); // FIXME: Remove... but this is useful.

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
                ("onclick", &format!("show_node_data('{}')", self.id)),
                ("class", class)
            ]))?;
            tag_writer.tag_with_text(
                "text",
                Attributes::from([
                    ("x", &*text_left.to_string()),
                    ("y", &*text_baseline.to_string()),
                    ("font-family", "Arial"),
                    ("fill", text_color),
                    ("style", "font-style: normal; font-size: 12.4px; pointer-events: none"), // FIXME: size for 14 and set this to 12.4 seems to work. WHY?
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


impl SvgPositioned for CapabilityData {
    /// Gives the bounding box for the node including text AND the box around it. Remember, if
    /// the node isn't correctly positioned yet, its location will be (0,0). Also know that
    /// self.location is the center-left or center-right of the box it occupies (depending
    /// on LAYOUT_DIRECTION).
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


/// Used internally, this just creates a mapping between strings and unique numbers.
/// which will last as long as the NumberMapper does.
struct NumberMapper {
    counter: usize,
    map: HashMap<String,usize> // FIXME: Can it use &str or &String?
}

impl NumberMapper {
    /// Create a NumberMapper
    fn new() -> NumberMapper {
        NumberMapper{counter: 0, map: HashMap::new()}
    }

    /// Get the numeric ID for a given string
    fn get_num(&mut self, s: &str) -> usize {
        match self.map.get(s) {
            Some(n) => *n,
            None => {
                let n = self.counter;
                self.counter += 1;
                self.map.insert(s.to_string(), n);
                n
            }
        }
    }

    /// Used to force a certain value. Caller must ensure the
    /// value is never reused.
    fn set(&mut self, s: &str, n: usize) {
        self.map.insert(s.to_string(), n);
    }
}


impl CapabilityNodeTree {
    pub fn new(layout_direction: TreeLayoutDirection, tree_collapse_policy: TreeCollapsePolicy) -> Self {
        let id = "ROOT";
        let parent_id = "";
        let text = "";
        let used_by_set = UsedBySet::all_mixed();
        let description = "";
        let core_surround = CoreOrSurround::Mixed;
        let notes = "";
        let collapsed = false;
        let data = CapabilityData::new_new(id, parent_id, text, used_by_set, description, core_surround, notes, collapsed);
        let tree = DTNode::new(data);
        CapabilityNodeTree {tree, layout_direction, tree_collapse_policy}
    }

    /// Adds nodes to the tree.
    pub fn grow_tree(&mut self, items: impl IntoIterator<Item=DTNodeBuild<CapabilityData>>) -> Result<(),InvalidGrowth> {
        self.tree.grow_tree(items)
    }


    pub fn find_data_by_id<'a>(&'a self, id: &str) -> Option<&'a CapabilityData> {
        let mut node_stack: Vec<&DTNode<CapabilityData>> = vec![&self.tree];
        while let Some(node) = node_stack.pop() {
            if node.data.id == id {
                return Some(&node.data)
            } else {
                for child in node.children.iter() {
                    node_stack.push(child)
                }
            }
        }
        return None
    }

    /// Given the ID of a node, this returns the tree node containing that item or None if it doesn't
    /// have one.
    ///
    /// FIXME: If I use this much I will need to make it efficient by maintaining a lookup table.
    pub fn find_node_by_id_mut<'a>(&'a mut self, id: &str) -> Option<&'a mut DTNode<CapabilityData>> {
        let mut node_stack: Vec<&'a mut DTNode<CapabilityData>> = vec![&mut self.tree];
        while let Some(node) = node_stack.pop() {
            if node.data.id == id {
                return Some(node)
            } else {
                for child in node.children.iter_mut() {
                    node_stack.push(child)
                }
            }
        }
        return None
    }

    /// Adds a node to the tree. Uses the node's parent_id to determine where to add it.
    /// If the node is incompatible in some fashion (its id is a repeat, its parent_id isn't
    /// in the tree, stuff like that) then it may panic.
    pub fn add_node(&mut self, data: CapabilityData) {
        match self.find_node_by_id_mut(&data.parent_id) {
            None => panic!("Cannot add node '{}' with parent id '{}' because the parent is not in the tree.", data.id, data.parent_id),
            Some(dt_node) => {
                dt_node.add_child_data(data)
            }
        }
    }

    /// Performs layout of the nodes.
    pub fn layout(&mut self) {
        // --- use tidy-tree to lay it out ---
        let mut nums = NumberMapper::new();
        nums.set("", NULL_ID);
        let mut tidy = TidyTree::with_tidy_layout(LAYER_SPACING, ITEM_SPACING);
        LAYOUT_DIRECTION.with(|it| it.set(Some(self.layout_direction)));
        add_to_tidy(&mut nums, &mut tidy, &self.tree, "");
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
        populate_locations(&mut nums, &mut self.tree, &locations);

        // set the node_loc_style
        set_node_loc_style(&mut self.tree);
    }

    /// Toggles the collapsed state of a node. Leaf and Root nodes are unaffected. Calling this
    /// with a node_id not found in the tree has no affect. Returns true if the tree needs to
    /// be laid out again after this, and false if it doesn't.
    ///
    /// FIXME: if DTNode someday grows a method for iterating over the tree I could use
    ///   that. Until then, it's done directly in this method.
    pub fn toggle_collapse(&mut self, node_id: &str) -> bool {
        /// Output of Internal recursive subroutine.
        enum Outcome {NotFound, FoundAndChanged, FoundNoChange}
        /// Internal recursive subroutine.
        fn toggle_collapse_node(node_id: &str, tree_node: &mut DTNode<CapabilityData>) -> Outcome {
            if &tree_node.data.id == node_id {
                match tree_node.data.node_loc_style {
                    NodeLocationStyle::BranchNode => {
                        tree_node.collapsed = ! tree_node.collapsed;
                        tree_node.data.collapsed = !tree_node.data.collapsed;
                        Outcome::FoundAndChanged
                    },
                    _ => Outcome::FoundNoChange,
                }
            } else {
                for child in tree_node.children.iter_mut() {
                    match toggle_collapse_node(node_id, child) {
                        Outcome::NotFound => {},
                        found_outcome => return found_outcome,
                    }
                }
                Outcome::NotFound
            }
        }

        // --- Call the recursive subroutine on the tree ---
        match toggle_collapse_node(node_id, &mut self.tree) {
            Outcome::NotFound => false,
            Outcome::FoundNoChange => false,
            Outcome::FoundAndChanged => true,
        }
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

    let used_by_set = Default::default();
    let mut id_source: usize = 1;
    let mut tree = CapabilityNodeTree::new(TreeLayoutDirection::Left, TreeCollapsePolicy::Nothing);
    let collapsed = false;
    tree.grow_tree([
        AddData(CapabilityData::new_generating_id(core_0, used_by_set, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityData::new_generating_id(core_0_0, used_by_set, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityData::new_generating_id(core_0_0_0, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_0_0_1, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_0_0_2, used_by_set, &mut id_source, collapsed)),
        EndChildren,
        AddData(CapabilityData::new_generating_id(core_0_1, used_by_set, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityData::new_generating_id(core_0_1_0, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_0_1_1, used_by_set, &mut id_source, collapsed)),
        EndChildren,
        EndChildren,
        AddData(CapabilityData::new_generating_id(core_1, used_by_set, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityData::new_generating_id(core_1_0, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_1_1, used_by_set, &mut id_source, collapsed)),
        StartChildren(false),
        AddData(CapabilityData::new_generating_id(core_1_1_0, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_1_1_1, used_by_set, &mut id_source, collapsed)),
        AddData(CapabilityData::new_generating_id(core_1_1_2, used_by_set, &mut id_source, collapsed)),
        EndChildren,
        EndChildren,
    ]).expect("The data insertion is unbalanced.");

    tree
}

/// Recursive function used in build_tidy_tree().
fn add_to_tidy(nums: &mut NumberMapper, tidy: &mut TidyTree, dtnode: &DTNode<CapabilityData>, parent_id: &str) {
    let data_bbox = dtnode.data.get_bbox();
    // note: width and height are swapped because we want to lay it out sideways not vertically
    tidy.add_node(nums.get_num(&dtnode.data.id), data_bbox.height(), data_bbox.width(), nums.get_num(parent_id));
    if !dtnode.collapsed {
        for child in dtnode.children.iter() {
            add_to_tidy(nums, tidy, child, &dtnode.data.id);
        }
    }
}

/// Recursive function used in build_tidy_tree().
fn populate_locations(nums: &mut NumberMapper, dtnode: &mut DTNode<CapabilityData>, locations: &HashMap<usize, (f64, f64)>) {
    match locations.get(&nums.get_num(&dtnode.data.id)) {
        None => panic!("All locations should be set but aren't."),
        Some((x,y)) => dtnode.data.location = (*y, *x),
    }
    if !dtnode.collapsed {
        for child in dtnode.children.iter_mut() {
            populate_locations(nums, child, locations);
        }
    }
}


/// Sets the node_loc_style field in the entire tree.
fn set_node_loc_style(dtnode: &mut DTNode<CapabilityData>) {
    dtnode.data.node_loc_style = NodeLocationStyle::RootNode;
    for child in dtnode.children.iter_mut() {
        set_node_loc_style_internal(child)
    }
}

/// Private internal subroutine of set_node_loc_style()
fn set_node_loc_style_internal(dtnode: &mut DTNode<CapabilityData>) {
    if dtnode.children.is_empty() {
        dtnode.data.node_loc_style = NodeLocationStyle::LeafNode;
    } else {
        dtnode.data.node_loc_style = NodeLocationStyle::BranchNode;
    }
    for child in dtnode.children.iter_mut() {
        set_node_loc_style_internal(child)
    }
}


/// Returns the core tree and the surrounds tree, from an input file that's been included at
/// compile time.
pub fn read_csv_from_bokor_str(data: &str, fold_info: fold_up::FoldInfo) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    let mut reader = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    read_csv_from_bokor_reader(&mut reader, fold_info)
}


/// Returns the core tree and the surrounds tree from data in the input file specified.
#[allow(dead_code)]
pub fn read_csv_from_bokor_file(input_filename: &str, fold_info: fold_up::FoldInfo) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    let mut reader = csv::Reader::from_path(input_filename)?;
    read_csv_from_bokor_reader(&mut reader, fold_info)
}


/// Returns the core tree and the surrounds tree
///
/// FIXME: This panics if the format isn't as expected. Should be made more robust.
pub fn read_csv_from_bokor_reader<R: std::io::Read>(reader: &mut csv::Reader<R>, fold_info: fold_up::FoldInfo) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    const LEVEL_COLS: [usize; 5] = [0,1,2,3,4];
    const FEATURE_PLACEMENT_COL: usize = 7;
    const LOB_USAGE_COLS: [usize;3] = [9,10,11];
    static EMPTY_STRING: String = String::new();

    // --- Variables we will track from row to row ---
    /// we'll track 3 fields for each tree we are building
    struct FieldsTrackedPerTree {
        commands: Vec<DTNodeBuild<CapabilityData>>,
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
    for result in reader.records() {
        let record = result.unwrap();

        // --- get the used_by_set for this leaf ---
        let usage_strs = [0,1,2]
            .map(|i| {
                record.get(LOB_USAGE_COLS[i])
                    .unwrap_or_else(|| panic!("Column {} missing for row {:?}", LOB_USAGE_COLS[0], record))
                    .into()
            });
        let used_by_set = UsedBySet::from_fields(usage_strs[0], usage_strs[1], usage_strs[2]);

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
                fields.commands.push(AddData(CapabilityData::new_generating_id(this_name, UsedBySet::all_mixed(), &mut fields.id_source, collapsed)));
                fields.commands.push(StartChildren(collapsed));
                depth += 1;
            }

            // --- add THIS node ---
            let collapsed = false;
            fields.commands.push(AddData(CapabilityData::new_generating_id(&item_text, used_by_set, &mut fields.id_source, collapsed)));

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



/// Returns the core tree and the surrounds tree, from an input file that's been included at
/// compile time.
pub fn read_csv_from_db_str(data: &str) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    let mut reader = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    read_csv_from_db_reader(&mut reader)
}


/// Returns the core tree and the surrounds tree
///
/// NOTE: This panics if the format isn't as expected. Probably OK since it is 'read'
/// at compile-time.
pub fn read_trees_from_capdb(capdb: &CapabilitiesDB) -> [CapabilityNodeTree; 2] {
    // --- Create two of them for the two trees ---
    let mut core_tree = CapabilityNodeTree::new(TreeLayoutDirection::Left, TreeCollapsePolicy::JavaScriptReplace);
    let mut surround_tree = CapabilityNodeTree::new(TreeLayoutDirection::Right, TreeCollapsePolicy::JavaScriptReplace);

    // --- Start reading the capabilities ---
    for row in capdb.capabilities.iter() {
        // --- Skip root ---
        if row.id == "ROOT" {
            continue;
        }

        // --- Create capability ---
        let used_by_set = UsedBySet::from_fields(row.used_by_consumer, row.used_by_sbb, row.used_by_commercial);
        let collapsed = false;
        let capability_data = CapabilityData::new_new(
            &row.id, &row.parent_id, &row.name, used_by_set, &row.description,
            row.core_surround, &row.notes, collapsed
        );

        // --- Add to one or both trees ---
        match capability_data.core_surround {
            CoreOrSurround::Core => {
                core_tree.add_node(capability_data);
            }
            CoreOrSurround::Surround => {
                surround_tree.add_node(capability_data);
            }
            CoreOrSurround::Blank | CoreOrSurround::NotSure | CoreOrSurround::Mixed => {
                core_tree.add_node(capability_data.clone());
                surround_tree.add_node(capability_data);
            }
        }
    }

    // --- Return the result ---
    [core_tree, surround_tree]
}


/// Returns the core tree and the surrounds tree
///
/// FIXME: This panics if the format isn't as expected. Should be made more robust.
pub fn read_csv_from_db_reader<R: std::io::Read>(reader: &mut csv::Reader<R>) -> Result<[CapabilityNodeTree; 2], std::io::Error> {
    // --- Create two of them for the two trees ---
    let mut core_tree = CapabilityNodeTree::new(TreeLayoutDirection::Left, TreeCollapsePolicy::JavaScriptReplace);
    let mut surround_tree = CapabilityNodeTree::new(TreeLayoutDirection::Right, TreeCollapsePolicy::JavaScriptReplace);

    // --- Start reading the CSV ---
    for result in reader.records() {
        let record = result.unwrap();

        let id = record.get(0).unwrap();
        let parent_id = record.get(1).unwrap();
        let name = record.get(2).unwrap();
        let _level = record.get(3).unwrap().parse::<usize>().unwrap();
        let _ = record.get(4).unwrap();
        let description = record.get(5).unwrap();
        let core_surround: CoreOrSurround = record.get(6).unwrap().into();
        let notes = record.get(7).unwrap();
        let used_by_consumer: UsedBy = record.get(8).unwrap().into();
        let used_by_sbb: UsedBy = record.get(9).unwrap().into();
        let used_by_commercial: UsedBy = record.get(10).unwrap().into();

        // --- Skip root ---
        if id == "ROOT" {
            continue;
        }

        // --- Create capability ---
        let used_by_set = UsedBySet::from_fields(used_by_consumer, used_by_sbb, used_by_commercial);
        let collapsed = false;
        let capability_data = CapabilityData::new_new(id, parent_id, name, used_by_set, description, core_surround, notes, collapsed);

        // --- Add to one or both trees ---
        match capability_data.core_surround {
            CoreOrSurround::Core => {
                core_tree.add_node(capability_data);
            }
            CoreOrSurround::Surround => {
                surround_tree.add_node(capability_data);
            }
            CoreOrSurround::Blank | CoreOrSurround::NotSure | CoreOrSurround::Mixed => {
                core_tree.add_node(capability_data.clone());
                surround_tree.add_node(capability_data);
            }
        }
    }

    // --- Return the result ---
    Ok([core_tree, surround_tree])
}
