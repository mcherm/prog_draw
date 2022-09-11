
mod svg_render;
mod visualize_core;
mod svg_writer;
mod tidy_tree;
mod data_tree;
mod text_size;


use std::sync::Mutex;
use wasm_bindgen::prelude::*;
use crate::text_size::TextSizeError;


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



fn get_svg_or_error() -> Result<String, svg_writer::TagWriterError> {
    log("entered get_svg_or_error()"); // FIXME: Remove
    let document = visualize_core::get_two_tree_view()?;
    log("got document"); // FIXME: Remove
    let answer = document.get_svg_str()?;
    Ok(answer)
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
    log("Testing text sizing");
    log(format!("'the quick brown fox' in '12.4px Arial' is {} high and {} long",
        get_text_height("the quick brown fox", "12.4px Arial"),
        get_text_width("the quick brown fox", "12.4px Arial")
    ).as_str());
    match get_svg_or_error() {
        Ok(s) => s.into(),
        Err(_) => "<h1>Error</h1>".into(),
    }
}

#[wasm_bindgen]
pub fn toggle_node(node_id: u32) -> String {
    alert(&format!("Toggle node {}", node_id));
    let old_state = match GLOBAL_STATE.lock().unwrap().pop() {
        None => false,
        Some(b) => b,
    };
    let new_state = !old_state;
    GLOBAL_STATE.lock().unwrap().push(new_state);
    if new_state {
        alert("orig");
        SAMPLE_SVG.into()
    } else {
        alert("alt");
        SAMPLE_SVG_ALT.into()
    }
}

static GLOBAL_STATE: Mutex<Vec<bool>> = Mutex::new(Vec::new());


static SAMPLE_SVG: &str = r##"
<svg viewBox="-532 -146 1064 294" xmlns="http://www.w3.org/2000/svg">
  <g>
    <g transform="translate(0 -250) scale(0.5)">

<g>
  <g>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" fill="#ffc77f"/>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" fill="#ffff7f"/>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" fill="#804000"/>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" fill="#80ff80"/>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" fill="#f58cff"/>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" fill="#8080ff"/>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" fill="#ff6163"/>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <text font-family="Arial" font-size="18" fill="#000000" x="-47.153914" y="-79.83107">Commercial</text>
  <text x="-131.024" y="62.162277" font-family="Arial" font-size="16" fill="#000000">Consumer</text>
  <text font-family="Arial" font-size="16" fill="#000000" x="75.5" y="65.5">SBB</text>
  <text font-family="Arial" font-size="16" fill="#FFFFFF" x="-9.362914" y="17">All</text>
