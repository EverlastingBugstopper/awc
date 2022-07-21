use apollo_compiler::ApolloDiagnostic;
use miette::{JSONReportHandler, Report};
use serde::{Deserialize, Serialize};
use std::{borrow::Borrow, fmt::Display, io, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct AwcDiagnostic {
    pub(crate) code: Option<String>,
    labels: Option<Vec<AwcLabel>>,
    message: Option<String>,
    severity: Option<String>,
    #[serde(flatten)]
    other: Option<serde_json::Value>,
}

impl AwcDiagnostic {
    pub(crate) fn error(message: impl Display) -> Self {
        Self {
            code: Some("awc-diagnostic error".to_string()),
            message: Some(message.to_string()),
            severity: Some("error".to_string()),
            labels: None,
            other: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
/// The level at which `AwcCompiler::validate` will fail
pub enum AwcDiagnosticKind {
    /// An `error` diagnostic, something went wrong
    Error,

    /// A `warning` diagnostic, something you might want to know
    Warning,

    /// An `advice` diagnostic, a helpful tip
    Advice,

    /// An `unknown` diagnostic, shouldn't appear in practice
    Other,
}

impl AwcDiagnosticKind {
    /// Enumerates the possible [`AwcDiagnosticKind`]s
    pub fn possible_values() -> Vec<AwcDiagnosticKind> {
        vec![Self::Error, Self::Warning, Self::Advice, Self::Other]
    }
}

impl Display for AwcDiagnosticKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match &self {
                Self::Error => "error",
                Self::Warning => "warn",
                Self::Advice => "advice",
                _ => "other",
            }
        )
    }
}

impl FromStr for AwcDiagnosticKind {
    type Err = io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "error" => Ok(Self::Error),
            "warning" | "warn" => Ok(Self::Warning),
            "advice" => Ok(Self::Advice),
            _ => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "valid inputs are 'error', 'warn', and 'advice'",
            )),
        }
    }
}

impl From<&ApolloDiagnostic> for AwcDiagnosticKind {
    fn from(diagnostic: &ApolloDiagnostic) -> Self {
        if diagnostic.is_error() {
            Self::Error
        } else if diagnostic.is_advice() {
            Self::Advice
        } else if diagnostic.is_warning() {
            Self::Warning
        } else {
            Self::Other
        }
    }
}

impl TryFrom<Report> for AwcDiagnostic {
    type Error = io::Error;

    fn try_from(report: Report) -> io::Result<Self> {
        let json_handler = JSONReportHandler::new();
        let mut json_buffer = String::new();
        let _ = json_handler
            .render_report(&mut json_buffer, report.borrow())
            .map_err(|e| {
                json_buffer.push_str("an unknown error occurred");
                e
            });
        serde_json::from_str(&json_buffer).map_err(|e| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("this diagnostic reported invalid JSON: {}", e),
            )
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct AwcLabel {
    label: Option<String>,
    span: Option<AwcSpan>,
    #[serde(flatten)]
    other: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct AwcSpan {
    length: Option<usize>,
    offset: Option<usize>,
    #[serde(flatten)]
    other: Option<serde_json::Value>,
}
