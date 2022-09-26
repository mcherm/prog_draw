//
// Contains the code to render a list of surrounds.
//

use prog_draw::svg_render::SvgPositioned;
use prog_draw::geometry::{Point, Coord, Rect};
use prog_draw::text_size::get_system_text_sizer;
use prog_draw::svg_writer::{Attributes, Renderable, TagWriter, TagWriterError};
use crate::capability_db::{CapabilitiesDB, SurroundRow};
use crate::used_by::{get_color_strs, UsedBy, UsedBySet};
use crate::document::{BASELINE_RISE, NODE_ITEM_ROUND_CORNER, TEXT_ITEM_PADDING, ITEM_SPACING};
use crate::spaced_layout;
use crate::spaced_layout::Spaceable;


#[derive(Debug)]
pub struct SurroundItem {
    data: SurroundRow,
    text_size: Point,
    used_by_set: UsedBySet,
    desired_y: Option<Coord>,
    actual_y: Option<Coord>,
}

#[derive(Debug)]
pub struct SurroundItems {
    items: Vec<SurroundItem>,
    x_position: Coord,
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
        let desired_y = None;
        let actual_y = None;
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
        Self{data, text_size, used_by_set, desired_y, actual_y}
    }

    /// Returns the (should be unique) ID for this surround.
    pub fn id(&self) -> &str {
        self.data.id.as_str()
    }

    /// Provide a new preferred y location for this node. The position is requested, but
    /// there might be crowding.
    pub fn reposition(&mut self, y_loc: Coord) {
        self.desired_y = Some(y_loc);
    }

    /// Returns the actual y coordinate
    pub fn get_actual_y(&self) -> Option<Coord> {
        self.actual_y
    }
}

impl spaced_layout::Spaceable for SurroundItem {
    fn get_desired_loc(&self) -> Coord {
        self.desired_y.unwrap() // NOTE: Panics if used on one without a desired position
    }

    fn get_extent(&self) -> Coord {
        let (_, text_height) = self.text_size;
        text_height + 2.0 * TEXT_ITEM_PADDING + ITEM_SPACING
    }

    fn set_position(&mut self, pos: Coord) {
        self.actual_y = Some(pos);
    }
}


impl SurroundItems {
    pub fn new(capdb: &CapabilitiesDB) -> Self {
        let items = capdb.surrounds.iter()
            .filter(|x| x.is_destination)
            .filter(|x| x.name.as_str() != "Destination Core") // suppress this one in particular
            .map(|x| SurroundItem::new(x))
            .collect();
        let x_position = 0.0;
        SurroundItems{items, x_position}
    }

    /// Given the name of a surround, this returns the SurroundItem (or None if it
    /// isn't found).
    pub fn get_by_name(&self, name: &str) -> Option<&SurroundItem> {
        for item in self.items.iter() {
            if item.data.name.as_str() == name {
                return Some(item)
            }
        }
        return None
    }

    pub fn get_by_id_mut(&mut self, id: &str) -> Option<&mut SurroundItem> {
        for item in self.items.iter_mut() {
            if item.data.id.as_str() == id {
                return Some(item)
            }
        }
        return None
    }


    /// Calling this clears away all information about where individual items are placed.
    /// After doing so (and before rendering) calls will be made to re-position things.
    /// This call provides the first piece of information: the new x_position.
    pub fn reset_positions(&mut self, x_pos: Coord) {
        self.x_position = x_pos;
        for item in self.items.iter_mut() {
            item.desired_y = None;
            item.actual_y = None;
        }
    }


    /// This is called after some (not necessarily all) of the individual items have been
    /// given desired y positions using the reposition() call. It goes through the entire
    /// collection and determines actual positions such that the items won't overlap.
    pub fn distribute_space(&mut self) {
        // --- call the layout function ---
        let mut has_desired: Vec<&mut SurroundItem> = self.items.iter_mut()
            .filter(|x| x.desired_y.is_some())
            .collect();
        spaced_layout::layout(&mut has_desired);

        // --- Now place the items that don't have desired places, right below the rest ---
        let last_item_opt = self.items.iter()
            .filter(|x| x.actual_y.is_some())
            .max_by(|x,y| x.actual_y.unwrap().total_cmp(&y.actual_y.unwrap()));
        let mut pos = match last_item_opt {
            None => 0.0, // FIXME: Should it be centered instead of starting in the middle?
            Some(last_item) => last_item.actual_y.unwrap() + last_item.get_extent() / 2.0,
        };
        for item in self.items.iter_mut().filter(|x| x.desired_y.is_none()) {
            let item_extent = item.get_extent();
            let item_pos = pos + item_extent / 2.0;
            item.actual_y = Some(item_pos);
            pos += item.get_extent();
        }
    }
}


impl Renderable for SurroundItem {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        // --- Decide the dimensions of everything ---
        let loc_x = 0.0; // the parent provides the x positioning
        let loc_y = self.actual_y.expect("Must position items before rendering.");
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
        let is_new = self.data.is_new_system;

        // --- draw it ---
        if is_new {
            const RING_DIST: Coord = 2.5;
            tag_writer.single_tag("rect", Attributes::from([
                ("x", &*(box_left - RING_DIST).to_string()),
                ("y", &*(box_top - RING_DIST).to_string()),
                ("width", &*(box_width + 2.0 * RING_DIST).to_string()),
                ("height", &*(box_height + 2.0 * RING_DIST).to_string()),
                ("rx", &*(NODE_ITEM_ROUND_CORNER + RING_DIST).to_string()),
                ("fill", "#FFFFFF"),
                ("stroke", "black"),
                ("stroke-width", &*1.to_string()),
            ]))?;
        }
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
    // Gives the bounding box for the node including text AND the box around it. Calling
    // this when the node hasn't been correctly positioned will result in a panic. Because
    // the parent takes care of x-positioning, this box will always have its left edge
    // at zero.
    fn get_bbox(&self) -> Rect {
        let left = 0.0;
        let center = self.actual_y.expect("Must position items before getting bbox.");
        let (text_width, text_height) = self.text_size;
        let width = text_width + 2.0 * TEXT_ITEM_PADDING;
        let height = text_height + 2.0 * TEXT_ITEM_PADDING;
        let top = center - height / 2.0;
        Rect::new_ltwh(left, top, width, height)
    }
}

impl Renderable for SurroundItems {
    fn render(&self, tag_writer: &mut dyn TagWriter) -> Result<(), TagWriterError> {
        tag_writer.begin_tag("g", Attributes::from([
            ("transform", format!("translate({}, 0.0)", self.x_position))
        ]))?;
        for item in self.items.iter() {
            item.render(tag_writer)?;
        }
        tag_writer.end_tag("g")?;
        Ok(())
    }
}

impl SvgPositioned for SurroundItems {
    fn get_bbox(&self) -> Rect {
        self.items.iter()
            .map(|x| x.get_bbox())
            .reduce(|r1, r2| r1.cover(&r2))
            .unwrap() // panic if there are NO items in the SurroundItems
            .translated(self.x_position, 0.0)
    }
}
