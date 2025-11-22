use std::path::PathBuf;

use timetable_core::parser::parse_pdf;

#[test]
fn synthetic_pdf_parses_expected_week() {
    let fixture: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("test/fixtures/synthetic_timetable.pdf");
    assert!(fixture.exists(), "fixture missing: {}", fixture.display());

    let weeks = parse_pdf(&fixture).expect("synthetic PDF should parse");
    assert_eq!(weeks.len(), 1, "expected exactly one week");

    let week = &weeks[0];
    assert_eq!(week.week_name, "Week 1");
    assert_eq!(week.student_name.as_deref(), Some("Alex Testington"));
    assert_eq!(week.form.as_deref(), Some("11XX"));
    assert!(
        !week.lessons.is_empty(),
        "expected at least one parsed lesson"
    );

    let has_math = week
        .lessons
        .iter()
        .any(|lesson| lesson.subject.contains("Mathematics"));
    assert!(has_math, "expected a Mathematics lesson in synthetic PDF");
}
