use crate::checks::execute_checks;
use crate::config::ScaffoldConfig;
use crate::theme::Theme;

pub fn handle_new(config: ScaffoldConfig, theme: &Theme) {
    // Run Checks
    match execute_checks(&config.name, theme) {
        Ok(()) => println!(
            "{} {}",
            theme.icons.success,
            theme
                .colors
                .success
                .apply_to("Preflight checks successful!")
        ),
        Err(e) => {
            println!("{} {}", theme.colors.error.apply_to(theme.icons.error), e);
            std::process::exit(1);
        }
    }
}

pub fn handle_demo(theme: &Theme) {
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
