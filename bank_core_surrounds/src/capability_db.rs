//
// Contains support for classes that contain the data from the capabilities_db.xlsx file
//

use serde::Deserialize;
use calamine::{Error, Xlsx, Reader, RangeDeserializerBuilder};
use itertools::Itertools;
use std::collections::HashSet;
use crate::capability_tree::CoreOrSurround;
use crate::used_by::{UsedBy, UsedBySet};


const NOT_REALLY_SURROUNDS: [&str; 2] = ["Destination Core", "N/A"];


#[derive(Deserialize, Debug)]
pub struct CapabilitiesRow {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(default, rename(deserialize = "ParentId"))]
    pub parent_id: String,
    #[serde(default, rename(deserialize = "Name"))]
    pub name: String,
    #[serde(default, rename(deserialize = "Level"))]
    pub level: i32,
    #[serde(default, rename(deserialize = "Description"))]
    pub description: String,
    #[serde(default, rename(deserialize = "Core/Surround"), deserialize_with = "deserialize_core_surround")]
    pub core_surround: CoreOrSurround,
    #[serde(default, rename(deserialize = "Notes"))]
    pub notes: String,
    #[serde(default, rename(deserialize = "UsedByConsumer"), deserialize_with = "deserialize_used_by")]
    pub used_by_consumer: UsedBy,
    #[serde(default, rename(deserialize = "UsedBySBB"), deserialize_with = "deserialize_used_by")]
    pub used_by_sbb: UsedBy,
    #[serde(default, rename(deserialize = "UsedByCommercial"), deserialize_with = "deserialize_used_by")]
    pub used_by_commercial: UsedBy,
    #[serde(default, rename(deserialize = "SSRId"))]
    pub ssr_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SurroundSheetRow {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(default, rename(deserialize = "Functionality"))]
    pub functionality: String,
    #[serde(default, rename(deserialize = "Description"))]
    pub description: String,
    #[serde(default, rename(deserialize = "Notes"))]
    pub notes: String,
    #[serde(default, rename(deserialize = "CoreOrSurround"), deserialize_with = "deserialize_surround_list")]
    pub core_surround: SurroundList,
    #[serde(default, rename(deserialize = "ConsumerCurrent"), deserialize_with = "deserialize_surround_list")]
    pub consumer_current: SurroundList,
    #[serde(default, rename(deserialize = "SBBCurrent"), deserialize_with = "deserialize_surround_list")]
    pub sbb_current: SurroundList,
    #[serde(default, rename(deserialize = "CommercialCurrent"), deserialize_with = "deserialize_surround_list")]
    pub commercial_current: SurroundList,
    #[serde(default, rename(deserialize = "ConsumerDestination"), deserialize_with = "deserialize_surround_list")]
    pub consumer_destination: SurroundList,
    #[serde(default, rename(deserialize = "SBBDestination"), deserialize_with = "deserialize_surround_list")]
    pub sbb_destination: SurroundList,
    #[serde(default, rename(deserialize = "CommercialDestination"), deserialize_with = "deserialize_surround_list")]
    pub commercial_destination: SurroundList,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SurroundRow {
    #[serde(rename(deserialize = "Id"))]
    pub id: String,
    #[serde(default, rename(deserialize = "Name"))]
    pub name: String,
    #[serde(rename(deserialize = "IsCore"), deserialize_with = "deserialize_yes_no")]
    pub is_core: bool,
    #[serde(rename(deserialize = "IsNewSystem"), deserialize_with = "deserialize_yes_no")]
    pub is_new_system: bool,
    #[serde(rename(deserialize = "IsCurrent"), deserialize_with = "deserialize_yes_no")]
    pub is_current: bool,
    #[serde(rename(deserialize = "IsDestination"), deserialize_with = "deserialize_yes_no")]
    pub is_destination: bool,
    #[serde(rename(deserialize = "ConsumerCurrent"), deserialize_with = "deserialize_yes_no")]
    pub consumer_current: bool,
    #[serde(rename(deserialize = "SBBCurrent"), deserialize_with = "deserialize_yes_no")]
    pub sbb_current: bool,
    #[serde(rename(deserialize = "CommercialCurrent"), deserialize_with = "deserialize_yes_no")]
    pub commercial_current: bool,
    #[serde(rename(deserialize = "ConsumerDestination"), deserialize_with = "deserialize_yes_no")]
    pub consumer_destination: bool,
    #[serde(rename(deserialize = "SBBDestination"), deserialize_with = "deserialize_yes_no")]
    pub sbb_destination: bool,
    #[serde(rename(deserialize = "CommercialDestination"), deserialize_with = "deserialize_yes_no")]
    pub commercial_destination: bool,
    #[serde(default, rename(deserialize = "AppOnMainframe"))]
    pub app_on_mainframe: String,
    #[serde(default, rename(deserialize = "Links"))]
    pub links: String,
}

