//
// Contains the code to render a list of surrounds.
//

use prog_draw::svg_render::SvgPositioned;
use prog_draw::geometry::{Point, Coord, Rect};
use prog_draw::text_size::get_system_text_sizer;
use prog_draw::svg_writer::{Attributes, Renderable, TagWriter, TagWriterError};
use crate::capability_db::{CapabilitiesDB, SurroundRow};
use crate::capability_tree::CapabilityNodeTree;
use crate::used_by::{get_color_strs, UsedBy, UsedBySet};
use crate::document::{BASELINE_RISE, NODE_ITEM_ROUND_CORNER, TEXT_ITEM_PADDING, ITEM_SPACING, SPACING_TO_SURROUNDS};


#[derive(Debug)]
pub struct SurroundItem {
    data: SurroundRow,
    location: Point,
    text_size: Point,
    used_by_set: UsedBySet,
}

#[derive(Debug)]
pub struct SurroundItems {
    items: Vec<SurroundItem>,
}


/// Returns the (width, height) of the text string.
fn get_text_size(text: &str) -> Point {
    match get_system_text_sizer().text_size(text, "Arial", 14.0) {
        Err(_) => panic!("Sizing isn't working."),
        Ok((width,height)) => (width as Coord, height as Coord)
    }
}


impl SurroundItem {
    pub fn new(data: &SurroundRow) -> Self {
        let data = data.clone();
        let location = Default::default();
        let text_size = get_text_size(&data.name);
        fn to_used_by(b: bool) -> UsedBy {
            match b {
                true => UsedBy::Yes,
                false => UsedBy::No,
            }
        }
        let used_by_set = UsedBySet::from_fields(
            to_used_by(data.consumer_destination),
            to_used_by(data.sbb_destination),
            to_used_by(data.commercial_destination)
        );
        Self{data, location, text_size, used_by_set}
    }
}


impl SurroundItems {
    pub fn new(capdb: &CapabilitiesDB) -> Self {
        let items = capdb.surrounds.iter()
            .filter(|x| x.is_destination)
            .map(|x| SurroundItem::new(x))
            .collect();
        SurroundItems{items}
    }


    /// Lays out the surround items. Is passed the (already laid out) surround_tree that
    /// it is being placed next to.
    pub fn layout(&mut self, surround_tree: &CapabilityNodeTree) {
        let item_height: Coord = self.items.iter()
            .map(|x| x.get_bbox().height())
            .sum();
        let spacing_height = ((self.items.len() - 1) as Coord) * ITEM_SPACING;
        let total_height = item_height + spacing_height;
        let half_height_of_top_item = self.items.first().unwrap().get_bbox().height() / 2.0;
        let starting_y = 0.5 * total_height - half_height_of_top_item;
        let x = surround_tree.get_bbox().right() + SPACING_TO_SURROUNDS;
        let mut y = 0.0 - starting_y;
        for mut item in self.items.iter_mut() {
            item.location = (x,y);
            y += item.get_bbox().height() + ITEM_SPACING;
        }
    }
}


impl Renderable for SurroundItem {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        // --- Decide the dimensions of everything ---
        let (loc_x, loc_y) = self.location;
        let (text_width, text_height) = self.text_size;
        let text_left = loc_x + TEXT_ITEM_PADDING;
        let text_top = loc_y - text_height / 2.0;
        let text_baseline = text_top + text_height - BASELINE_RISE;
        let box_left = text_left - TEXT_ITEM_PADDING;
        let box_top = text_top - TEXT_ITEM_PADDING;
        let box_width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let box_height = text_height + 2.0 * TEXT_ITEM_PADDING;

        // --- decide on decoration & color ---
        let (box_color, text_color) = get_color_strs(&self.used_by_set);

        // --- draw it ---
        tag_writer.single_tag("rect", Attributes::from([
            ("x", &*box_left.to_string()),
            ("y", &*box_top.to_string()),
            ("width", &*box_width.to_string()),
            ("height", &*box_height.to_string()),
            ("rx", &*NODE_ITEM_ROUND_CORNER.to_string()),
            ("fill", box_color),
            ("stroke", "black"),
            ("stroke-width", &*1.to_string()),
            ("onclick", &format!("show_surround_data('{}')", self.data.id)),
            ("class", "surround")
        ]))?;
        tag_writer.tag_with_text(
            "text",
            Attributes::from([
                ("x", &*text_left.to_string()),
                ("y", &*text_baseline.to_string()),
                ("font-family", "Arial"),
                ("fill", text_color),
                ("style", "font-style: normal; font-size: 12.4px; pointer-events: none"), // FIXME: size for 14 and set this to 12.4 seems to work. WHY?
                ("class", "surround"),
            ]),
            &self.data.name
        )?;

        // --- Finished ---
        Ok(())
    }
}


impl SvgPositioned for SurroundItem {
    // Gives the bounding box for the node including text AND the box around it. Remember, if
    // the node isn't correctly positioned yet, its location will be (0,0). Also know that
    // self.location is the center-left of the box it occupies.
    fn get_bbox(&self) -> Rect {
        let center = self.location;
        let (text_width, text_height) = self.text_size;
        let width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let height = text_height + 2.0 * TEXT_ITEM_PADDING;
        let left = center.0;
        let top = center.1 - height / 2.0;
        Rect::new_ltwh(left, top, width, height)
    }
}

impl Renderable for SurroundItems {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        for item in self.items.iter() {
            item.render(tag_writer)?;
        }
        Ok(())
    }
}

impl SvgPositioned for SurroundItems {
    fn get_bbox(&self) -> Rect {
        self.items.iter()
            .map(|x| x.get_bbox())
            .reduce(|r1, r2| r1.cover(&r2))
            .unwrap() // panic if there are NO items in the SurroundItems
    }
}