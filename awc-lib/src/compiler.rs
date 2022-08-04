use apollo_compiler::ApolloCompiler;
use buildstructor::buildstructor;
use saucer::Timer;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tracing::info;

use crate::{AwcDiagnostic, AwcDiagnosticSeverity, AwcRules};

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
        fail_level: AwcDiagnosticSeverity,
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
        let mut error_count = 0;
        let mut warn_count = 0;
        let mut advice_count = 0;
        let mut diagnostics = Vec::new();
        let mut pretty = String::new();
        let mut success = true;
        let timer = Timer::start();
        let raw_diagnostics = self.compiler.validate();
        let elapsed = timer.stop();
        raw_diagnostics.iter().for_each(|diagnostic| {
            pretty.push_str(diagnostic.to_string().as_str());
            let diagnostic = AwcDiagnostic::from(diagnostic);
            let severity = diagnostic.severity();
            if !self.rules.is_ok(&severity) {
                success = false;
            }
            if !self.rules.should_ignore(&severity) {
                match severity {
                    AwcDiagnosticSeverity::Advice => {
                        advice_count += 1;
                    }
                    AwcDiagnosticSeverity::Error => {
                        error_count += 1;
                    }
                    AwcDiagnosticSeverity::Warning => {
                        warn_count += 1;
                    }
                    _ => error_count += 1,
                };

                diagnostics.push(diagnostic);
            }
        });

        if !pretty.is_empty() {
            pretty.push_str("\n");
        }
        let mut message = "".to_string();
        if success {
            message.push_str("🎉 Your GraphQL is looking great! ");
        }
        message.push_str(
            match (error_count > 0, warn_count > 0, advice_count > 0) {
                (true, true, true) => format!(
                    "❌ Found {} errors, {} warnings, and {} advice in {}.",
                    error_count, warn_count, advice_count, elapsed
                ),
                (true, true, false) => format!(
                    "❌ Found {} errors and {} warnings in {}.",
                    error_count, warn_count, elapsed
                ),
                (true, false, false) => format!("❌ Found {} errors in {}.", error_count, elapsed),
                (false, true, false) => format!("⚠️ Found {} warnings in {}.", warn_count, elapsed),
                (false, false, true) => format!("💡 Found {} advice in {}.", advice_count, elapsed),
                (false, true, true) => format!(
                    "⚠️ Found {} warnings and {} advice in {}.",
                    warn_count, advice_count, elapsed
                ),
                (false, false, false) => format!("Found no problems in {}.", elapsed),
                (true, false, true) => format!(
                    "❌ Found {} errors and {} advice in {}.",
                    error_count, advice_count, elapsed
                ),
            }
            .as_str(),
        );
        info!("{}", &message);
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
    /// Get an [`AwcResult`] in JSON form
    pub fn json(&self) -> Value {
        json!(self)
    }

    /// Get an [`AwcResult`] in pretty form (contains ANSI-escapes)
    pub fn pretty(&self) -> String {
        self.pretty.to_string()
    }
}
