//
// Contains support for classes that contain the data from the capabilities_db.xlsx file
//

use serde::Deserialize;
use calamine::{Error, Xlsx, Reader, RangeDeserializerBuilder};
use itertools::Itertools;
use crate::capability_tree::CoreOrSurround;
use crate::used_by::UsedBy;


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
    #[serde(rename(deserialize = "Core/Surround"), deserialize_with = "deserialize_core_surround")]
    pub core_surround: CoreOrSurround,
    #[serde(default, rename(deserialize = "Notes"))]
    pub notes: String,
    #[serde(rename(deserialize = "UsedByConsumer"), deserialize_with = "deserialize_used_by")]
    pub used_by_consumer: UsedBy,
    #[serde(rename(deserialize = "UsedBySBB"), deserialize_with = "deserialize_used_by")]
    pub used_by_sbb: UsedBy,
    #[serde(rename(deserialize = "UsedByCommercial"), deserialize_with = "deserialize_used_by")]
    pub used_by_commercial: UsedBy,
    #[serde(default, rename(deserialize = "SSRId"))]
    pub ssr_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SurroundSheetRow {
}

#[derive(Deserialize, Debug)]
pub struct SurroundRow {
}

/// An object that contains all the data from the Capabilities DB file.
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


pub fn read_db(bytes: &[u8]) -> Result<CapabilitiesDB, Error> {
    // --- Open file ---
    let mut workbook: Xlsx<_> = Xlsx::new(std::io::Cursor::new(bytes))?;

    // --- Read capabilities ---
    let capabilities_range = workbook.worksheet_range("Capabilities")
        .ok_or(Error::Msg("Missing tab 'Capabilities'"))??;
    let capabilities: Vec<CapabilitiesRow> = RangeDeserializerBuilder::new()
        .from_range::<_, CapabilitiesRow>(&capabilities_range)? // use header names and the bind object
        .try_collect()?;

    // --- Read surround_sheet_rows ---
    let capabilities_range = workbook.worksheet_range("SurroundSheetRows")
        .ok_or(Error::Msg("Missing tab 'SurroundSheetRows'"))??;
    let surround_sheet_rows: Vec<SurroundSheetRow> = RangeDeserializerBuilder::new()
        .from_range::<_, SurroundSheetRow>(&capabilities_range)? // use header names and the bind object
        .try_collect()?;

    // --- Read surrounds ---
    let capabilities_range = workbook.worksheet_range("Surrounds")
        .ok_or(Error::Msg("Missing tab 'Surrounds'"))??;
    let surrounds: Vec<SurroundRow> = RangeDeserializerBuilder::new()
        .from_range::<_, SurroundRow>(&capabilities_range)? // use header names and the bind object
        .try_collect()?;

    // --- Return the object ---
    Ok(CapabilitiesDB{capabilities, surround_sheet_rows, surrounds})
}
