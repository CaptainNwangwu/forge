mod checks;
mod cli;
mod config;
mod theme;

use checks::check_dotnet_version;
use clap::Parser;
use cli::Cli;
use config::ScaffoldConfig;
use theme::*;

fn main() {
    let theme = Theme::new();

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

            match check_dotnet_version() {
                Ok(version) => println!("Version: {}", theme.colors.success.apply_to(version)),
                Err(e) => println!("Error: {}", theme.colors.error.apply_to(e)),
            }
        }

        cli::Commands::Demo => {
            println!("{}", theme.colors.heading.apply_to("Forge v0.1.0"));
            println!(
                "{} Operation successful",
                theme.colors.success.apply_to(theme.icons.success)
            );
            println!(
                "{} Something to note",
                theme.colors.info.apply_to(theme.icons.info)
            );
            println!(
                "{} Proceed with caution",
                theme.colors.warning.apply_to(theme.icons.warning)
            );
            println!(
                "{} Something went wrong",
                theme.colors.error.apply_to(theme.icons.error)
            );
            println!("  path: {}", theme.colors.dim.apply_to("./my-api"));
            println!("{} forge", theme.icons.forge);
        }
    }
}
