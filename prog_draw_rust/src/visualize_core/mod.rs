mod trifoil;
mod fold_up;

use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;
use std;
use csv;
use super::data_tree::{
    DTNode,
    DTNodeBuild,
    DTNodeBuild::{AddData, StartChildren, EndChildren},
    InvalidGrowth
};
use super::tidy_tree::{TidyTree, NULL_ID};
use super::svg_writer::{Renderable, TagWriter, Attributes, TagWriterError, Context};
use super::svg_render::{Svg, Group, SvgPositioned};
use super::text_size::text_size;
use super::svg_render::geometry::{Coord, Rect};



const INPUT_FILENAME: &str = "input/core_surrounds.csv";
const FOLD_UP_FILENAME: &str = "input/fold_up.csv";
const OUTPUT_FILENAME: &str = "output/core_surrounds.svg";

const TEXT_ITEM_PADDING: Coord = 2.0;
const BASELINE_RISE: Coord = 2.0;
const NODE_ITEM_ROUND_CORNER: Coord = 3.0;
const CENTER_DOT_RADIUS: Coord = 16.0;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TreeLayoutDirection {
    Right,
    Left
}

#[derive(Debug, Eq, PartialEq)]
pub enum NodeLocationStyle {
    RootNode,
    BranchNode,
    LeafNode,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LobUsage {
    consumer: bool,
    sbb: bool,
    commercial: bool,
}

#[derive(Debug)]
pub struct MyNode {
    id: usize,
    text: String,
    lob_usage: Option<LobUsage>,
    location: (f64, f64),
    node_loc_style: NodeLocationStyle,
}

pub struct MyNodeTree {
    tree: DTNode<MyNode>,
    layout_direction: TreeLayoutDirection,
}




impl LobUsage {
    pub fn new(bools: [bool;3]) -> Self {
        LobUsage{consumer: bools[0], sbb: bools[1], commercial: bools[2]}
    }

