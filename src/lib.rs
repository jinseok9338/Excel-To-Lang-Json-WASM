use calamine::{open_workbook_auto_from_rs, DataType, Reader};
use dashmap::DashMap;
use serde_json::json;
use std::{collections::BTreeMap, io::Cursor};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    Ok(())
}

#[wasm_bindgen]
pub fn parse_excel(file_content: &[u8]) -> Result<JsValue, JsValue> {
    // Create a cursor from the byte slice
    let cursor = Cursor::new(file_content);

    let ko_map: DashMap<String, BTreeMap<String, String>> = DashMap::new();
    let en_map: DashMap<String, BTreeMap<String, String>> = DashMap::new();

    // Open the workbook from the cursor
    let mut workbook = open_workbook_auto_from_rs(cursor)
        .map_err(|e| JsValue::from_str(&format!("Error opening workbook: {}", e)))?;
    let sheet = workbook
        .worksheet_range_at(0)
        .ok_or(string_to_js_value(
            "Worksheet not found in the Excel file".to_string(),
        ))
        .unwrap();
    let sheet = sheet.unwrap();
    let mut rows = sheet.rows();

    // Skip the header row
    rows.next()
        .ok_or(string_to_js_value("Empty Excel file".to_string()))?;

    // Process data rows in parallel
    rows.for_each(|row| {
        if row.len() < 4 {
            return; // Skip rows with insufficient columns
        }

        let category = row[0].as_string().unwrap_or_default();
        let key = row[1].as_string().unwrap_or_default();
        let ko = row[3].as_string().unwrap_or_default();
        let en = row[2].as_string().unwrap_or_default();

        // Update ko_map
        ko_map
            .entry(category.to_string())
            .or_default()
            .insert(key.to_string(), ko.to_string());

        // Update en_map
        en_map
            .entry(category.to_string())
            .or_default()
            .insert(key.to_string(), en.to_string());
    });

    let ko_json = serde_json::to_value(ko_map.into_iter().collect::<BTreeMap<_, _>>())
        .map_err(|e| JsValue::from_str(&format!("Error serializing ko_map: {}", e)))?;

    let en_json = serde_json::to_value(en_map.into_iter().collect::<BTreeMap<_, _>>())
        .map_err(|e| JsValue::from_str(&format!("Error serializing en_map: {}", e)))?;

    let result = json!({
        "ko": ko_json,
        "en": en_json
    });

    // Convert the result to a JSON string and return it
    let result = serde_json::to_string(&result)
        .map_err(|e| JsValue::from_str(&format!("Error converting to JSON string: {}", e)));
    result.map(|s| JsValue::from_str(&s))
}

fn string_to_js_value(string: String) -> JsValue {
    JsValue::from_str(&string)
}
