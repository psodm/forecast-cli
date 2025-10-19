use dialoguer::{Select, theme::ColorfulTheme};
mod allocations;
mod menus;
mod util;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Print the inital banner
    let version = std::env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_string());
    menus::print_banner(&version);

    // Get path to allocation CSV file
    let filepath = menus::get_allocation_filepath();

    // Read and parse the CSV file
    let result = allocations::read_csv_allocations::<allocations::CsvAllocationRow>(&filepath);
    let mut allocations: Vec<allocations::AllocationRow> = Vec::new();
    match result {
        Ok(rows) => {
            println!("\nSuccessfully read {} allocation rows...", rows.len());
            println!("Cleaning data (removing links)...");
            allocations = allocations::convert_to_allocations(rows)?;
            println!("Converted {} links to allocations.", allocations.len() * 14);
        }
        Err(_) => {
            return Err(
                "Incorrect fields found. Please check Allocations report settings in Clarity."
                    .into(),
            );
        }
    }
    println!("Allocations: {}", allocations.len());

    // Print the options menu
    loop {
        let selections = vec![
            "Forecast Utilization",
            "Forecast Bench",
            "Show Overallocated",
            "Exit",
        ];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("\n\nChoose an option")
            .default(0)
            .items(&selections)
            .interact()?;
        match selection {
            0 => {
                allocations::forecast_utilization(&allocations)?;
            }
            1 => {
                allocations::forecast_bench(&allocations)?;
            }
            2 => {
                allocations::forecast_overallocated(&allocations)?;
            }
            3 => {
                println!("\n\nExiting...\n");
                return Ok(());
            }
            _ => unreachable!(),
        }
    }
}

fn main() {
    let err = run();
    match ((), err) {
        ((), Ok(())) => {}
        (_, Err(err)) => {
            eprintln!("Application error: {:?}", err);
            std::process::exit(1);
        }
    }
}
