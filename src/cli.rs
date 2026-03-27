use clap::{Parser, Subcommand, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum DatabaseOption {
    Postgres,
    SqlServer,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AuthenticationOption {
    Jwt,
    OAuth2,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum MiddlewareOption {
    RateLimiting,
    Caching,
    ResponseCompression,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ApiSurfaceOption {
    Swagger,
    Versioning,
    CORS,
}

#[derive(Parser)]
#[command(
    name = "forge",
    version = "0.1.0",
    about = "A .NET project scaffolding tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new project with optional database, authentication, middleware, and API surface configurations.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project to create
    /// * `db` - Optional database provider to use
    /// * `auth` - Optional authentication method to enable
    /// * `middleware` - Comma-separated list of middleware components to include
    /// * `surface` - Comma-separated list of API surfaces to expose
    New {
        // Name of Project
        name: String,

        // Database provider
        #[arg(long)]
        db: Option<DatabaseOption>,

        // Authentication
        #[arg(long)]
        auth: Option<AuthenticationOption>,

        // Middleware
        #[arg(long, value_delimiter = ',')]
        middleware: Vec<MiddlewareOption>,

        // API Exposure
        #[arg(long, value_delimiter = ',')]
        surface: Vec<ApiSurfaceOption>,
    },
    // For testing/verifying functionality in development
    Demo,
}
