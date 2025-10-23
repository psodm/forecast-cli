use csv;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

use crate::util;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CsvAllocationRow {
    pub resource_id: String,
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
    pub resource_id: String,
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

pub fn convert_to_allocations(
    rows: Vec<CsvAllocationRow>,
) -> Result<Vec<AllocationRow>, Box<dyn std::error::Error>> {
    rows.into_iter()
        .map(|csv_row| convert_row(csv_row))
        .collect()
}

pub fn forecast_utilization(
    allocations: &Vec<AllocationRow>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Forecasting Utilization...");
    // Implementation goes here
    Ok(())
}

pub fn forecast_bench(allocations: &Vec<AllocationRow>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Forecasting Bench...\n");
    // Implementation goes here
    let names = get_unique_names(allocations);
    let week0 = util::get_first_day_of_current_week();
    let week1 = week0 + chrono::Duration::days(7);
    let week2 = week1 + chrono::Duration::days(7);
    let week3 = week2 + chrono::Duration::days(7);
    let week4 = week3 + chrono::Duration::days(7);
    println!("\n{:30}|{:>15}{:>15}{:>15}{:>15}{:>15}", "Resource", week0.to_string(), week1.to_string(), week2.to_string(), week3.to_string(), week4.to_string());
    println!("----------------------------------------------------------------------------------------------------------");
    for name in names {
        get_benched_resources(&name, &allocations);
    }
    println!("\n");
    Ok(())
}

struct WeeklyAllocations {
    week0: f64,
    week1: f64,
    week2: f64,
    week3: f64,
    week4: f64,
}

pub fn forecast_overallocated(
    allocations: &Vec<AllocationRow>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Forecasting Overallocated Resources...\n");
    let names = get_unique_names(allocations);
    let week0 = util::get_first_day_of_current_week();
    let week1 = week0 + chrono::Duration::days(7);
    let week2 = week1 + chrono::Duration::days(7);
    let week3 = week2 + chrono::Duration::days(7);
    let week4 = week3 + chrono::Duration::days(7);
    println!(
        "\n{:30}|{:>15}{:>15}{:>15}{:>15}{:>15}",
        "Resource",
        week0.to_string(),
        week1.to_string(),
        week2.to_string(),
        week3.to_string(),
        week4.to_string(),
    );
    println!(
        "----------------------------------------------------------------------------------------------------------"
    );
    for name in names {
        get_overallocated_resources(&name, &allocations);
    }
    println!("\n\n");
    Ok(())
}

fn get_unique_names(allocations: &Vec<AllocationRow>) -> Vec<String> {
    let mut unique_names = Vec::new();
    for allocation in allocations {
        if !unique_names.contains(&allocation.resource_name) {
            unique_names.push(allocation.resource_name.clone());
        }
    }
    unique_names.sort();
    unique_names
}

fn convert_row(csv_row: CsvAllocationRow) -> Result<AllocationRow, Box<dyn std::error::Error>> {
    Ok(AllocationRow {
        resource_id: csv_row.resource_id,
        resource_name: csv_row.resource_name,
        resource_manager: csv_row.resource_manager,
        employment_type: csv_row.employment_type,
        investment_id: csv_row.investment_id,
        investment_name: csv_row.investment_name,
        investment_type: csv_row.investment_type,
        investment_role: csv_row.investment_role,
        investment_manager: csv_row.investment_manager,
        allocation: csv_row.allocation,
        a0: util::extract_number(&csv_row.a0),
        a1: util::extract_number(&csv_row.a1),
        a2: util::extract_number(&csv_row.a2),
        a3: util::extract_number(&csv_row.a3),
        a4: util::extract_number(&csv_row.a4),
        a5: util::extract_number(&csv_row.a5),
        a6: util::extract_number(&csv_row.a6),
        a7: util::extract_number(&csv_row.a7),
        a8: util::extract_number(&csv_row.a8),
        a9: util::extract_number(&csv_row.a9),
        a10: util::extract_number(&csv_row.a10),
        a11: util::extract_number(&csv_row.a11),
        a12: util::extract_number(&csv_row.a12),
        a13: util::extract_number(&csv_row.a13),
        hard_allocation: csv_row.hard_allocation,
        h0: util::extract_number(&csv_row.h0),
        h1: util::extract_number(&csv_row.h1),
        h2: util::extract_number(&csv_row.h2),
        h3: util::extract_number(&csv_row.h3),
        h4: util::extract_number(&csv_row.h4),
        h5: util::extract_number(&csv_row.h5),
        h6: util::extract_number(&csv_row.h6),
        h7: util::extract_number(&csv_row.h7),
        h8: util::extract_number(&csv_row.h8),
        h9: util::extract_number(&csv_row.h9),
        h10: util::extract_number(&csv_row.h10),
        h11: util::extract_number(&csv_row.h11),
        h12: util::extract_number(&csv_row.h12),
        h13: util::extract_number(&csv_row.h13),
    })
}

fn get_overallocated_resources(name: &String, allocations: &Vec<AllocationRow>) {
    let mut allocation_map: HashMap<String, WeeklyAllocations> = HashMap::new();
    for allocation in allocations {
        if &allocation.resource_name == name {
            if allocation_map.get(name).is_none() {
                allocation_map.insert(
                    name.clone(),
                    WeeklyAllocations {
                        week0: allocation.a0,
                        week1: allocation.a1,
                        week2: allocation.a2,
                        week3: allocation.a3,
                        week4: allocation.a4,
                    },
                );
            } else {
                let entry = allocation_map.get_mut(name).unwrap();
                entry.week0 += allocation.a0;
                entry.week1 += allocation.a1;
                entry.week2 += allocation.a2;
                entry.week3 += allocation.a3;
                entry.week4 += allocation.a4;
            }
        }
    }
    for (key, value) in allocation_map.iter() {
        if value.week0 > 41.0 || value.week1 > 41.0 || value.week2 > 41.0 || value.week3 > 41.0 {
            println!(
                "{:<30}|{:>15.0}{:>15.0}{:>15.0}{:>15.0}{:>15.0}",
                key, value.week0, value.week1, value.week2, value.week3, value.week4
            );
        }
    }
}

fn get_benched_resources(name: &String, allocations: &Vec<AllocationRow>) {
    let mut allocation_map: HashMap<String, WeeklyAllocations> = HashMap::new();
    for allocation in allocations {
        if &allocation.resource_name == name {
            if allocation_map.get(name).is_none() {
                allocation_map.insert(name.clone(), WeeklyAllocations{
                    week0: allocation.a0,
                    week1: allocation.a1,
                    week2: allocation.a2,
                    week3: allocation.a3,
                    week4: allocation.a4,
                });
            } else {
                let entry = allocation_map.get_mut(name).unwrap();
                entry.week0 += allocation.a0;
                entry.week1 += allocation.a1;
                entry.week2 += allocation.a2;
                entry.week3 += allocation.a3;
                entry.week4 += allocation.a4;
            }
        }
    }
    for (key, value) in allocation_map.iter() {
        if value.week0 < 40.0 || value.week1 < 40.0 || value.week2 < 40.0 || value.week3 < 40.0 {
            println!("{:<30}|{:>15.0}{:>15.0}{:>15.0}{:>15.0}{:>15.0}", key, (40.0 - value.week0).abs(), (40.0 - value.week1).abs(), (40.0 - value.week2).abs(), (40.0 - value.week3).abs(), (40.0 - value.week4).abs());
        }
    }
}
