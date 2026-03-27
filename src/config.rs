use crate::cli::{ApiSurfaceOption, AuthenticationOption, DatabaseOption, MiddlewareOption};

/// Configuration for scaffolding a new project.
///
/// This struct defines the core settings used to generate and initialize a new scaffold,
/// including project metadata, database configuration, security, and API structure.
///
/// # Fields
///
/// * `name` - The name of the scaffold project
/// * `database` - The database system to use (e.g., "postgres", "mysql", "sqlite")
/// * `authorization` - The authorization strategy to implement (e.g., "jwt", "oauth2", "session")
/// * `middleware` - A list of middleware components to apply to requests
/// * `api_surface` - A list of api exposure components to make external interaction easier (e.g. Swagger)
#[derive(Debug)]
pub struct ScaffoldConfig {
    pub name: String,
    pub database: Option<DatabaseOption>,
    pub authorization: Option<AuthenticationOption>,
    pub middleware: Vec<MiddlewareOption>,
    pub api_surface: Vec<ApiSurfaceOption>,
}