    /// Return the (box_color, text_color) that should be used to render this.
    pub fn get_color_str(&self) -> (&'static str, &'static str) {
        match (self.commercial, self.sbb, self.consumer) {
            ( true,  true,  true) => ("#6C686C", "#FFFFFF"),
            ( true,  true, false) => ("#F58CFF", "#000000"),
            ( true, false,  true) => ("#FFC77F", "#000000"),
            ( true, false, false) => ("#FF6163", "#000000"),
            (false,  true,  true) => ("#80FF80", "#000000"),
            (false,  true, false) => ("#7F7FFF", "#000000"),
            (false, false,  true) => ("#FFFF7F", "#000000"),
            (false, false, false) => ("#FFFFFF", "#000000"),
        }
    }
}


impl MyNode {
    pub fn new(text: &str, lob_usage: Option<LobUsage>, id_source: &mut usize) -> Self {
        let id = *id_source;
        let text = text.to_string();
        let location = (0.0, 0.0); // default location until it gets repositioned
        let node_loc_style = NodeLocationStyle::BranchNode; // everything is assumed to be a branch until proven otherwise
        *id_source += 1;
        MyNode{id, text, lob_usage, location, node_loc_style}
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


impl Renderable for MyNode {
    fn render(&self, tag_writer: &mut TagWriter, context: &mut Context) -> Result<(), TagWriterError> {
        // --- Decide the dimensions of everything ---
        let (loc_x, loc_y) = self.location;
        let (text_width, text_height) = self.text_size();
        let text_left = loc_x + match *context.get("layout_direction").unwrap() {
            "Right" => TEXT_ITEM_PADDING,
            "Left" => (TEXT_ITEM_PADDING + text_width) * -1.0,
            _ => panic!("No layout direction set."),
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
            NodeLocationStyle::BranchNode => "branch",
            NodeLocationStyle::LeafNode => "leaf",
        };
        let (box_color, text_color) = match self.lob_usage {
            None => ("#E8E8E8", "#000000"),
            Some(lob_usage) => lob_usage.get_color_str(),
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
        }
        Ok(())
    }
}


impl SvgPositioned for MyNode {
    // Gives the bounding box for the node including text AND the box around it. Remember, if
    // the node isn't correctly positioned yet, its location will be (0,0). Also know that
    // self.location is the center-left of the box it occupies.
    fn get_bbox(&self, context: &mut Context) -> Rect {
        let center = self.location;
        let (text_width, text_height) = self.text_size();
        let width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let height = text_height + 2.0 * TEXT_ITEM_PADDING;
        let left = center.0 - match *context.get("layout_direction").unwrap() {
            "Right" => 0.0,
            "Left" => width,
            _ => panic!("Invalid layout direction"),
        };
        let top = center.1 - height / 2.0;
        Rect::new_ltwh(left, top, width, height)
    }
}


impl MyNodeTree {
    pub fn new(layout_direction: TreeLayoutDirection) -> Self {
        let mut id_source = 0;
        let tree = DTNode::new(MyNode::new("", None, &mut id_source));
        MyNodeTree{tree, layout_direction}
    }

    /// Adds nodes to the tree.
    pub fn grow_tree(&mut self, items: impl IntoIterator<Item=DTNodeBuild<MyNode>>) -> Result<(),InvalidGrowth> {
        self.tree.grow_tree(items)
    }

    /// Performs layout of the nodes.
    pub fn layout(&mut self) {
        // --- set up a context ---
        let mut context = Context::default();
        context.insert("layout_direction".to_string(), match self.layout_direction {
            TreeLayoutDirection::Right => &"Right",
            TreeLayoutDirection::Left => &"Left",
        });

        // --- use tidy-tree to lay it out ---
        let mut tidy = TidyTree::with_tidy_layout(16.0, 8.0);
        add_to_tidy(&mut tidy, &self.tree, NULL_ID, &mut context);
        tidy.layout();
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

impl Renderable for MyNodeTree {
    fn render(&self, tag_writer: &mut TagWriter, context: &mut Context) -> Result<(), TagWriterError> {
        tag_writer.begin_tag("g", Attributes::new())?;
        // FIXME: I used to have style here; don't need it now.
        let style_text = r#"
        "#;
        tag_writer.tag_with_text("style", Attributes::new(), style_text)?;
        context.insert("layout_direction".to_string(), match self.layout_direction {
            TreeLayoutDirection::Right => &"Right",
            TreeLayoutDirection::Left => &"Left",
        });
        self.tree.render(tag_writer, context)?;
        context.remove("layout_direction");
        tag_writer.end_tag("g")?;
        Ok(())
    }
}

impl SvgPositioned for MyNodeTree {
    fn get_bbox(&self, context: &mut Context) -> Rect {
        context.insert("layout_direction".to_string(), match self.layout_direction {
            TreeLayoutDirection::Right => &"Right",
            TreeLayoutDirection::Left => &"Left",
        });
        let answer = self.tree.get_bbox(context);
        context.remove("layout_direction");
        answer
    }
}



#[allow(dead_code)]
fn dummy_data() -> MyNodeTree {
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
    let mut my_tree = MyNodeTree::new(TreeLayoutDirection::Left);
    my_tree.grow_tree([
        AddData(MyNode::new(core_0, lob_usage, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_0, lob_usage, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_0_0, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_0_0_1, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_0_0_2, lob_usage, &mut id_source)),
        EndChildren,
        AddData(MyNode::new(core_0_1, lob_usage, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_1_0, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_0_1_1, lob_usage, &mut id_source)),
        EndChildren,
        EndChildren,
        AddData(MyNode::new(core_1, lob_usage, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_1_0, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_1_1, lob_usage, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_1_1_0, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_1_1_1, lob_usage, &mut id_source)),
        AddData(MyNode::new(core_1_1_2, lob_usage, &mut id_source)),
        EndChildren,
        EndChildren,
    ]).expect("The data insertion is unbalanced.");

    my_tree
}

/// Recursive function used in build_tidy_tree().
fn add_to_tidy(tidy: &mut TidyTree, dtnode: &DTNode<MyNode>, parent_id: usize, context: &mut Context) {
    let data_bbox = dtnode.data.get_bbox(context);
    // note: width and height are swapped because we want to lay it out sideways not vertically
    tidy.add_node(dtnode.data.id, data_bbox.height(), data_bbox.width(), parent_id);
    for child in dtnode.children.iter() {
        add_to_tidy(tidy, child, dtnode.data.id, context);
    }
}

/// Recursive function used in build_tidy_tree().
fn populate_locations(dtnode: &mut DTNode<MyNode>, locations: &HashMap<usize, (f64, f64)>) {
    match locations.get(&dtnode.data.id) {
        None => panic!("All locations should be set but aren't."),
        Some((x,y)) => dtnode.data.location = (*y, *x),
    }
    for child in dtnode.children.iter_mut() {
        populate_locations(child, locations);
    }
}


/// Sets the node_loc_style field in the entire tree.
fn set_node_loc_style(dtnode: &mut DTNode<MyNode>) {
    dtnode.data.node_loc_style = NodeLocationStyle::RootNode;
    for child in dtnode.children.iter_mut() {
        set_node_loc_style_internal(child)
    }
}

/// Private internal subroutine of set_node_loc_style()
fn set_node_loc_style_internal(dtnode: &mut DTNode<MyNode>) {
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
fn read_csv(input_filename: &str, fold_info: fold_up::FoldInfo) -> Result<[MyNodeTree; 2], std::io::Error> {
    const LEVEL_COLS: [usize; 5] = [0,1,2,3,4];
    const FEATURE_PLACEMENT_COL: usize = 7;
    const LOB_USAGE_COLS: [usize;3] = [9,10,11];
    static EMPTY_STRING: String = String::new();

    // --- Variables we will track from row to row ---
    /// we'll track 3 fields for each tree we are building
    struct FieldsTrackedPerTree {
        commands: Vec<DTNodeBuild<MyNode>>,
        id_source: usize,
        prev_node_names: Vec<String>, // entry for each branch node
    }
    impl FieldsTrackedPerTree {
        fn new() -> Self {
            FieldsTrackedPerTree{commands: Vec::new(), id_source: 1, prev_node_names: Vec::new()}
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
            let mut item_text = this_node_names.pop().unwrap(); // the last one isn't a node name, it's the item

            // --- If part of it is folded, adjust accordingly ---
            if let Some(fold_position) = fold_info.get_fold_position(&this_node_names) {
                assert!(fold_position < this_node_names.len()); // should always be true
                item_text = this_node_names.get(fold_position).unwrap().clone();
                this_node_names.truncate(fold_position);
                // FIXME: Later I need to make sure I don't add the same item multiple times. But only after I merge branches
            }

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
                fields.commands.push(AddData(MyNode::new(this_name, None, &mut fields.id_source)));
                fields.commands.push(StartChildren);
                depth += 1;
            }

            // --- add THIS node ---
            fields.commands.push(AddData(MyNode::new(&item_text, Some(lob_usage), &mut fields.id_source)));

            // --- update prev_node_names ---
            fields.prev_node_names = this_node_names;
        }
    }

    // --- Create a tree from the commands ---
    let mut core_tree = MyNodeTree::new(TreeLayoutDirection::Left);
    core_tree.grow_tree(fields_core.commands).expect("The data insertion is unbalanced for core.");
    let mut surround_tree = MyNodeTree::new(TreeLayoutDirection::Right);
    surround_tree.grow_tree(fields_surround.commands).expect("The data insertion is unbalanced for surround.");

    // --- Return the result ---
    Ok([core_tree, surround_tree])
}


struct CenterDot;

impl Renderable for CenterDot {
    fn render(&self, tag_writer: &mut TagWriter, _context: &mut Context) -> Result<(), TagWriterError> {
        tag_writer.single_tag("circle", Attributes::from([
            ("cx", "0"),
            ("cy", "0"),
            ("r", &*CENTER_DOT_RADIUS.to_string()),
        ]))?;
        Ok(())
    }
}

impl SvgPositioned for CenterDot {
    fn get_bbox(&self, _context: &mut Context) -> Rect {
        Rect::new_cwh((0.0,0.0), 2.0 * CENTER_DOT_RADIUS, 2.0 * CENTER_DOT_RADIUS)
    }
}


/// Function that places all the pieces that are highly specific to the diagram we
/// are building.
fn layout_this_diagram(core_tree: MyNodeTree, surround_tree: MyNodeTree) -> Svg<Group> {
    let shift_dist = CENTER_DOT_RADIUS - 2.0 * TEXT_ITEM_PADDING;

    let core_tree_group = Group::item_transformed(Box::new(core_tree), &format!("translate({},0)", shift_dist * -1.0));
    let surround_tree_group = Group::item_transformed(Box::new(surround_tree), &format!("translate({},0)", shift_dist));
    let trifoil_group = Group::item_transformed(Box::new(trifoil::Trifoil), "translate(0 -250) scale(0.5)");

    let content: [Box<dyn SvgPositioned>; 4] = [
        Box::new(trifoil_group),
        Box::new(core_tree_group),
        Box::new(surround_tree_group),
        Box::new(CenterDot),
    ];
    Svg::new(Group::from(content))
}


fn build_tidy_tree() -> Result<(),TagWriterError> {
    // --- Read in the file saying what to ignore due to folding ---
    let fold_info = fold_up::read_fold_info(FOLD_UP_FILENAME)?;

    // --- read the nodes ---
    let [mut core_tree, mut surround_tree] = read_csv(INPUT_FILENAME, fold_info)?;

    // --- perform layout ---
    core_tree.layout();
    surround_tree.layout();

    // -- Output it ---
    let svg = layout_this_diagram(core_tree, surround_tree);
    let output: File = File::create(OUTPUT_FILENAME)?;
    let mut tag_writer = TagWriter::new(output);
    svg.render(&mut tag_writer, &mut Context::default())?;
    tag_writer.close()?;
    Ok(())
}


/// This is the main entry point of the visualize_core functionality.
/// When run, it outputs some hardcoded data to output/core_surrounds.svg
pub fn visualize_core() {
    match build_tidy_tree() {
        Ok(_) => {},
        Err(e) => println!("ERROR: {}", e),
    }
}
