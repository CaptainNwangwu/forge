use crate::theme::Theme;
use std::path::Path;
use std::process::Command;

/// The function `verify_dotnet` checks for the presence of the dotnet command and returns its version
/// if found.
///
/// Returns:
///
/// The `verify_dotnet` function returns a `Result<String, String>`. If the `dotnet --version` command
/// is successful, it will return the version string as a `String`. If the command fails, it will return
/// an error message as a `String`.
pub fn check_dotnet_version() -> Result<String, String> {
    let output = Command::new("dotnet")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let version = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        Ok(version)
    } else {
        Err(String::from(
            "Dotnet command not found. Check to ensure you have the dotnet sdk.",
        ))
    }
}

/// The function `check_directory_collision` checks if a directory with the given name already exists at
/// the current path.
///
/// Arguments:
///
/// * `name`: The function `check_directory_collision` takes a reference to a string `name` as input.
/// This function checks if a directory with the given name already exists at the current path. If the
/// directory exists, it returns an error message indicating the collision. Otherwise, it returns
/// Ok(()).
///
/// Returns:
///
/// The function `check_directory_collision` returns a `Result<(), String>`. If the directory with the
/// given name already exists, it returns an error message as a `String`. Otherwise, it returns `Ok(())`
/// indicating that there is no collision.
pub fn check_directory_collision(name: &str) -> Result<(), String> {
    //? Consider replacing `String` err type with an explicit error or anyhow::Error from anyhow crate.
    if Path::new(name).exists() {
        Err(format!(
            "Directory named {name} already exists at current path."
        ))
    } else {
        Ok(())
    }
}

/// The function `check_git_version` checks the installed version of Git and returns it if successful,
/// or an error message if Git is not found.
///
/// Returns:
///
/// The function `check_git_version()` returns a `Result<String, String>`. If the `git --version`
/// command is successful, it returns the Git version as a `String` inside an `Ok` variant. If the
/// command fails or if Git is not installed, it returns an error message as a `String` inside an `Err`
/// variant.

pub fn check_git_version() -> Result<String, String> {
    let output = Command::new("git")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let version = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        Ok(version)
    } else {
        Err(String::from(
            "Git command not found. Check to ensure you have git installed.",
        ))
    }
}

/// The `run_checks` function in Rust performs various checks and returns warnings and fatals based on
/// the results.
///
/// Arguments:
///
/// * `dir`: The `dir` parameter in the `run_checks` function represents the directory path that will be
/// checked for various conditions like dotnet version, directory collision, and git version.
pub fn run_checks(dir: &str) -> (Vec<String>, Vec<String>) {
    // (warnings, fatals)
    let mut warnings = Vec::new();
    let mut fatals = Vec::new();

    if let Err(e) = check_dotnet_version() {
        fatals.push(format!("Fatal: {}", e));
    }
    if let Err(e) = check_directory_collision(dir) {
        fatals.push(format!("Fatal: {}", e))
    }
    if let Err(e) = check_git_version() {
        warnings.push(format!("Warning: {}", e))
    }

    (warnings, fatals)
}

/// The function `execute_checks` runs preflight checks on a directory and prints warnings and errors
/// using a specified theme, returning an error message if any fatal errors occur.
///
/// Arguments:
///
/// * `dir`: The `dir` parameter in the `execute_checks` function is a reference to a string that
/// represents the directory path where the checks will be executed. This directory is where the
/// `run_checks` function will be called to perform the checks.
/// * `theme`: The `theme` parameter in the `execute_checks` function is a reference to a `Theme`
/// struct. This struct likely contains information such as colors, icons, or other styling elements
/// that are used to format and display warning and error messages during the execution of checks in the
/// specified directory.
///
/// Returns:
///
/// The function `execute_checks` returns a `Result<(), String>`. If there are no fatal errors (`fatals`
/// is empty), it returns `Ok(())`, indicating that the preflight checks were successful. If there are
/// fatal errors present, it returns an `Err` containing the message "Preflight checks failed".
pub fn execute_checks(dir: &str, theme: &Theme) -> Result<(), String> {
    let (warnings, fatals) = run_checks(dir);

    for w in &warnings {
        println!(
            "{} {}",
            theme.colors.warning.apply_to(theme.icons.warning),
            w
        );
    }

    for f in &fatals {
        println!("{} {}", theme.colors.error.apply_to(theme.icons.error), f);
    }

    if fatals.is_empty() {
        Ok(())
    } else {
        Err(String::from("Preflight checks failed"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dotnet_returns_version() {
        let result = check_dotnet_version();
        assert!(result.is_ok());
    }

    #[test]
    fn test_version_is_not_empty() {
        let version = check_dotnet_version().unwrap();
        assert!(!version.is_empty());
    }

    #[test]
    fn test_collision_returns_err() {
        let dir_name = "existing-dir";
        std::fs::create_dir(dir_name).unwrap();

        let err = check_directory_collision(dir_name).unwrap_err();
        assert!(err.contains("already exists"));

        std::fs::remove_dir(dir_name).unwrap();
    }

    #[test]
    fn test_no_collision_returns_ok() {
        let name = "missing-dir";
        if Path::new(name).exists() {
            std::fs::remove_dir(name).unwrap();
        }

        assert!(check_directory_collision(name).is_ok());
    }

    #[test]
    fn test_git_returns_version() {
        let result = check_git_version();
        assert!(result.is_ok());
    }

    #[test]
    fn test_git_version_not_empty() {
        let result = check_git_version().unwrap();
        assert!(!result.is_empty())
    }
}
