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
    match result {
        Ok(rows) => {
            println!("\nSuccessfully read {} allocation rows...", rows.len());
            // Further processing can be done here
        }
        Err(_) => {
            return Err(
                "Incorrect fields found. Please check Allocations report settings in Clarity."
                    .into(),
            );
        }
    }
    Ok(())
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
