
mod geometry;
mod svg_render;
mod visualize_core;
mod svg_writer;
mod tidy_tree;
mod data_tree;
mod text_size;


use std::sync::{Mutex, Once};
use std::borrow::BorrowMut;
use visualize_core::TwoTreeViewDocument;
use crate::text_size::TextSizeError;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    pub fn log(s: &str);
    pub fn get_text_width(s: &str, font: &str) -> f32;
    pub fn get_text_height(s: &str, font: &str) -> f32;
}

#[derive(Debug)]
pub struct FontError;


struct WASMTextSizer;

impl text_size::TextSizer for WASMTextSizer {
    fn text_size(&self, text: &str, font_family: &str, font_size: f32) -> Result<(f32, f32), TextSizeError> {
        let font_str = format!("{}px {}", font_size, font_family);
        let x = get_text_width(text, &font_str);
        let y = get_text_height(text, &font_str);
        Ok((x,y))
    }
}



/// This must be called first, to initialize things in the rust world.
#[wasm_bindgen]
pub fn initialize() {
    unsafe { // must be called before anything else happens, then never called again
        text_size::set_system_text_sizer(&WASMTextSizer);
    }
}


#[wasm_bindgen]
pub fn get_svg() -> String {
    match global_document().lock().unwrap().get_svg_str() {
        Ok(s) => s.into(),
        Err(_) => "<h1>Error</h1>".into(),
    }
}

#[wasm_bindgen]
pub fn toggle_node(node_id: u32) -> String {
    global_document().lock().unwrap().toggle_collapse(node_id.try_into().unwrap());
    get_svg()
}


fn get_initial_document() -> TwoTreeViewDocument {
    match visualize_core::get_two_tree_view() {
        Ok(doc) => doc,
        Err(_) => panic!("Invalid document"),
    }
}


static mut GLOBAL_DOCUMENT: Option<Mutex<TwoTreeViewDocument>> = None;
static INIT_GLOBAL_DOCUMENT: Once = Once::new();


fn global_document<'a>() -> &'a Mutex<TwoTreeViewDocument> {
    INIT_GLOBAL_DOCUMENT.call_once(||{
        // This is safe; see https://www.sitepoint.com/rust-global-variables/#multithreadedglobalswithruntimeinitialization
        unsafe {
            *GLOBAL_DOCUMENT.borrow_mut() = Some(Mutex::new(get_initial_document()))
        }
    });
    unsafe { GLOBAL_DOCUMENT.as_ref().unwrap() }
}
