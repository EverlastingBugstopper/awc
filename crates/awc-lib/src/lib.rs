pub use apollo_compiler::ApolloCompiler;
use miette::{JSONReportHandler, Report};
use serde_json::{json, Value};

use std::borrow::Borrow;

pub type NewAwcResult = Result<(), AwcDiagnostic>;

#[derive(Debug)]
pub struct AwcDiagnostic {
    diagnostics: Vec<Report>,
}

impl AwcDiagnostic {
    pub fn new(errors: Vec<Report>) -> Self {
        Self {
            diagnostics: errors,
        }
    }

    pub fn unrecoverable_failure() -> Self {
        Self::error("an unrecoverable error was encountered".to_string())
    }

    pub fn error(message: String) -> Self {
        Self {
            diagnostics: vec![Report::msg(message)],
        }
    }

    pub fn print(&self, is_json: bool) {
        if is_json {
            println!("{}", self.json())
        } else {
            self.diagnostics.iter().for_each(|error| {
                eprintln!("{:?}", error);
            })
        }
    }

    pub fn success(&self) -> bool {
        self.diagnostics.is_empty()
    }

    pub fn json(&self) -> Value {
        // let old_no_color = env::var_os("NO_COLOR");
        // env::set_var("NO_COLOR", "1");
        let handler = JSONReportHandler::new();
        let mut diagnostics: Vec<Value> = Vec::new();
        self.diagnostics.iter().for_each(|report| {
            let mut buffer = String::new();
            let _ = handler
                .render_report(&mut buffer, report.borrow())
                .map_err(|e| {
                    buffer.push_str("an unknown error occurred");
                    e
                });
            let mut json: Value = serde_json::from_str(&buffer).unwrap();
            let obj = json.as_object_mut().unwrap();
            obj.remove_entry("filename");
            obj.remove_entry("related");
            diagnostics.push(json);
        });
        json!({"success": self.success(), "diagnostics": diagnostics})
    }
}

impl From<ApolloCompiler> for AwcDiagnostic {
    fn from(compiler: ApolloCompiler) -> Self {
        let mut reports = Vec::new();
        let diagnostics = compiler.validate();
        diagnostics.iter().for_each(|diagnostic| {
            reports.push(diagnostic.report());
        });
        AwcDiagnostic::new(reports)
    }
}
