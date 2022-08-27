

const INPUT_FILENAME: &str = "input/core_surrounds.csv";
const OUTPUT_FILENAME: &str = "output/core_surrounds.svg";


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
use super::svg_writer::{Renderable, TagWriter, Attributes, TagWriterError};
use super::svg_render::{Svg, SvgPositioned};
use super::text_size::text_size;
use super::svg_render::geometry::{Coord, Rect};


static TEXT_ITEM_PADDING: Coord = 2.0;
static BASELINE_RISE: Coord = 2.0;
static NODE_ITEM_ROUND_CORNER: Coord = 3.0;


#[derive(Debug, Eq, PartialEq)]
enum NodeLocationStyle {
    RootNode,
    BranchNode,
    LeafNode,
}

#[derive(Debug)]
struct MyNode {
    id: usize,
    text: String,
    location: (f64, f64),
    node_loc_style: NodeLocationStyle,
}


struct MyNodeTree {
    tree: DTNode<MyNode>,
}





impl MyNode {
    pub fn new(text: &str, id_source: &mut usize) -> Self {
        let id = *id_source;
        let text = text.to_string();
        let location = (0.0, 0.0); // default location until it gets repositioned
        let node_loc_style = NodeLocationStyle::BranchNode; // everything is assumed to be a branch until proven otherwise
        *id_source += 1;
        MyNode{id, text, location, node_loc_style}
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
    fn render(&self, tag_writer: &mut TagWriter) -> Result<(), TagWriterError> {
        let class: &str = match self.node_loc_style {
            NodeLocationStyle::RootNode => "root",
            NodeLocationStyle::BranchNode => "branch",
            NodeLocationStyle::LeafNode => "leaf",
        };

        let (loc_x, loc_y) = self.location;
        let (text_width, text_height) = self.text_size();
        let text_left = loc_x + TEXT_ITEM_PADDING;
        let text_top = loc_y - text_height / 2.0;
        let text_baseline = text_top + text_height - BASELINE_RISE;
        let box_left = text_left - TEXT_ITEM_PADDING;
        let box_top = text_top - TEXT_ITEM_PADDING;
        let box_width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let box_height = text_height + 2.0 * TEXT_ITEM_PADDING;

        if self.node_loc_style != NodeLocationStyle::RootNode {
            tag_writer.single_tag("rect", Attributes::from([
                ("x", &*box_left.to_string()),
                ("y", &*box_top.to_string()),
                ("width", &*box_width.to_string()),
                ("height", &*box_height.to_string()),
                ("rx", &*NODE_ITEM_ROUND_CORNER.to_string()),
                ("fill", "none"),
                ("stroke", "black"),
                ("stroke-width", &*1.to_string()),
                ("class", class)
            ]))?;
            tag_writer.tag_with_text(
                "text",
                Attributes::from([
                    ("x", &*text_left.to_string()),
                    ("y", &*text_baseline.to_string()),
                    ("style", "font-family; 'Arial'; font-style: normal; font-size: 14px"),
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
    fn get_bbox(&self) -> Rect {
        let center = self.location;
        let (text_width, text_height) = self.text_size();
        let width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let height = text_height + 2.0 * TEXT_ITEM_PADDING;
        let left = center.0;
        let top = center.1 - height / 2.0;
        Rect::new_ltwh(left, top, width, height)
    }
}


impl MyNodeTree {
    pub fn new() -> Self {
        let mut id_source = 0;
        let tree = DTNode::new(MyNode::new("", &mut id_source));
        MyNodeTree{tree}
    }

    /// Adds nodes to the tree.
    pub fn grow_tree(&mut self, items: impl IntoIterator<Item=DTNodeBuild<MyNode>>) -> Result<(),InvalidGrowth> {
        self.tree.grow_tree(items)
    }

    /// Performs layout of the nodes.
    pub fn layout(&mut self) {
        let mut tidy = TidyTree::with_tidy_layout(12.0, 12.0);
        add_to_tidy(&mut tidy, &self.tree, NULL_ID);
        tidy.layout();
        let locations: HashMap<usize, (f64, f64)> = tidy.get_pos().iter()
            .tuples::<(_,_,_)>() // break into groups of 3
            .map(|(id,x,y)| (*id as usize, (*x, *y))) // convert to ID and (x,y)
            .collect(); // and collect into a hashmap

        // set the location field in each one.
        populate_locations(&mut self.tree, &locations);

        // set the node_loc_style
        set_node_loc_style(&mut self.tree);
    }
}

impl Renderable for MyNodeTree {
    fn render(&self, tag_writer: &mut TagWriter) -> Result<(), TagWriterError> {
        tag_writer.begin_tag("g", Attributes::new())?;
        let style_text = r#"
            rect.branch {
                fill: #e0e0e0;
            }
            text.branch {
                fill: #606060;
            }
        "#;
        tag_writer.tag_with_text("style", Attributes::new(), style_text)?;
        self.tree.render(tag_writer)?;
        tag_writer.end_tag("g")?;
        Ok(())
    }
}

impl SvgPositioned for MyNodeTree {
    fn get_bbox(&self) -> Rect {
        self.tree.get_bbox()
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

    let mut id_source: usize = 1;
    let mut my_tree = MyNodeTree::new();
    my_tree.grow_tree([
        AddData(MyNode::new(core_0, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_0, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_0_0, &mut id_source)),
        AddData(MyNode::new(core_0_0_1, &mut id_source)),
        AddData(MyNode::new(core_0_0_2, &mut id_source)),
        EndChildren,
        AddData(MyNode::new(core_0_1, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_0_1_0, &mut id_source)),
        AddData(MyNode::new(core_0_1_1, &mut id_source)),
        EndChildren,
        EndChildren,
        AddData(MyNode::new(core_1, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_1_0, &mut id_source)),
        AddData(MyNode::new(core_1_1, &mut id_source)),
        StartChildren,
        AddData(MyNode::new(core_1_1_0, &mut id_source)),
        AddData(MyNode::new(core_1_1_1, &mut id_source)),
        AddData(MyNode::new(core_1_1_2, &mut id_source)),
        EndChildren,
        EndChildren,
    ]).expect("The data insertion is unbalanced.");

    my_tree
}

/// Recursive function used in build_tidy_tree().
fn add_to_tidy(tidy: &mut TidyTree, dtnode: &DTNode<MyNode>, parent_id: usize) {
    let data_bbox = dtnode.data.get_bbox();
    // note: width and height are swapped because we want to lay it out sideways not vertically
    tidy.add_node(dtnode.data.id, data_bbox.height(), data_bbox.width(), parent_id);
    for child in dtnode.children.iter() {
        add_to_tidy(tidy, child, dtnode.data.id);
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


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum FeaturePlacement {
    Core,
    Surround,
}

// FIXME: This panics if the format isn't as expected. Should be made more robust.
fn read_csv(input_filename: &str) -> Result<MyNodeTree, std::io::Error> {
    static NUM_LEVEL_COLS: usize = 5;
    static NAME_COL: usize = 5;
    static FEATURE_PLACEMENT_COL: usize = 6;
    static EMPTY_STRING: String = String::new();

    // --- Variables we will track from row to row ---
    let mut commands: Vec<DTNodeBuild<MyNode>> = Vec::new();
    let mut id_source = 1;
    let mut prev_node_names: Vec<String> = Vec::new(); // entry for each branch node

    // --- Start reading the CSV ---
    let mut reader = csv::Reader::from_path(input_filename)?;
    for result in reader.records() {
        let record = result.unwrap();

        // --- get the name of this leaf ---
        let text = record.get(NAME_COL).unwrap();

        // --- find which tree this leaf is on ---
        let feature_placement = match record.get(FEATURE_PLACEMENT_COL).unwrap() {
            "Core" => FeaturePlacement::Core,
            "Surround" => FeaturePlacement::Surround,
            _ => panic!("Invalid feature placement"),
        };
        assert_eq!(feature_placement, FeaturePlacement::Core); // FIXME: Only one implemented for now

        // --- find the node_names ---
        let this_node_names: Vec<String> = (0..NUM_LEVEL_COLS)
            .map(|x| record.get(x).unwrap().to_string())
            .take_while(|x| x.len() > 0)
            .collect();

        // --- close out nodes as needed ---
        let mut depth;
        if prev_node_names.len() == 0 {
            depth = 0;
        } else {
            depth = prev_node_names.len() - 1;
            loop {
                let prev_name = prev_node_names.get(depth).unwrap();
                let this_name = this_node_names.get(depth).unwrap_or(&EMPTY_STRING);
                if prev_name == this_name {
                    depth += 1;
                    break; // we can exit the loop; we found an identical ancestor node
                } else {
                    commands.push(EndChildren); // they're different, close out that depth
                }
                if depth == 0 {
                    break;
                } else {
                    depth -= 1;
                }
            }
        }

        // --- create new nodes as needed ---
        while depth < this_node_names.len() {
            let this_name = this_node_names.get(depth).unwrap();
            commands.push(AddData(MyNode::new(this_name, &mut id_source)));
            commands.push(StartChildren);
            depth += 1;
        }

        // --- add THIS node ---
        commands.push(AddData(MyNode::new(text, &mut id_source)));

        // --- update prev_node_names ---
        prev_node_names = this_node_names;
    }

    // --- Create a tree from the commands ---
    let mut my_tree = MyNodeTree::new();
    my_tree.grow_tree(commands).expect("The data insertion is unbalanced.");

    // --- Return the result ---
    Ok(my_tree)
}


fn build_tidy_tree() -> Result<(),TagWriterError> {
    let mut my_tree = read_csv(INPUT_FILENAME)?;

    my_tree.layout();

    // Output it
    let svg = Svg::new(my_tree);
    let output: File = File::create(OUTPUT_FILENAME)?;
    let mut tag_writer = TagWriter::new(output);
    svg.render(&mut tag_writer)?;
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
