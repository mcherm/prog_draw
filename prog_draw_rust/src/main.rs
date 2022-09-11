
mod svg_render;
mod visualize_core;
mod text_size;
mod macos_text_size;
mod svg_writer;
mod tidy_tree;
mod data_tree;

use text_size::set_system_text_sizer;

fn main() {
    svg_render::main();
    unsafe { // system initialization; must happen before anything else
        set_system_text_sizer(&macos_text_size::MacOSTextSizer);
    }

    visualize_core::visualize_core();
}
