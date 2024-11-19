use serde::{Deserialize, Serialize};

/// Commands issued by the client.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientCommand {
    /// Register a worker with the controller.
    RegisterRequest {
        /// Name of the machine. If none is given, the controller will assign one.
        alias: String,
        /// Maximum parallel jobs for this machine.
        jobs: usize,
    },
    /// Sumbit build results to the controller.
    FinalizeRequest {
        /// Name of the built package.
        package: String,
        /// Tarball of the package data.
        data: Vec<u8>,
    },
    /// Responds to `ServerCommand::StatusRequest`.
    StatusResponse {
        /// Package being built, if any.
        package: Option<String>,
        /// Progress of the current package.
        progress: f64,
    },
}

/// Commands issued by the server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerCommand {
    /// Build a package.
    RunRequest {
        /// The name of the package to build.
        package: String,
        /// Configuration of the
        config: BuildConfig,
        /// Tar of all package files.
        data: Vec<u8>,
    },
    /// Request a status of the current worker.
    StatusRequest,
    /// Returns an error to a recent request.
    ErrorResponse { code: ErrorCode },
    /// Drop a worker connection. This is non-negotiable.
    Drop,
}

/// Possible errors.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ErrorCode {
    AliasAlreadyInUse,
}

/// Options for a build target.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BuildConfig {
    /// Target architecture to build for.
    target_arch: String,
}
