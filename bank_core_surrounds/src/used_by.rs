use core::convert::From;
use core::default::Default;


#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum UsedBy {
    Yes,
    No,
    Maybe,
    Blank,
    Mixed,
}

#[derive(Debug, Copy, Clone)]
pub struct UsedBySet {
    pub consumer: UsedBy,
    pub sbb: UsedBy,
    pub commercial: UsedBy,
}


impl From<&str> for UsedBy {
    /// Parse a Used string or panic if it's invalid.
    fn from(s: &str) -> Self {
        match s {
            "Yes" => UsedBy::Yes,
            "No" => UsedBy::No,
            "Maybe" => UsedBy::Maybe,
            "" => UsedBy::Blank,
            "Mixed" => UsedBy::Mixed,
            _ => panic!("Invalid UsedBy: '{}'", s),
        }
    }
}

impl From<UsedBy> for &'static str {
    fn from(x: UsedBy) -> &'static str {
        match x {
            UsedBy::Yes => "Yes",
            UsedBy::No => "No",
            UsedBy::Maybe => "Maybe",
            UsedBy::Blank => "",
            UsedBy::Mixed => "Mixed",
        }
    }
}


impl Default for UsedBy {
    /// Default to "Blank", mostly so serde can read from excel.
    fn default() -> Self {
        UsedBy::Blank
    }
}


impl UsedBySet {
    /// Returns a UsedBy where everything is "Mixed".
    pub fn all_mixed() -> Self {
        UsedBySet{consumer: UsedBy::Mixed, sbb: UsedBy::Mixed, commercial: UsedBy::Mixed}
    }

    /// Returns a UsedBy where everything is "Blank".
    pub fn all_blank() -> Self {
        UsedBySet{consumer: UsedBy::Blank, sbb: UsedBy::Blank, commercial: UsedBy::Blank}
    }

    /// Given 3 strings from the file (in order consumer, sbb, commercial) this returns the
    /// UsedBySet or panics if the strings are invalid.
    pub fn from_fields(consumer: UsedBy, sbb: UsedBy, commercial: UsedBy) -> Self {
        UsedBySet{consumer, sbb, commercial}
    }
}

/// Returns the colors to use for (box, text) to represent this UsedBySet.
pub fn get_color_strs(used_by_set: &UsedBySet) ->  (&'static str, &'static str) {
    let used_bys = [used_by_set.consumer, used_by_set.sbb, used_by_set.commercial];
    let has_mixed = used_bys.iter().any(|x| *x == UsedBy::Mixed);
    let has_undecided = used_bys.iter().any(|x| *x == UsedBy::Blank || *x == UsedBy::Maybe);
    if has_mixed {
        ("#E8E8E8", "#000000") // grey box if anything is mixed
    } else if has_undecided {
        ("#E8E8E8", "#000000") // grey box if anything is undecided
    } else { // colored box if everything is Yes or No
        let bools = used_bys.map(|x| match x {
            UsedBy::Yes => true,
            UsedBy::No => false,
            _ => panic!()
        });
        match bools {
            [ true,  true,  true] => ("#804000", "#FFFFFF"),
            [ true,  true, false] => ("#80FF80", "#000000"),
            [ true, false,  true] => ("#FFC77F", "#000000"),
            [ true, false, false] => ("#FFFF7F", "#000000"),
            [false,  true,  true] => ("#F58CFF", "#000000"),
            [false,  true, false] => ("#8080FF", "#000000"),
            [false, false,  true] => ("#FF6163", "#000000"),
            [false, false, false] => ("#FFFFFF", "#000000"),
        }
    }
}


impl Default for UsedBySet {
    fn default() -> Self {
        UsedBySet::all_blank()
    }
}
