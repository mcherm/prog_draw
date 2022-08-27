
mod svg_render;
mod visualize_core;
mod text_size;
mod svg_writer;
mod tidy_tree;
mod data_tree;


fn main() {
    svg_render::main();

    visualize_core::visualize_core();
}