#[derive(Debug)]
pub struct SurroundList {
    names: Vec<String>
}


/// An object that contains all the data from the Capabilities DB file.
#[derive(Debug)]
pub struct CapabilitiesDB {
    pub capabilities: Vec<CapabilitiesRow>,
    pub surround_sheet_rows: Vec<SurroundSheetRow>,
    pub surrounds: Vec<SurroundRow>,
}


fn deserialize_core_surround<'de, D>(deserializer: D) -> Result<CoreOrSurround,D::Error>
    where D: serde::Deserializer<'de>
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => {
            let st: &str = &s;
            Ok(CoreOrSurround::from(st))
        },
        Ok(_) => panic!("Blank or non-string field in Core/Surround."), // FIXME: should return err, but I don't know how
        Err(e) => Err(e),
    }
}


fn deserialize_used_by<'de, D>(deserializer: D) -> Result<UsedBy,D::Error>
    where D: serde::Deserializer<'de>
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => {
            let st: &str = &s;
            Ok(UsedBy::from(st))
        },
        Ok(_) => panic!("Blank or non-string field in UsedBy."), // FIXME: should return err, but I don't know how
        Err(e) => Err(e),
    }
}


fn deserialize_yes_no<'de, D>(deserializer: D) -> Result<bool,D::Error>
    where D: serde::Deserializer<'de>
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => {
            let st: &str = &s;
            match st {
                "Yes" => Ok(true),
                "No" => Ok(false),
                _ => panic!("String other than 'Yes' or 'No' used for boolean."),
            }
        },
        Ok(_) => panic!("Blank or non-string field in UsedBy."), // FIXME: should return err, but I don't know how
        Err(e) => Err(e),
    }
}

fn deserialize_surround_list<'de, D>(deserializer: D) -> Result<SurroundList,D::Error>
    where D: serde::Deserializer<'de>
{
    let data_type = calamine::DataType::deserialize(deserializer);
    match data_type {
        Ok(calamine::DataType::String(s)) => {
            let names: Vec<String> = s.split("\n")
                .map(|x| match x.strip_prefix("New Surround - ") {
                    None => x,
                    Some(s) => s,
                })
                .map(|s| s.to_string())
                .collect();
            Ok(SurroundList{names})
        },
        Ok(calamine::DataType::Empty) => {
            Ok(SurroundList{names: Vec::new()})
        }
        Ok(_) => panic!("Non-string field in UsedBy."), // FIXME: should return err, but I don't know how
        Err(e) => Err(e),
    }
}



pub fn read_db(bytes: &[u8]) -> Result<CapabilitiesDB, Error> {
    // --- Open file ---
    let mut workbook: Xlsx<_> = Xlsx::new(std::io::Cursor::new(bytes))?;

    // --- Read capabilities ---
    let range = workbook.worksheet_range("Capabilities")
        .ok_or(Error::Msg("Missing tab 'Capabilities'"))??;
    let capabilities: Vec<CapabilitiesRow> = RangeDeserializerBuilder::new()
        .from_range::<_, CapabilitiesRow>(&range)? // use header names and the bind object
        .try_collect()?;

    // --- Read surround_sheet_rows ---
    let range = workbook.worksheet_range("SurroundSheetRows")
        .ok_or(Error::Msg("Missing tab 'SurroundSheetRows'"))??;
    let surround_sheet_rows: Vec<SurroundSheetRow> = RangeDeserializerBuilder::new()
        .from_range::<_, SurroundSheetRow>(&range)? // use header names and the bind object
        .try_collect()?;

    // --- Read surrounds ---
    let range = workbook.worksheet_range("Surrounds")
        .ok_or(Error::Msg("Missing tab 'Surrounds'"))??;
    let surrounds: Vec<SurroundRow> = RangeDeserializerBuilder::new()
        .from_range::<_, SurroundRow>(&range)? // use header names and the bind object
        .try_collect()?;

    // --- Return the object ---
    Ok(CapabilitiesDB{capabilities, surround_sheet_rows, surrounds})
}


impl Default for SurroundList {
    /// Default to an empty list.
    fn default() -> Self {
        SurroundList{names: Default::default()}
    }
}


impl CapabilitiesDB {

    /// This is given a capability by ID and returns the row with that capability's data, or None
    /// if there is no such capability id
    pub fn get_capability_by_id(&self, cap_id: &str) -> Option<&CapabilitiesRow> {
        self.capabilities.iter().filter(|x| x.id == cap_id).nth(0)
    }

