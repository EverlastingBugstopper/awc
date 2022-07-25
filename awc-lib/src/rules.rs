use buildstructor::buildstructor;

use crate::AwcDiagnosticSeverity;

/// Configures the behavior of [`AwcCompiler::validate`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AwcRules {
    /// Do not emit [`AwcDiagnosticKind::Warn`]
    ignore_warnings: bool,

    /// Do not emit [`AwcDiagnosticKind::Advice`]
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

    /// Whether or not an [`ApolloDiagnostic`] constitutes a failure,
    /// configured by setting [`ApolloDiagnostic::fail_level`]
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

    /// Whether or not an [`ApolloDiagnostic`] should be emitted,
    /// configured by setting [`AwcRules::ignore_warnings`]
    /// and/or [`AwcRules::ignore_advice`]
    pub fn should_ignore(&self, diagnostic_kind: &AwcDiagnosticSeverity) -> bool {
        match diagnostic_kind {
            AwcDiagnosticSeverity::Advice => self.ignore_advice,
            AwcDiagnosticSeverity::Warning => self.ignore_warnings,
            _ => false,
        }
    }
}
