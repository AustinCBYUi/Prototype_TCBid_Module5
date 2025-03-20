use serde::{Deserialize};
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize)]
pub struct PricingData {
    pub linear_footage_rates: std::collections::HashMap<String, Vec<u32>>,
    pub over_300_rate: std::collections::HashMap<String, u32>,
    pub multipliers: std::collections::HashMap<String, f32>,
}


pub fn load_pricing_data() -> PricingData {
    let data = fs::read_to_string("src/pricing_sheet.json").expect("Unable to read the JSON file...");
    serde_json::from_str(&data).expect("Unable to parse the JSON file...")
}