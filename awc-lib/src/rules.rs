use buildstructor::buildstructor;

use crate::AwcDiagnosticKind;

/// Configures the behavior of [`AwcCompiler::validate`]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AwcRules {
    /// Do not emit [`AwcDiagnosticKind::Warn`]
    ignore_warnings: bool,

    /// Do not emit [`AwcDiagnosticKind::Advice`]
    ignore_advice: bool,

    /// Configures whether to fail on warnings or not
    fail_level: AwcDiagnosticKind,
}

#[buildstructor]
impl AwcRules {
    /// Create new [`AwcRules`]
    #[builder]
    pub fn new(ignore_warnings: bool, ignore_advice: bool, fail_level: AwcDiagnosticKind) -> Self {
        Self {
            ignore_warnings,
            ignore_advice,
            fail_level,
        }
    }

    /// Whether or not an [`ApolloDiagnostic`] constitutes a failure,
    /// configured by setting [`ApolloDiagnostic::fail_level`]
    pub fn is_ok(&self, diagnostic_kind: &AwcDiagnosticKind) -> bool {
        match (diagnostic_kind, &self.fail_level) {
            (AwcDiagnosticKind::Error, _) => false,
            (AwcDiagnosticKind::Warning, AwcDiagnosticKind::Error)
            | (AwcDiagnosticKind::Warning, AwcDiagnosticKind::Warning) => false,
            (AwcDiagnosticKind::Advice, AwcDiagnosticKind::Advice)
            | (AwcDiagnosticKind::Advice, AwcDiagnosticKind::Warning)
            | (AwcDiagnosticKind::Advice, AwcDiagnosticKind::Error) => false,
            _ => true,
        }
    }

    /// Whether or not an [`ApolloDiagnostic`] should be emitted,
    /// configured by setting [`AwcRules::ignore_warnings`]
    /// and/or [`AwcRules::ignore_advice`]
    pub fn should_ignore(&self, diagnostic_kind: &AwcDiagnosticKind) -> bool {
        match diagnostic_kind {
            AwcDiagnosticKind::Advice => self.ignore_advice,
            AwcDiagnosticKind::Warning => self.ignore_warnings,
            _ => false,
        }
    }
}
