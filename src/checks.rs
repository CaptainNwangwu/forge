use std::process::Command;
/// The function `verify_dotnet` checks for the presence of the dotnet command and returns its version
/// if found.
///
/// Returns:
///
/// The `verify_dotnet` function returns a `Result<String, String>`. If the `dotnet --version` command
/// is successful, it will return the version string as a `String`. If the command fails, it will return
/// an error message as a `String`.
pub fn verify_dotnet() -> Result<String, String> {
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