</g>
    </g>
    <g transform="translate(-12,0)">
      <g>
        <style>
          text.leaf {
            pointer-events: none;
          }x
        </style>
        <path d="M -4 0 C -14 0, -10 -48.75, -20 -48.75" fill="none" stroke="black"/>
        <path d="M -4 0 C -14 0, -10 48.75, -20 48.75" fill="none" stroke="black"/>
        <path d="M -146 -48.75 C -156 -48.75, -152 -81.25, -162 -81.25" fill="none" stroke="black"/>
        <path d="M -146 -48.75 C -156 -48.75, -152 -16.25, -162 -16.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -107.25, -296 -107.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -81.25, -296 -81.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -55.25, -296 -55.25" fill="none" stroke="black"/>
        <rect x="-451" y="-116.25" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-449" y="-102.25" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="-468" y="-90.25" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-466" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="-435" y="-64.25" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-433" y="-50.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="-280" y="-90.25" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-278" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <circle cx="-280" cy="-81.25" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(2)"/>
        <rect x="-302" y="-25.25" width="140" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-300" y="-11.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Capital One Legal Entity</text>
        <circle cx="-302" cy="-16.25" r="3" fill="#000000" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(6)"/>
        <rect x="-146" y="-57.75" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-144" y="-43.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <circle cx="-146" cy="-48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(1)"/>
        <path d="M -125 48.75 C -135 48.75, -131 22.75, -141 22.75" fill="none" stroke="black"/>
        <path d="M -125 48.75 C -135 48.75, -131 74.75, -141 74.75" fill="none" stroke="black"/>
        <rect x="-310" y="13.75" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-308" y="27.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M -253 74.75 C -263 74.75, -259 48.75, -269 48.75" fill="none" stroke="black"/>
        <path d="M -253 74.75 C -263 74.75, -259 74.75, -269 74.75" fill="none" stroke="black"/>
        <path d="M -253 74.75 C -263 74.75, -259 100.75, -269 100.75" fill="none" stroke="black"/>
        <rect x="-522" y="39.75" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-520" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="-510" y="65.75" width="241" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-508" y="79.75" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Assign funds availability policy to accounts</text>
        <rect x="-404" y="91.75" width="135" height="18" rx="3" fill="#7F7FFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-402" y="105.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="-253" y="65.75" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-251" y="79.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <circle cx="-253" cy="74.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(13)"/>
        <rect x="-125" y="39.75" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-123" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
        <circle cx="-125" cy="48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(11)"/>
      </g>
    </g>
    <g transform="translate(12,0)">
      <g>
        <style>
          text.leaf {
            pointer-events: none;
          }
        </style>
        <path d="M 4 0 C 14 0, 10 -48.75, 20 -48.75" fill="none" stroke="black"/>
        <path d="M 4 0 C 14 0, 10 48.75, 20 48.75" fill="none" stroke="black"/>
        <path d="M 146 -48.75 C 156 -48.75, 152 -81.25, 162 -81.25" fill="none" stroke="black"/>
        <path d="M 146 -48.75 C 156 -48.75, 152 -16.25, 162 -16.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -107.25, 296 -107.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -81.25, 296 -81.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -55.25, 296 -55.25" fill="none" stroke="black"/>
        <rect x="296" y="-116.25" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-102.25" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="296" y="-90.25" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="296" y="-64.25" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-50.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="162" y="-90.25" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="164" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <circle cx="280" cy="-81.25" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(2)"/>
        <rect x="162" y="-25.25" width="140" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="leaf"/>
        <text x="164" y="-11.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Capital One Legal Entity</text>
        <circle cx="302" cy="-16.25" r="3" fill="#000000" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(6)"/>
        <rect x="20" y="-57.75" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="-43.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <circle cx="146" cy="-48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(1)"/>
        <path d="M 125 48.75 C 135 48.75, 131 22.75, 141 22.75" fill="none" stroke="black"/>
        <path d="M 125 48.75 C 135 48.75, 131 74.75, 141 74.75" fill="none" stroke="black"/>
        <rect x="141" y="13.75" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="143" y="27.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M 253 74.75 C 263 74.75, 259 48.75, 269 48.75" fill="none" stroke="black"/>
        <path d="M 253 74.75 C 263 74.75, 259 74.75, 269 74.75" fill="none" stroke="black"/>
        <path d="M 253 74.75 C 263 74.75, 259 100.75, 269 100.75" fill="none" stroke="black"/>
        <rect x="269" y="39.75" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="269" y="65.75" width="241" height="18" rx="3" fill="#FFFF7F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="79.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Assign funds availability policy to accounts</text>
        <rect x="269" y="91.75" width="135" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="105.75" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="141" y="65.75" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="143" y="79.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <circle cx="253" cy="74.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(11)"/>
        <rect x="20" y="39.75" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
        <circle cx="125" cy="48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(9)"/>
      </g>
    </g>
    <circle cx="0" cy="0" r="16"/>
  </g>
</svg>
"##;



static SAMPLE_SVG_ALT: &str = r##"
<svg viewBox="-532 -146 1064 294" xmlns="http://www.w3.org/2000/svg">
  <g>
    <g transform="translate(0 -250) scale(0.5)">

