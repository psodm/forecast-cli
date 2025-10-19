use crate::util;
use colored::*;

pub fn print_banner(version: &str) {
    println!("\n\n----------------------------------------");
    println!("{:^40}", "Professional Services Delivery");
    println!("{:^40}", "Forecasting Tools");
    println!("{:^40}", version);
    println!("----------------------------------------\n");
}

fn get_input_with_prompt(prompt: &str) -> String {
    dialoguer::Input::new()
        .with_prompt(prompt)
        .interact_text()
        .unwrap()
}

pub fn get_allocation_filepath() -> String {
    loop {
        let input: String = get_input_with_prompt("Enter the path to the allocation file");
        if std::path::Path::new(&input).exists() {
            let ext = util::get_extension_from_filename(&input);
            if ext == Some("csv") {
                return input;
            } else {
                println!(
                    "{}",
                    "Filetype is not supported. Please provide a .csv file.".red()
                );
            }
        } else {
            println!(
                "{} '{}' {}",
                "The file path".red(),
                input.red(),
                "does not exist. Please try again".red()
            );
        }
    }
}
