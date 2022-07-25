use apollo_compiler::ApolloDiagnostic;
use buildstructor::buildstructor;
use miette::Severity;
use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    io,
    str::{self, FromStr},
};

/// A single diagnostic emitted from the [`ApolloCompiler`]
#[derive(Serialize, Deserialize)]
pub struct AwcDiagnostic {
    /// The type of diagnostic that was produced
    code: Option<String>,

    /// Labels annotating the GraphQL document
    labels: Option<Vec<AwcLabel>>,

    /// A help message
    help: Option<String>,

    /// The severity of the diagnostic
    severity: AwcDiagnosticSeverity,

    /// The URL of the diagnostic
    url: Option<String>,

    /// Other unknown fields
    #[serde(flatten)]
    other: Option<serde_json::Value>,
}

impl AwcDiagnostic {
    /// Get the severity of a diagnostic
    pub fn severity(&self) -> AwcDiagnosticSeverity {
        self.severity.clone()
    }
}

impl From<&ApolloDiagnostic> for AwcDiagnostic {
    fn from(diagnostic: &ApolloDiagnostic) -> Self {
        let report = diagnostic.report();
        let help = report.help().map(|h| h.to_string());
        let severity = report
            .severity()
            .map(AwcDiagnosticSeverity::from)
            .unwrap_or(AwcDiagnosticSeverity::Other);
        let url = report.url().map(|u| u.to_string());
        let code = report.code().map(|c| c.to_string());
        let labels = if let Some(dl) = report.labels() {
            let mut labels = Vec::new();
            for l in dl {
                let label_builder = AwcLabel::builder();
                let label = if let Some(label) = l.label() {
                    label_builder.label(label).build()
                } else {
                    label_builder.build()
                };
                if let Some(label) = label {
                    labels.push(label);
                }
            }
            if labels.is_empty() {
                None
            } else {
                Some(labels)
            }
        } else {
            None
        };

        Self {
            code,
            labels,
            url,
            help,
            severity,
            other: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
/// The level at which `AwcCompiler::validate` will fail
pub enum AwcDiagnosticSeverity {
    /// An `error` diagnostic, something went wrong
    Error,

    /// A `warning` diagnostic, something you might want to know
    Warning,

    /// An `advice` diagnostic, a helpful tip
    Advice,

    /// An `unknown` diagnostic, shouldn't appear in practice
    Other,
}

impl AwcDiagnosticSeverity {
    /// Enumerates the possible [`AwcDiagnosticKind`]s
    pub fn possible_values() -> Vec<AwcDiagnosticSeverity> {
        vec![Self::Error, Self::Warning, Self::Advice, Self::Other]
    }
}

impl Display for AwcDiagnosticSeverity {
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

impl FromStr for AwcDiagnosticSeverity {
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

impl From<Severity> for AwcDiagnosticSeverity {
    fn from(severity: Severity) -> Self {
        match severity {
            Severity::Advice => Self::Advice,
            Severity::Warning => Self::Warning,
            Severity::Error => Self::Error,
        }
    }
}

impl From<&ApolloDiagnostic> for AwcDiagnosticSeverity {
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

#[derive(Serialize, Deserialize)]
pub struct AwcLabel {
    label: Option<String>,
    span: Option<AwcSpan>,
    #[serde(flatten, skip_deserializing)]
    other: Option<serde_json::Value>,
}

#[buildstructor]
impl AwcLabel {
    #[builder]
    pub fn new(
        label: Option<String>,
        length: Option<usize>,
        offset: Option<usize>,
    ) -> Option<Self> {
        if label.is_none() && length.is_none() && offset.is_none() {
            None
        } else {
            let span_builder = AwcSpan::builder();

            let span = match (length, offset) {
                (Some(l), Some(o)) => span_builder.length(l).offset(o).build(),
                (Some(l), None) => span_builder.length(l).build(),
                (None, Some(o)) => span_builder.offset(o).build(),
                (None, None) => span_builder.build(),
            };

            Some(Self {
                label,
                span,
                other: None,
            })
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AwcSpan {
    length: Option<usize>,
    offset: Option<usize>,
    #[serde(flatten, skip_deserializing)]
    other: Option<serde_json::Value>,
}

#[buildstructor]
impl AwcSpan {
    #[builder]
    pub fn new(length: Option<usize>, offset: Option<usize>) -> Option<Self> {
        if length.is_none() && offset.is_none() {
            None
        } else {
            Some(Self {
                length,
                offset,
                other: None,
            })
        }
    }
}