    /// This is given a capability ID and returns a vector of the IDs of that capability's
    /// immediate children.
    pub fn get_child_capability_ids_by_id(&self, cap_id: &str) -> Vec<String> {
        self.capabilities.iter()
            .filter(|x| &x.parent_id == cap_id)
            .map(|x| x.id.clone())
            .collect()
    }

    pub fn get_ssr_by_id(&self, ssr_id: &str) -> Option<&SurroundSheetRow> {
        self.surround_sheet_rows.iter().filter(|x| x.id == ssr_id).nth(0)
    }


    /// This finds the surrounds that are expected to implement a given capability. It is passed
    /// the ID for a capability; it uses the data to find surrounds that are expected to implement
    /// it, and it returns a vector of pairs, giving the name of the surround and a UsedBySet
    /// containing only YES and NO values (at least one of which should be YES) to indicate
    /// WHICH divisions are using that surround.
    pub fn get_related_surrounds<'a>(&'a self, cap_id: &str) -> Vec<(&'a str, UsedBySet)> {
        // --- First, find the list of SSRs (if any) ---
        let mut ssr_ids: Vec<&str> = Vec::new();
        let cap = self.get_capability_by_id(cap_id).expect("Capability ID was invalid.");
        let ssrid_opt: Option<&str> = match &cap.ssr_id {None => None, Some(x) => Some(&x)};
        match ssrid_opt {
            Some("CORE") | Some("") | None => {
            },
            Some("*") => {
                let mut ancestor_cap: &CapabilitiesRow = cap; // start with self
                loop {
                    // move ancestor_cap to parent
                    ancestor_cap = self.get_capability_by_id(&ancestor_cap.parent_id).expect("Parent capability invalid.");
                    let ancestor_ssrid_opt: Option<&str> = match &ancestor_cap.ssr_id {None => None, Some(x) => Some(&x)};
                    match ancestor_ssrid_opt {
                        Some("*") => continue,
                        Some("CORE") | Some("^") | Some("") | None => panic!("Node with * for ssr_id has no ancestor with a value."),
                        Some(ssr_id) => {
                            ssr_ids.push(ssr_id); // We found the right ancestor ssr_id to use
                            break;
                        }
                    }
                }
            },
            Some("^") => {
                fn collect_ssrids<'b>(capdb: &'b CapabilitiesDB, ssr_ids: &mut Vec<&'b str>, cap: &'b CapabilitiesRow) {
                    let descendant_ssrid_opt: Option<&str> = match &cap.ssr_id {None=>None, Some(x) => Some(&x)};
                    match descendant_ssrid_opt {
                        Some("") | Some("*") | None => panic!("Node with ^ for ssr_id has descendant with \"\" or \"*\"."),
                        Some("^") | Some("CORE") => {
                            for child_id in capdb.get_child_capability_ids_by_id(&cap.id) {
                                let child_cap = capdb.get_capability_by_id(&child_id).expect("Capability id not found.");
                                collect_ssrids(capdb, ssr_ids, child_cap);
                            }
                        },
                        Some(ssr_id) => {
                            ssr_ids.push(ssr_id);
                            // do not recurse below this
                        },
                    }
                }
                collect_ssrids(self, &mut ssr_ids, cap);
            }
            Some(ssr_id) => {
                ssr_ids.push(ssr_id);
            },
        }

        // --- Find the destination systems for that SSR_ID ---
        let mut answer: Vec<(&'a str, UsedBySet)> = Vec::new();
        for ssr_id in ssr_ids {
            let ssr_row = self.get_ssr_by_id(ssr_id).expect("SSRID is invalid.");
            let mut unique_names: HashSet<&str> = HashSet::new();
            unique_names.extend(ssr_row.consumer_destination.names.iter().map(|x| x.as_str()));
            unique_names.extend(ssr_row.sbb_destination.names.iter().map(|x| x.as_str()));
            unique_names.extend(ssr_row.commercial_destination.names.iter().map(|x| x.as_str()));
            for surround_name in unique_names {
                if NOT_REALLY_SURROUNDS.contains(&surround_name) {
                    continue; // skip these special names
                }
                fn ub(sl: &SurroundList, name: &str) -> UsedBy {
                    if sl.names.iter().any(|x| x.as_str() == name) {
                        UsedBy::Yes
                    } else {
                        UsedBy::No
                    }
                }
                let used_by = UsedBySet::from_fields(
                    ub(&ssr_row.consumer_destination, surround_name),
                    ub(&ssr_row.sbb_destination, surround_name),
                    ub(&ssr_row.commercial_destination, surround_name),
                );
                answer.push((surround_name, used_by))
            }
        }
        answer
    }

}
