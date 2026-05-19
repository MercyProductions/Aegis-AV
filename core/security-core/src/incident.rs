use crate::process_tree::ProcessNode;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ThreatTimelineEvent {
    pub time: String,
    pub event: String,
    pub details: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IncidentReport {
    pub incident_id: String,
    pub created_at: String,
    pub detection_name: String,
    pub severity: String,
    pub file_path: PathBuf,
    pub sha256: String,
    pub process_tree: Option<ProcessNode>,
    pub timeline: Vec<ThreatTimelineEvent>,
    pub matched_signatures: Vec<String>,
    pub matched_rules: Vec<String>,
    pub heuristic_reasons: Vec<String>,
    pub user_action_taken: String,
    pub recommended_next_steps: Vec<String>,
}

pub struct IncidentReportExporter;

impl IncidentReportExporter {
    pub fn export_json(report: &IncidentReport) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    pub fn export_html(report: &IncidentReport) -> String {
        let timeline = report
            .timeline
            .iter()
            .map(|event| {
                format!(
                    "<li><strong>{}</strong> {}<span>{}</span></li>",
                    escape_html(&event.time),
                    escape_html(&event.event),
                    escape_html(&event.details)
                )
            })
            .collect::<Vec<_>>()
            .join("");
        let recommendations = report
            .recommended_next_steps
            .iter()
            .map(|step| format!("<li>{}</li>", escape_html(step)))
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Aegis Incident Report {incident_id}</title>
  <style>
    body {{ font-family: Segoe UI, Arial, sans-serif; background: #07111f; color: #e7f2ff; margin: 0; padding: 32px; }}
    main {{ max-width: 980px; margin: 0 auto; border: 1px solid #5f7890; padding: 28px; }}
    h1 {{ margin-top: 0; }}
    dl {{ display: grid; grid-template-columns: 170px 1fr; gap: 10px; }}
    dt {{ color: #91a8bd; }}
    dd {{ margin: 0; word-break: break-all; }}
    li {{ margin: 10px 0; }}
    li span {{ display: block; color: #9fb5ca; }}
  </style>
</head>
<body>
  <main>
    <h1>{detection_name}</h1>
    <dl>
      <dt>Incident</dt><dd>{incident_id}</dd>
      <dt>Severity</dt><dd>{severity}</dd>
      <dt>File</dt><dd>{file_path}</dd>
      <dt>SHA256</dt><dd>{sha256}</dd>
      <dt>User Action</dt><dd>{user_action}</dd>
    </dl>
    <h2>Timeline</h2>
    <ol>{timeline}</ol>
    <h2>Recommended Next Steps</h2>
    <ul>{recommendations}</ul>
  </main>
</body>
</html>"#,
            incident_id = escape_html(&report.incident_id),
            detection_name = escape_html(&report.detection_name),
            severity = escape_html(&report.severity),
            file_path = escape_html(&report.file_path.display().to_string()),
            sha256 = escape_html(&report.sha256),
            user_action = escape_html(&report.user_action_taken),
        )
    }

    pub fn export_pdf(report: &IncidentReport) -> Vec<u8> {
        let mut lines = vec![
            "Aegis Incident Report".to_string(),
            format!("Incident: {}", report.incident_id),
            format!("Detection: {}", report.detection_name),
            format!("Severity: {}", report.severity),
            format!("File: {}", report.file_path.display()),
            format!("SHA256: {}", report.sha256),
            format!("Action: {}", report.user_action_taken),
            "Timeline:".to_string(),
        ];
        lines.extend(
            report
                .timeline
                .iter()
                .map(|event| format!("{} - {} - {}", event.time, event.event, event.details)),
        );
        lines.push("Recommended next steps:".to_string());
        lines.extend(
            report
                .recommended_next_steps
                .iter()
                .map(|step| format!("- {step}")),
        );

        simple_pdf(&lines)
    }
}

fn simple_pdf(lines: &[String]) -> Vec<u8> {
    let escaped_lines = lines
        .iter()
        .take(42)
        .enumerate()
        .map(|(index, line)| {
            let y = 760_i32.saturating_sub(index as i32 * 16);
            format!("BT /F1 10 Tf 42 {y} Td ({}) Tj ET", escape_pdf_text(line))
        })
        .collect::<Vec<_>>()
        .join("\n");
    let stream = escaped_lines;

    let objects = [
        "1 0 obj << /Type /Catalog /Pages 2 0 R >> endobj".to_string(),
        "2 0 obj << /Type /Pages /Kids [3 0 R] /Count 1 >> endobj".to_string(),
        "3 0 obj << /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >> endobj".to_string(),
        "4 0 obj << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> endobj".to_string(),
        format!("5 0 obj << /Length {} >> stream\n{}\nendstream endobj", stream.len(), stream),
    ];

    let mut pdf = b"%PDF-1.4\n".to_vec();
    let mut offsets = Vec::new();
    for object in objects {
        offsets.push(pdf.len());
        pdf.extend_from_slice(object.as_bytes());
        pdf.push(b'\n');
    }
    let xref_start = pdf.len();
    pdf.extend_from_slice(
        format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes(),
    );
    for offset in offsets {
        pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
    }
    pdf.extend_from_slice(
        format!(
            "trailer << /Size {} /Root 1 0 R >>\nstartxref\n{xref_start}\n%%EOF\n",
            6
        )
        .as_bytes(),
    );
    pdf
}

fn escape_html(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

fn escape_pdf_text(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('(', "\\(")
        .replace(')', "\\)")
        .chars()
        .filter(|ch| ch.is_ascii() && !ch.is_control())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exports_json_html_and_pdf() {
        let report = sample_report();

        let json = IncidentReportExporter::export_json(&report).expect("json");
        let html = IncidentReportExporter::export_html(&report);
        let pdf = IncidentReportExporter::export_pdf(&report);

        assert!(json.contains("Test.EICAR.Signature"));
        assert!(html.contains("<!doctype html>"));
        assert!(pdf.starts_with(b"%PDF-1.4"));
    }

    fn sample_report() -> IncidentReport {
        IncidentReport {
            incident_id: "inc_test".to_string(),
            created_at: "2026-05-18T12:00:00Z".to_string(),
            detection_name: "Test.EICAR.Signature".to_string(),
            severity: "high".to_string(),
            file_path: PathBuf::from("C:/Users/User/Downloads/eicar.com.txt"),
            sha256: "275a021bbfb6489e54d471899f7db9d1663fc695ec2fe2a2c4538aabf651fd0f".to_string(),
            process_tree: None,
            timeline: vec![ThreatTimelineEvent {
                time: "10:02".to_string(),
                event: "File scanned".to_string(),
                details: "Hash signature matched".to_string(),
            }],
            matched_signatures: vec!["AEGIS-TEST-0001".to_string()],
            matched_rules: vec!["SIG-KNOWN-BAD-HASH".to_string()],
            heuristic_reasons: vec!["Known test signature".to_string()],
            user_action_taken: "quarantined".to_string(),
            recommended_next_steps: vec!["Review quarantine record".to_string()],
        }
    }
}
