use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};

pub fn parse(source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for (line_idx, line) in source.lines().enumerate() {
        if line.contains("??") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position::new(line_idx as u32, 0),
                    end: Position::new(line_idx as u32, line.len() as u32),
                },
                severity: Some(DiagnosticSeverity::ERROR),
                message: "Syntax error: unexpected '??'".into(),
                ..Default::default()
            });
        }
    }

    diagnostics
}
