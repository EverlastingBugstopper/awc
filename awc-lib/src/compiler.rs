use apollo_compiler::ApolloCompiler;
use buildstructor::buildstructor;
use saucer::Timer;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::{AwcDiagnostic, AwcDiagnosticKind, AwcRules};

/// Struct that validates GraphQL documents
/// Mostly just a wrapper around `ApolloCompiler`
/// that makes it into a public API.
/// It is not stable.
pub struct AwcCompiler {
    /// An `ApolloCompiler` to validate GraphQL documents
    compiler: ApolloCompiler,

    /// Rules that govern [`ApolloCompiler::validate`]
    /// and the [`ApolloDiagnostic`]s  they emit
    rules: AwcRules,
}

#[buildstructor]
impl AwcCompiler {
    /// Create a new [`AwcCompiler`]
    #[builder]
    pub fn new(
        input: String,
        ignore_warnings: bool,
        ignore_advice: bool,
        fail_level: AwcDiagnosticKind,
    ) -> Self {
        Self {
            compiler: ApolloCompiler::new(&input),
            rules: AwcRules::builder()
                .ignore_warnings(ignore_warnings)
                .ignore_advice(ignore_advice)
                .fail_level(fail_level)
                .build(),
        }
    }

    /// Consume the [`ApolloCompiler`] and produce an `AwcResult`
    /// based on the rules defined by [`AwcRules`]
    pub fn validate(&self) -> AwcResult {
        let timer = Timer::start();
        let mut error_count = 0;
        let mut warn_count = 0;
        let mut advice_count = 0;
        let mut diagnostics = Vec::new();
        let mut pretty = String::new();
        let mut success = true;
        log::debug!("started validation");
        let raw_diagnostics = self.compiler.validate();
        log::debug!("completed validation");
        let elapsed = timer.stop();
        raw_diagnostics.iter().for_each(|diagnostic| {
            let diagnostic_kind = AwcDiagnosticKind::from(diagnostic);
            log::debug!("{:?}", &diagnostic_kind);
            if !self.rules.is_ok(&diagnostic_kind) {
                success = false;
            }
            if !self.rules.should_ignore(&diagnostic_kind) {
                match diagnostic_kind {
                    AwcDiagnosticKind::Advice => {
                        advice_count += 1;
                    }
                    AwcDiagnosticKind::Error => {
                        error_count += 1;
                    }
                    AwcDiagnosticKind::Warning => {
                        warn_count += 1;
                    }
                    _ => (),
                };

                pretty.push_str(&diagnostic.to_string());
                diagnostics.push(
                    AwcDiagnostic::try_from(diagnostic.report())
                        .unwrap_or_else(|e| AwcDiagnostic::error(e)),
                );
            } else {
                log::debug!("ignored {:?}", diagnostic);
            }
        });

        if !pretty.is_empty() {
            pretty.push_str("\n");
        }
        let mut message = "".to_string();
        if success {
            message.push_str("🎉 Your GraphQL is looking great! ");
        } else {
            message.push_str("❌ ");
        }
        message.push_str(
            match (error_count > 0, warn_count > 0, advice_count > 0) {
                (true, true, true) => format!(
                    "Found {} errors, {} warnings, and {} advice in {}.",
                    error_count, warn_count, advice_count, elapsed
                ),
                (true, true, false) => format!(
                    "Found {} errors and {} warnings in {}.",
                    error_count, warn_count, elapsed
                ),
                (true, false, false) => format!("Found {} errors in {}.", error_count, elapsed),
                (false, true, false) => format!("Found {} warnings in {}.", warn_count, elapsed),
                (false, false, true) => format!("Found {} advice in {}.", advice_count, elapsed),
                (false, true, true) => format!(
                    "Found {} warnings and {} advice in {}.",
                    warn_count, advice_count, elapsed
                ),
                (false, false, false) => format!("Found no problems in {}.", elapsed),
                (true, false, true) => format!(
                    "Found {} errors and {} advice in {}.",
                    error_count, advice_count, elapsed
                ),
            }
            .as_str(),
        );
        pretty.push_str(&message);

        AwcResult {
            error_count,
            warn_count,
            advice_count,
            diagnostics,
            pretty,
            success,
            message,
            elapsed: Some(elapsed),
        }
    }
}

/// [`AwcResult`] is emitted when an [`ApolloCompiler`] is consumed in [`AwcCompiler::validate`]
#[derive(Serialize, Deserialize)]
pub struct AwcResult {
    success: bool,
    message: String,
    diagnostics: Vec<AwcDiagnostic>,
    pretty: String,
    error_count: usize,
    warn_count: usize,
    advice_count: usize,
    elapsed: Option<String>,
}

impl AwcResult {
    /// Create an adhoc `AwcResult` failure
    pub fn error(message: impl Display) -> Self {
        let err = AwcDiagnostic::error(&message);
        let code = err.code.clone().unwrap();
        Self {
            message: message.to_string(),
            success: false,
            diagnostics: vec![err],
            pretty: format!("{}\n{}", code, message),
            error_count: 1,
            warn_count: 0,
            advice_count: 0,
            elapsed: None,
        }
    }

    /// Get an [`AwcResult`] in JSON form
    pub fn json(&self) -> String {
        // this unwrap is fine, if [`ApolloCompiler`] starts to emit
        // JSON fields we didn't define here, we
        // we will capture them in the `.other` field with `#[serde(flatten)]`
        match serde_json::to_string(&self) {
            Ok(s) => s,
            Err(e) => Self::error(format!("this result reported invalid JSON: {}", e)).json(),
        }
    }

    /// Get an [`AwcResult`] in pretty form (contains ANSI-escapes)
    pub fn pretty(&self) -> String {
        self.pretty.to_string()
    }
}
