//!
//!       ____   _____   _____  ____   __
//!/ __ \ |  __ \ |_   _|/ __ \ | \ | |
//!| |  | || |__) |  | | | |  | ||  \| |
//!| |  | ||  _  /   | | | |  | || . ` |
//!| |__| || | \ \  _| |_| |__| || |\  |
//!\____/ |_|  \_\|_____|\____/ |_| \_|
//!_____  _       _____
//! / ____|| |     |_   _|
//!| |     | |       | |
//!| |     | |       | |
//!| |____ | |____  _| |_
//! \_____||______||_____|
//! 

//! # ORION CLI Custom Project
//! **Welcoming people to the Orion ecosystem**
//! 
//! ## Features:
//! - Import into any library or project
//! - Run your main function from the CLI
//! - More to come this is mostly an experimental project
//! 
//! Example: 
//! use yourcrate::your_module::main;
//! use orion_cli::orion_cli::main;
//! 

use log::{debug, error, info, log_enabled, Level};
use colored::*;
use clap::{App, Arg};
use thiserror::Error;
use env_logger::{Env, Builder};

const env: Env = Env::new()
 .filter_or("MY_LOG", "info")
 .write_style_or("MY_LOG_STYLE", "always");
 
pub fn init_logger() {
    let env = Env::new()
        // EXPORT ENV VARIABLES MY_LOG or MY_LOG_STYLE.
        // Alternatively use dotenv.
        .filter_or("MY_LOG", "info")
        .write_style_or("MY_LOG_STYLE", "auto");

    Builder::from_env(env).init();
}


/// CLI Visual Adjustments
/// Customizable Logging and Display

fn display_welcome(project_name: &str, description: &str) {
    let welcome_message = r#"
         ____   _____   _____  ____   _   _
    / __ \ |  __ \ |_   _|/ __ \ | \ | |
   | |  | || |__) |  | | | |  | ||  \| |
   | |  | ||  _  /   | | | |  | || . ` |
   | |__| || | \ \  _| |_| |__| || |\  |
    \____/ |_|  \_\|_____|\____/ |_| \_|
     _____  _       _____
     | |     | |       | |
    / ____|| |     |_   _|
   | |     | |       | |
   | |____ | |____  _| |_
    \_____||______||_____| "

:: {project_name} ::
{description}

    ######### WELCOME ##########
"#;
    println!("{}", welcome_message.replace("{project_name}", project_name).replace("{description}", description, colorize_output(welcome_message, "green", project_name="yellow", description="yellow"), bold_heading(welcome_message)));
}

pub fn colorize_output(output: &str, color: &str) -> ColoredString {
    match color {
        "error" => output.red(),
        "info" => output.green(),
        "debug" => output.blue(),
        "critical" => output.yellow(),
        _ => output.normal(),
    }
}

pub fn bold_heading(heading: &str) -> ColoredString {
    heading.bold()

}

/// Flags and Arguments
/// use clap::{App, Arg};
/// use thiserror:Error;
/// 

pub fn get_matches() -> clap::ArgMatches {
    App::new("Custom CLI")
        .version("1.0")
        .author("Snyata <core@synavate.tech>")
        .about("Customizes and colorizes your command line")
        .arg(Arg::new("main")
            .short("-m")
            .long("--main")
            .takes_value(false)
            .help("Call the main function of the project"))
        .arg(Arg::new("yes")
            .short("-y")
            .long("--yes")
            .takes_value(false)
            .help("Automatically answer yes to options"))
        .arg(Arg::new("verbose")
            .short("-v")
            .long("--verbose")
            .takes_value(false)
            .help("Sets the level of verbosity"))
        .arg(Arg::new("help")
            .short("-h")
            .long("--help")
            .takes_value(false)
            .help("Prints help information"))
        .get_matches()
}

/// Error Handling
/// use thiserror::Error;
/// use log::{debug, error, info, Level};

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Operation Failed: {0}")]
    Error(String),
    #[error("Fatal Error: {0}")]
    Fatal(String),
}

pub enum LogLevel {
    Debug,
    Info,
    Error,
}

impl LogLevel {
    fn to_string(&self) -> &str {
        match self {
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Error => "error",
        }
    }
}

pub fn handle_logs(level: LogLevel, message: &str) {
    match level {
        LogLevel::Debug => {
            if log_enabled!(Level::Debug) {
                debug!("{}", message);
            }
        }
        LogLevel::Info => {
            if log_enabled!(Level::Info) {
                info!("{}", message);
            }
        }
        LogLevel::Error => {
            if log_enabled!(Level::Error) {
                error!("{}", message);
            }
        }
    }
}

pub fn orion_main(project_name: &str, description: &str, main_function: fn()) -> Result<(), CliError> {
    display_welcome(project_name, description);
    let matches = get_matches();

    if matches.is_present("main") {
        main_function();
    }

    Ok(())
}
    



    


