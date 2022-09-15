pub mod geometry;
pub mod data_tree;
pub mod tidy_tree;
pub mod svg_writer;
pub mod svg_render;
pub mod text_size;
pub mod macos_text_size;


// NOTES: IF there were a main() it might look like this:
// fn main() {
//     unsafe { // system initialization; must happen before anything else
//         text_size::set_system_text_sizer(&macos_text_size::MacOSTextSizer);
//     }
//
//     visualize_core::visualize_core();
// }
