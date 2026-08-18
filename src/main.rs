mod config;
mod network;
mod repl;
mod utility;

use config::*;
use network::*;
use repl::*;


fn fetch_dependencies() -> bool {
    let mut failed = false;

    for dep in Config::get_dependencies() {
        if !dep.path.exists() {
            println!("Fetching {}.", dep.name);
            match download_dep(&dep) {
                Ok(()) => println!("{} download.", dep.name),
                Err(e) => {
                    println!("Failed to fetch {}: {}", dep.name, e);

                    if !failed {
                        failed = true;
                    }
                }
            }
        }
    }

    !failed
}

fn main() -> Result<(), Box<dyn std::error::Error>>
{

    println!("Fetching dependencies.");
    if !fetch_dependencies() {
        println!("Error: failed to fetch all needed dependencies.");
        std::process::exit(1);
    } else {
        println!("All dependencies downloaded successfully.")
    }

    run_menu()?;

    Ok(())
}