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
}
