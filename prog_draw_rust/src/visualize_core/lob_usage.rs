use core::option::Option;
use core::option::Option::{None, Some};
use std::ops::BitOrAssign;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct LobUsage {
    consumer: bool,
    sbb: bool,
    commercial: bool,
}


impl LobUsage {
    pub fn new(bools: [bool;3]) -> Self {
        LobUsage{consumer: bools[0], sbb: bools[1], commercial: bools[2]}
    }
}


impl BitOrAssign for LobUsage {
    /// Use the |= operator to update an LobUsage to include all fields mentioned in it
    /// previously OR mentioned in the right-hand-side argument.
    fn bitor_assign(&mut self, rhs: Self) {
        self.consumer |= rhs.consumer;
        self.sbb |= rhs.sbb;
        self.commercial |= rhs.commercial;
    }
}


/// Returns the (box_color, text_color) appropriate for drawing the given LobUsage.
pub fn get_color_strs(lob_usage: Option<LobUsage>) -> (&'static str, &'static str) {
    match lob_usage {
        None => ("#E8E8E8", "#000000"),
        Some(lu) => {
            match (lu.commercial, lu.sbb, lu.consumer) {
                ( true,  true,  true) => ("#804000", "#FFFFFF"),
                ( true,  true, false) => ("#F58CFF", "#000000"),
                ( true, false,  true) => ("#FFC77F", "#000000"),
                ( true, false, false) => ("#FF6163", "#000000"),
                (false,  true,  true) => ("#80FF80", "#000000"),
                (false,  true, false) => ("#7F7FFF", "#000000"),
                (false, false,  true) => ("#FFFF7F", "#000000"),
                (false, false, false) => ("#FFFFFF", "#000000"),
            }
        }
    }
}