<g>
  <g>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" fill="#ffc77f"/>
    <path d="M -96.99305 -39.66228 C -97.29329 -14.579836 -87.87417 10.597343 -68.73569 29.73573 C -64.436375 34.035063 -59.83232 37.84391 -54.99305 41.162277 C -55.29329 16.079833 -45.87417 -9.097347 -26.735686 -28.235732 C -18.86879 -36.10267 -9.981547 -42.327377 -.4999575 -46.909854 L -.4999575 -46.909854 C -31.2572 -61.77491 -68.26867 -59.35905 -96.99305 -39.66228 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" fill="#ffff7f"/>
    <path d="M -96.99304 -39.662143 C -101.83231 -36.343777 -106.43637 -32.534928 -110.73568 -28.235594 C -148.42144 9.449972 -148.42144 70.5503 -110.73568 108.23586 C -80.91701 138.05468 -36.43895 144.27939 -.4999575 126.90999 C -9.981547 122.3275 -18.868785 116.1028 -26.73568 108.23586 C -45.282956 89.68868 -54.70208 65.470036 -54.993044 41.162415 C -59.83231 37.84405 -64.43637 34.0352 -68.73568 29.735867 C -87.87416 10.597481 -97.29329 -14.579698 -96.99304 -39.662143 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" fill="#804000"/>
    <path d="M -54.99305 41.162277 C -22.253024 63.61264 21.25311 63.61264 53.99313 41.162277 C 54.293375 16.079834 44.874254 -9.097344 25.73577 -28.235728 C 17.868876 -36.102664 8.981638 -42.32737 -.4999493 -46.909846 L -.49995175 -46.90985 C -9.981544 -42.327376 -18.868787 -36.102668 -26.735686 -28.235728 C -45.87417 -9.097344 -55.29329 16.079834 -54.99305 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" fill="#80ff80"/>
    <path d="M -55.000006 41.162277 C -54.70904 65.4699 -45.28992 89.68855 -26.742643 108.23573 C -18.875747 116.10267 -9.988509 122.32737 -.5069217 126.90985 C 8.974673 122.32738 17.861916 116.10267 25.728815 108.23573 C 44.27609 89.68855 53.69521 65.4699 53.98618 41.162277 C 21.246153 63.61264 -22.25998 63.61264 -55.000004 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" fill="#f58cff"/>
    <path d="M 54.49308 41.162277 C 59.332344 37.843914 63.9364 34.03507 68.2357 29.73574 C 87.37419 10.59735 96.79331 -14.579833 96.49307 -39.66228 C 67.76869 -59.35904 30.75723 -61.774895 -5745658e-12 -46.909847 L -33114763e-13 -46.90984 C 9.481584 -42.327364 18.368822 -36.10266 26.235718 -28.235723 C 45.3742 -9.09734 54.79332 16.079836 54.49308 41.162277 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" fill="#8080ff"/>
    <path d="M 54.49309 41.162268 C 54.20213 65.46989 44.783007 89.68854 26.23573 108.23573 C 18.36883 116.10267 9.481586 122.32737 -5745658e-12 126.90985 C 35.93899 144.27925 80.41706 138.05455 110.23573 108.23573 C 147.92149 70.55016 147.92149 9.449834 110.23573 -28.235732 C 105.93641 -32.53507 101.33235 -36.343923 96.49308 -39.66229 C 96.79332 -14.579843 87.3742 10.597341 68.235715 29.73573 C 63.93641 34.03506 59.332355 37.843905 54.49309 41.162268 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <g>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" fill="#ff6163"/>
    <path d="M -.5069629 -46.90976 C 30.2503 -61.77482 67.26179 -59.35895 95.98618 -39.662144 L 95.98618 -39.662143 C 95.69524 -63.96979 86.27611 -88.18848 67.728816 -106.73568 C 30.04325 -144.42144 -31.057076 -144.42144 -68.74264 -106.73568 C -87.28992 -88.1885 -96.70904 -63.969845 -97 -39.66222 C -68.27563 -59.358956 -31.264185 -61.7748 -.50696047 -46.90975 Z" stroke="#000000" stroke-width="2" fill-opacity="0"/>
  </g>
  <text font-family="Arial" font-size="18" fill="#000000" x="-47.153914" y="-79.83107">Commercial</text>
  <text x="-131.024" y="62.162277" font-family="Arial" font-size="16" fill="#000000">Consumer</text>
  <text font-family="Arial" font-size="16" fill="#000000" x="75.5" y="65.5">SBB</text>
  <text font-family="Arial" font-size="16" fill="#FFFFFF" x="-9.362914" y="17">All</text>
