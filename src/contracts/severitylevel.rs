use serde::Serialize;
use crate::contracts::*;

// NOTE: This file was automatically generated.

/// Defines the level of severity for the event.
#[derive(Debug, Clone, Serialize)]
pub enum SeverityLevel {
    Verbose,
    Information,
    Warning,
    Error,
    Critical,
}