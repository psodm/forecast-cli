pub fn print_banner() {
    println!("\n\n----------------------------------------");
    println!("{:^40}", "Professional Services Delivery");
    println!("{:^40}", "Forecasting Tools");
    println!("----------------------------------------\n");
}

pub fn get_allocation_filepath() -> String {
    loop {
        let input: String = dialoguer::Input::new()
            .with_prompt("Enter the path to the allocation file")
            .interact_text()
            .unwrap();
        if std::path::Path::new(&input).exists() {
            return input;
        } else {
            println!(
                "The file path '{}' does not exist. Please try again.",
                input
            );
        }
    }
}
