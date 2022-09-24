mod trifoil;
mod used_by;
mod capability_tree;
mod capability_db;
mod center_dot;
mod document;
mod capability_html;
mod surrounds;
mod connecting_lines;


use calamine::Error;
use crate::capability_db::CapabilitiesDB;
use prog_draw::macos_text_size::MacOSTextSizer;

/// A main() that exists just for testing.
fn main() -> Result<(), Error> {
    println!("BEGIN");
    // --- Set the text sizer ---
    static MACOS_TEXT_SIZER: MacOSTextSizer = MacOSTextSizer;
    unsafe { // must be called before anything else happens, then never called again
        prog_draw::text_size::set_system_text_sizer(&MACOS_TEXT_SIZER);
    }

    // --- read in data and make document ---
    let db_or_err = capability_db::read_db(include_bytes!("../input/capabilities_db.xlsx"));
    let capdb: CapabilitiesDB = match db_or_err {
        Ok(capdb) => capdb,
        Err(err) => panic!("{}", err), // it's read at compile time, so handle errors with a panic.
    };
    let document = document::TwoTreeViewDocument::new(capdb);
    document.get_svg_str();

    // --- print it ---
    println!("END");
    Ok(())
}
// fn main() {}
