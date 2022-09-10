mod trifoil;
mod fold_up;
mod lob_usage;
mod capability_tree;
mod center_dot;
mod document;

use std::fs::File;
use std;
use super::svg_writer::TagWriterError;
use capability_tree::read_csv_from_str;



const OUTPUT_FILENAME: &str = "output/core_surrounds.svg";



/// Build and render the view that shows two trees (for core and surrounds) somewhat folded
/// and neatly laid out with a key to the colors.
fn build_two_tree_view() -> Result<(),TagWriterError> {
    // --- Read in the file saying what to ignore due to folding ---
    let fold_info = fold_up::read_fold_info_from_str(include_str!("../../input/fold_up.csv"))?;

    // --- read the nodes ---
    let [core_tree, surround_tree] = read_csv_from_str(include_str!("../../input/core_surrounds.csv"), fold_info)?;

    // --- Create the document ---
    let two_tree_view = document::TwoTreeViewDocument::new(core_tree, surround_tree);

    // -- Output it ---
    let mut output: File = File::create(OUTPUT_FILENAME)?;
    two_tree_view.output_to(&mut output)?;
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
