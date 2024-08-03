use orion_cli::{orion_main, handle_error, init_logger};

fn main() {
    init_logger();

    // Run the main application function
    if let Err(e) = orion_main("Happy Days", "A welcoming crate for import into Orion and Synavate Projects", main_function) {
        handle_error(e);
    }

    println!("{}", orion_cli::colorize_output("Running the main function!", "green"));
}
