mod checks;
mod cli;
mod config;

use checks::verify_dotnet;
use clap::Parser;
use cli::Cli;
use config::ScaffoldConfig;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::New {
            name,
            db,
            auth,
            middleware,
            surface,
        } => {
            let config = ScaffoldConfig {
                name,
                database: db,
                authorization: auth,
                middleware,
                api_surface: surface,
            };

            println!("{:#?}", config);

            match verify_dotnet() {
                Ok(version) => println!("Version: {}", version),
                Err(e) => println!("Error: {}", e),
            }
        }
    }
}
