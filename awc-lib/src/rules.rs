use buildstructor::buildstructor;

use crate::AwcDiagnosticSeverity;

#[cfg(doc)]
use crate::AwcCompiler;

#[cfg(doc)]
use crate::AwcDiagnostic;

/// Configures the behavior of [`AwcCompiler::validate`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AwcRules {
    /// Do not emit [`AwcDiagnosticSeverity::Warning`]
    ignore_warnings: bool,

    /// Do not emit [`AwcDiagnosticSeverity::Advice`]
    ignore_advice: bool,

    /// Configures whether to fail on warnings or not
    fail_level: AwcDiagnosticSeverity,
}

#[buildstructor]
impl AwcRules {
    /// Create new [`AwcRules`]
    #[builder]
    pub fn new(
        ignore_warnings: bool,
        ignore_advice: bool,
        fail_level: AwcDiagnosticSeverity,
    ) -> Self {
        Self {
            ignore_warnings,
            ignore_advice,
            fail_level,
        }
    }

    /// Whether or not an [`AwcDiagnostic`] constitutes a failure based on the current [`AwcRules`]
    pub fn is_ok(&self, diagnostic_kind: &AwcDiagnosticSeverity) -> bool {
        match (diagnostic_kind, &self.fail_level) {
            // error/other diagnostics always fail
            (AwcDiagnosticSeverity::Error, _) | (AwcDiagnosticSeverity::Other, _) => false,

            // warning diagnostics fail when warning/error fail levels are set
            (AwcDiagnosticSeverity::Warning, AwcDiagnosticSeverity::Error)
            | (AwcDiagnosticSeverity::Warning, AwcDiagnosticSeverity::Warning) => false,

            // advice diagnostics fail when advice fail levels are set
            (AwcDiagnosticSeverity::Advice, AwcDiagnosticSeverity::Advice) => false,
            _ => true,
        }
    }

    /// Whether or not an [`AwcDiagnostic`] should be emitted,
    /// configured by calling `.ignore_warnings(true)`
    /// and/or `.ignore_advice(true)` on an [`AwcRules::builder`]
    pub fn should_ignore(&self, diagnostic_kind: &AwcDiagnosticSeverity) -> bool {
        match diagnostic_kind {
            AwcDiagnosticSeverity::Advice => self.ignore_advice,
            AwcDiagnosticSeverity::Warning => self.ignore_warnings,
            _ => false,
        }
    }
}
