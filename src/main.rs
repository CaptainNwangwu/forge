mod checks;
mod cli;
mod commands;
mod config;
mod theme;

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
            let input = ScaffoldConfig::new(name, db, auth, middleware, surface);

            commands::handle_new(input, &theme);
        }
        cli::Commands::Demo => {
            commands::handle_demo(&theme);
        }
    }
}