</g>
    </g>
    <g transform="translate(-12,0)">
      <g>
        <style>
          text.leaf {
            pointer-events: none;
          }x
        </style>
        <path d="M -4 0 C -14 0, -10 -48.75, -20 -48.75" fill="none" stroke="black"/>
        <path d="M -4 0 C -14 0, -10 48.75, -20 48.75" fill="none" stroke="black"/>
        <path d="M -146 -48.75 C -156 -48.75, -152 -81.25, -162 -81.25" fill="none" stroke="black"/>
        <path d="M -146 -48.75 C -156 -48.75, -152 -16.25, -162 -16.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -107.25, -296 -107.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -81.25, -296 -81.25" fill="none" stroke="black"/>
        <path d="M -280 -81.25 C -290 -81.25, -286 -55.25, -296 -55.25" fill="none" stroke="black"/>
        <rect x="-451" y="-116.25" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-449" y="-102.25" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="-468" y="-90.25" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-466" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="-435" y="-64.25" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-433" y="-50.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="-280" y="-90.25" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-278" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <circle cx="-280" cy="-81.25" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(2)"/>
        <circle cx="-302" cy="-16.25" r="3" fill="#000000" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(6)"/>
        <rect x="-146" y="-57.75" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-144" y="-43.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <circle cx="-146" cy="-48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(1)"/>
        <path d="M -125 48.75 C -135 48.75, -131 22.75, -141 22.75" fill="none" stroke="black"/>
        <path d="M -125 48.75 C -135 48.75, -131 74.75, -141 74.75" fill="none" stroke="black"/>
        <rect x="-310" y="13.75" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-308" y="27.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M -253 74.75 C -263 74.75, -259 48.75, -269 48.75" fill="none" stroke="black"/>
        <path d="M -253 74.75 C -263 74.75, -259 74.75, -269 74.75" fill="none" stroke="black"/>
        <path d="M -253 74.75 C -263 74.75, -259 100.75, -269 100.75" fill="none" stroke="black"/>
        <rect x="-522" y="39.75" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-520" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="-404" y="91.75" width="135" height="18" rx="3" fill="#7F7FFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="-402" y="105.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="-253" y="65.75" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-251" y="79.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <circle cx="-253" cy="74.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(13)"/>
        <rect x="-125" y="39.75" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="-123" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
        <circle cx="-125" cy="48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(11)"/>
      </g>
    </g>
    <g transform="translate(12,0)">
      <g>
        <style>
          text.leaf {
            pointer-events: none;
          }
        </style>
        <path d="M 4 0 C 14 0, 10 -48.75, 20 -48.75" fill="none" stroke="black"/>
        <path d="M 4 0 C 14 0, 10 48.75, 20 48.75" fill="none" stroke="black"/>
        <path d="M 146 -48.75 C 156 -48.75, 152 -81.25, 162 -81.25" fill="none" stroke="black"/>
        <path d="M 146 -48.75 C 156 -48.75, 152 -16.25, 162 -16.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -107.25, 296 -107.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -81.25, 296 -81.25" fill="none" stroke="black"/>
        <path d="M 280 -81.25 C 290 -81.25, 286 -55.25, 296 -55.25" fill="none" stroke="black"/>
        <rect x="296" y="-116.25" width="155" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-102.25" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on accounts</text>
        <rect x="296" y="-90.25" width="172" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Enact a status on transactions</text>
        <rect x="296" y="-64.25" width="139" height="18" rx="3" fill="#FF6163" stroke="black" stroke-width="1" class="leaf"/>
        <text x="298" y="-50.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Link overdraft protection</text>
        <rect x="162" y="-90.25" width="118" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="164" y="-76.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Administer Accounts</text>
        <circle cx="280" cy="-81.25" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(2)"/>
        <rect x="162" y="-25.25" width="140" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="leaf"/>
        <text x="164" y="-11.25" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Capital One Legal Entity</text>
        <circle cx="302" cy="-16.25" r="3" fill="#000000" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(6)"/>
        <rect x="20" y="-57.75" width="126" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="-43.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Management</text>
        <circle cx="146" cy="-48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(1)"/>
        <path d="M 125 48.75 C 135 48.75, 131 22.75, 141 22.75" fill="none" stroke="black"/>
        <path d="M 125 48.75 C 135 48.75, 131 74.75, 141 74.75" fill="none" stroke="black"/>
        <rect x="141" y="13.75" width="169" height="18" rx="3" fill="#FFC77F" stroke="black" stroke-width="1" class="leaf"/>
        <text x="143" y="27.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Perform Year End Processing</text>
        <path d="M 253 74.75 C 263 74.75, 259 48.75, 269 48.75" fill="none" stroke="black"/>
        <path d="M 253 74.75 C 263 74.75, 259 74.75, 269 74.75" fill="none" stroke="black"/>
        <path d="M 253 74.75 C 263 74.75, 259 100.75, 269 100.75" fill="none" stroke="black"/>
        <rect x="269" y="39.75" width="253" height="18" rx="3" fill="#F58CFF" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="leaf">Aggregate available balance within customer</text>
        <rect x="269" y="91.75" width="135" height="18" rx="3" fill="#804000" stroke="black" stroke-width="1" class="leaf"/>
        <text x="271" y="105.75" font-family="Arial" fill="#FFFFFF" style="font-style: normal; font-size: 12.4px" class="leaf">Maintain daily balances</text>
        <rect x="141" y="65.75" width="112" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="143" y="79.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Calculate Balances</text>
        <circle cx="253" cy="74.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(11)"/>
        <rect x="20" y="39.75" width="105" height="18" rx="3" fill="#E8E8E8" stroke="black" stroke-width="1" class="branch"/>
        <text x="22" y="53.75" font-family="Arial" fill="#000000" style="font-style: normal; font-size: 12.4px" class="branch">Account Servicing</text>
        <circle cx="125" cy="48.75" r="3" fill="#FFFFFF" stroke="#000000" stroke-width="1.0" onclick="toggle_then_draw(9)"/>
      </g>
    </g>
    <circle cx="0" cy="0" r="16"/>
  </g>
</svg>
"##;
