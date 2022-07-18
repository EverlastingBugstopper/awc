pub use apollo_compiler::ApolloCompiler;
use miette::{GraphicalReportHandler, JSONReportHandler, Report};
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
        let json_handler = JSONReportHandler::new();
        let pretty_handler = GraphicalReportHandler::new();
        let mut diagnostics: Vec<Value> = Vec::new();
        let mut pretty = Vec::new();
        self.diagnostics.iter().for_each(|report| {
            let mut pretty_buffer = String::new();
            let _ = pretty_handler
                .render_report(&mut pretty_buffer, report.borrow())
                .map_err(|e| {
                    pretty_buffer.push_str("an unknown error occurred");
                    e
                });
            pretty.push(pretty_buffer);
            let mut json_buffer = String::new();
            let _ = json_handler
                .render_report(&mut json_buffer, report.borrow())
                .map_err(|e| {
                    json_buffer.push_str("an unknown error occurred");
                    e
                });
            let mut json: Value = serde_json::from_str(&json_buffer).unwrap();
            let obj = json.as_object_mut().unwrap();
            obj.remove_entry("filename");
            obj.remove_entry("related");
            diagnostics.push(json);
        });
        let success = self.success();
        let pretty = if success {
            "🎉 Your GraphQL is looking great!".to_string()
        } else {
            pretty.join(" ").to_string()
        };
        json!({
            "success": success,
            "diagnostics": diagnostics,
            "pretty": pretty
        })
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
