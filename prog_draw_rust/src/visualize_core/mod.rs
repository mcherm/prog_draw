mod trifoil;
mod fold_up;
mod lob_usage;
mod capability_tree;
mod center_dot;
mod document;

#[allow(unused_imports)]
use std::fs::File;
use std;
use capability_tree::read_csv_from_str;
pub use document::TwoTreeViewDocument;


#[allow(dead_code)]
const OUTPUT_FILENAME: &str = "output/core_surrounds.svg";


pub fn get_two_tree_view() -> Result<TwoTreeViewDocument,std::io::Error> {
    // --- Read in the file saying what to ignore due to folding ---
    let fold_info = fold_up::read_fold_info_from_str(include_str!("../../input/fold_up.csv"))?;

    // --- read the nodes ---
    let [core_tree, surround_tree] = read_csv_from_str(include_str!("../../input/core_surrounds.csv"), fold_info)?;

    // --- Create the document ---
    let answer = Ok(TwoTreeViewDocument::new(core_tree, surround_tree));
    answer
}


/// This is the main entry point of the visualize_core functionality.
/// When run, it outputs some hardcoded data to OUTPUT_FILENAME
#[allow(dead_code)]
pub fn visualize_core() {
    let two_tree_view = match get_two_tree_view() {
        Ok(x) => x,
        Err(x) => panic!("Error getting document: {}", x),
    };
    let mut output: File = match File::create(OUTPUT_FILENAME) {
        Ok(x) => x,
        Err(x) => panic!("Error opening output file: {}", x),
    };
    match  two_tree_view.output_to(&mut output) {
        Ok(_) => {},
        Err(x) => panic!("Error writing to output file: {}", x),
    }
}
