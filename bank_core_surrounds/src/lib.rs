
mod trifoil;
mod fold_up;
mod used_by;
mod capability_tree;
mod capability_db;
mod center_dot;
mod document;
mod capability_html;



use std::sync::{Mutex, Once};
use std::borrow::BorrowMut;
use document::TwoTreeViewDocument;
use capability_tree::read_csv_from_bokor_str;
use prog_draw::text_size;
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
    fn text_size(&self, text: &str, font_family: &str, font_size: f32) -> Result<(f32, f32), text_size::TextSizeError> {
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
pub fn get_style() -> String {
    capability_html::style().into()
}

#[wasm_bindgen]
pub fn get_svg() -> String {
    match global_document().lock().unwrap().get_svg_str() {
        Ok(s) => s.into(),
        Err(_) => "<h1>Error</h1>".into(),
    }
}

#[wasm_bindgen]
pub fn toggle_node(node_id: String) -> String {
    global_document().lock().unwrap().toggle_collapse(node_id.as_str());
    get_svg()
}

#[wasm_bindgen]
pub fn show_node(node_id: String) -> String {
    capability_html::as_html(
        global_document().lock().unwrap().get_node_data(&node_id).unwrap()
    )
}

pub fn get_initial_document() -> TwoTreeViewDocument {
    // --- read the data ---
    let db_or_err = capability_db::read_db(include_bytes!("../input/capabilities_db.xlsx"));
    let capdb: capability_db::CapabilitiesDB = match db_or_err {
        Ok(capdb) => capdb,
        Err(err) => panic!("{}", err), // it's read at compile time, so handle errors with a panic.
    };

    // --- Create the document ---
    TwoTreeViewDocument::new(capdb)
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

//
// FIXME:
//   Folks on the Rust discord who helped me with another issue suggested that instead of doing
//   GLOBAL_DOCUMENT as a "static mut" I should use either once_cell::sync::Lazy or else
//   use "static GLOBAL_DOCUMENT: Mutex<Option<TwoTreeViewDocument>> = Mutex::new(None)"
//   |
//   I should try it
//
