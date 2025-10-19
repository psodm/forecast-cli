use csv;
use serde::Deserialize;
use serde::de::DeserializeOwned;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvAllocationRow {
    pub resourcce_id: String,
    pub resource_name: String,
    pub resource_manager: String,
    pub employment_type: String,
    pub investment_id: String,
    pub investment_name: String,
    pub investment_type: String,
    pub investment_role: String,
    pub investment_manager: String,
    pub allocation: String,
    pub a0: String,
    pub a1: String,
    pub a2: String,
    pub a3: String,
    pub a4: String,
    pub a5: String,
    pub a6: String,
    pub a7: String,
    pub a8: String,
    pub a9: String,
    pub a10: String,
    pub a11: String,
    pub a12: String,
    pub a13: String,
    pub hard_allocation: String,
    pub h0: String,
    pub h1: String,
    pub h2: String,
    pub h3: String,
    pub h4: String,
    pub h5: String,
    pub h6: String,
    pub h7: String,
    pub h8: String,
    pub h9: String,
    pub h10: String,
    pub h11: String,
    pub h12: String,
    pub h13: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AllocationRow {
    pub resourcce_id: String,
    pub resource_name: String,
    pub resource_manager: String,
    pub employment_type: String,
    pub investment_id: String,
    pub investment_name: String,
    pub investment_type: String,
    pub investment_role: String,
    pub investment_manager: String,
    pub allocation: String,
    pub a0: f64,
    pub a1: f64,
    pub a2: f64,
    pub a3: f64,
    pub a4: f64,
    pub a5: f64,
    pub a6: f64,
    pub a7: f64,
    pub a8: f64,
    pub a9: f64,
    pub a10: f64,
    pub a11: f64,
    pub a12: f64,
    pub a13: f64,
    pub hard_allocation: String,
    pub h0: f64,
    pub h1: f64,
    pub h2: f64,
    pub h3: f64,
    pub h4: f64,
    pub h5: f64,
    pub h6: f64,
    pub h7: f64,
    pub h8: f64,
    pub h9: f64,
    pub h10: f64,
    pub h11: f64,
    pub h12: f64,
    pub h13: f64,
}

pub fn read_csv_allocations<T: DeserializeOwned>(
    filepath: &str,
) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(filepath)?;
    let mut allocations: Vec<T> = Vec::new();

    rdr.records().next(); // Skip header row
    rdr.records().next(); // Skip second row

    for result in rdr.deserialize() {
        let record: T = result?;
        allocations.push(record);
    }

    Ok(allocations)
}
