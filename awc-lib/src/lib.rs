use apollo_compiler::ApolloCompiler;
use miette::{JSONReportHandler, Report};
use serde::{Deserialize, Serialize};

use std::{borrow::Borrow, fmt::Display, io};

/// Struct that validates GraphQL documents
pub struct AwcCompiler {
    compiler: ApolloCompiler,
    strict: bool,
}

impl AwcCompiler {
    pub fn new(input: &str, strict: bool) -> Self {
        Self {
            compiler: ApolloCompiler::new(input),
            strict,
        }
    }

    pub fn run(&self) -> AwcResult {
        let mut error_count = 0;
        let mut warn_count = 0;
        let mut advice_count = 0;
        let mut diagnostics = Vec::new();
        let mut pretty = String::new();
        let mut success = true;

        self.compiler.validate().iter().for_each(|diagnostic| {
            if diagnostic.is_error() {
                success = false;
                error_count += 1;
            } else if diagnostic.is_warning() {
                if self.strict {
                    success = false;
                    warn_count += 1
                }
            } else if diagnostic.is_advice() {
                advice_count += 1
            };

            pretty.push_str(&diagnostic.to_string());

            diagnostics.push(
                AwcDiagnostic::try_from(diagnostic.report())
                    .unwrap_or_else(|e| AwcDiagnostic::error(e)),
            );
        });

        if pretty.is_empty() {
            pretty.push_str("🎉 Your GraphQL is looking great!");
        }

        AwcResult {
            error_count,
            warn_count,
            advice_count,
            diagnostics,
            pretty,
            success,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AwcResult {
    success: bool,
    diagnostics: Vec<AwcDiagnostic>,
    pretty: String,
    error_count: usize,
    warn_count: usize,
    advice_count: usize,
}

impl AwcResult {
    pub fn error(message: impl Display) -> Self {
        let err = AwcDiagnostic::error(&message);
        let code = err.code.clone().unwrap();
        Self {
            success: false,
            diagnostics: vec![err],
            pretty: format!("{}\n{}", code, message),
            error_count: 1,
            warn_count: 0,
            advice_count: 0,
        }
    }

    /// Get an `AwcResult` in JSON form
    pub fn json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Get an `AwcResult` in pretty form (contains ANSI-escapes)
    pub fn pretty(&self) -> String {
        self.pretty.to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct AwcDiagnostic {
    pub(crate) code: Option<String>,
    labels: Option<Vec<AwcLabel>>,
    message: Option<String>,
    severity: Option<String>,
}

impl AwcDiagnostic {
    fn error(message: impl Display) -> Self {
        Self {
            code: Some("awc-diagnostic error".to_string()),
            message: Some(message.to_string()),
            severity: Some("error".to_string()),
            labels: None,
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
}

#[derive(Serialize, Deserialize)]
pub struct AwcSpan {
    length: Option<usize>,
    offset: Option<usize>,
}
