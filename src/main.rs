mod menus;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    menus::print_banner();
    let allocation_filepath = menus::get_allocation_filepath();
    println!("Using allocation file: {}", allocation_filepath);
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
