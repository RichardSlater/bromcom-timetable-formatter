use std::{collections::BTreeMap, path::Path};

use timetable_core::parser::parse_pdf;

const EXPECTED: &str = include_str!("../../../test/fixtures/wrapped-pdi-timetable.expected");

#[test]
fn wrapped_pdi_timetable_matches_every_expected_cell() {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("test/fixtures/wrapped-pdi-timetable.pdf");
    let weeks = parse_pdf(&fixture).expect("wrapped PDI fixture parses");

    let actual: BTreeMap<_, _> = weeks
        .iter()
        .enumerate()
        .flat_map(|(week_index, week)| {
            week.lessons.iter().map(move |lesson| {
                (
                    (week_index, lesson.day_index, lesson.period_index),
                    format!(
                        "{}|{}|{}|{}",
                        lesson.subject, lesson.class_code, lesson.room, lesson.teacher
                    ),
                )
            })
        })
        .collect();

    let expected: BTreeMap<_, _> = EXPECTED
        .lines()
        .map(|line| {
            let mut fields = line.split('|');
            let week = fields.next().unwrap().parse().unwrap();
            let day = fields.next().unwrap().parse().unwrap();
            let period = fields.next().unwrap().parse().unwrap();
            let value = fields.collect::<Vec<_>>().join("|");
            ((week, day, period), value)
        })
        .collect();

    assert_eq!(
        actual, expected,
        "every subject, group, room, and teacher must match"
    );
}
