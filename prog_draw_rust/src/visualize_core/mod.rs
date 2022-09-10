mod trifoil;
mod fold_up;
mod lob_usage;
mod capability_tree;
mod center_dot;

use std::fs::File;
use std;
use super::svg_writer::{TagWriter, TagWriterError};
use super::svg_render::{Group, Svg, SvgPositioned};
use super::svg_render::geometry::{Coord};
use capability_tree::{CapabilityNodeTree, read_csv};
use center_dot::CenterDot;



const INPUT_FILENAME: &str = "input/core_surrounds.csv";
const FOLD_UP_FILENAME: &str = "input/fold_up.csv";
const OUTPUT_FILENAME: &str = "output/core_surrounds.svg";

const TEXT_ITEM_PADDING: Coord = 2.0;
const BASELINE_RISE: Coord = 2.0;
const NODE_ITEM_ROUND_CORNER: Coord = 3.0;
const CENTER_DOT_RADIUS: Coord = 16.0;


/// Function that places all the pieces that are highly specific to the diagram we
/// are building.
fn layout_this_diagram(core_tree: CapabilityNodeTree, surround_tree: CapabilityNodeTree) -> Svg<Group> {
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


/// Build and render the view that shows two trees (for core and surrounds) somewhat folded
/// and neatly laid out with a key to the colors.
fn build_two_tree_view() -> Result<(),TagWriterError> {
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
    svg.render(&mut tag_writer)?;
    tag_writer.close()?;
    Ok(())
}


/// This is the main entry point of the visualize_core functionality.
/// When run, it outputs some hardcoded data to output/core_surrounds.svg
pub fn visualize_core() {
    match build_two_tree_view() {
        Ok(_) => {},
        Err(e) => println!("ERROR: {}", e),
    }
}
