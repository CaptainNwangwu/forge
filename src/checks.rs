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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_dotnet_returns_version() {
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
}
