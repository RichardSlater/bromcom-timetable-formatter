use serde::Deserialize;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("TOML parsing error: {0}")]
    Toml(#[from] toml::de::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_style_for_room_longest_prefix() {
        let toml = r###"
            [[mappings]]
            prefix = "M"
            bg_color = "#fff"
            fg_color = "#000"
            map_id = "M_rooms"

            [[mappings]]
            prefix = "MA"
            bg_color = "#abc"
            fg_color = "#111"
            map_id = "MA_rooms"
        "###;

        let cfg: Config = toml::from_str(toml).unwrap();
        let m = cfg.get_style_for_room("MA12").unwrap();
        assert_eq!(m.prefix, "MA");
        assert_eq!(m.bg_color, "#abc");
    }

    #[test]
    fn test_default_fg_color() {
        let toml = r###"
            [[mappings]]
            prefix = "EN"
            color = "#ddeeff"
            map_id = "EN_rooms"
        "###;

        let cfg: Config = toml::from_str(toml).unwrap();
        let m = cfg.get_style_for_room("EN4").unwrap();
        assert_eq!(m.fg_color, "#231f20");
    }

    #[test]
    fn test_apply_overrides_updates_lesson() {
        use crate::parser::{Lesson, Week};

        let lessons = vec![Lesson {
            subject: "Maths".into(),
            room: "MA3".into(),
            teacher: "Mr A".into(),
            class_code: "MA3".into(),
            day_index: 3,    // Thursday
            period_index: 1, // L1
        }];

        let mut weeks = vec![Week {
            lessons,
            week_name: "Week 1".into(),
            student_name: None,
            form: None,
        }];

        let toml = r###"
            mappings = []
            [[overrides]]
            week = 1
            day = "Thursday"
            period = "L1"
            room = "SC6"
            teacher = "Mr Test B"
        "###;

        let cfg: Config = toml::from_str(toml).unwrap();
        cfg.apply_overrides(&mut weeks);

        let lesson = &weeks[0].lessons[0];
        assert_eq!(lesson.room, "SC6");
        assert_eq!(lesson.teacher, "Mr Test B");
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mappings: Vec<Mapping>,
    #[serde(default)]
    pub overrides: Vec<Override>,
}

#[derive(Debug, Deserialize)]
pub struct Mapping {
    pub prefix: String,
    #[serde(alias = "color")]
    pub bg_color: String,
    #[serde(default = "default_fg_color")]
    pub fg_color: String,
    pub map_id: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Override {
    pub week: usize,    // Week number (1-based)
    pub day: String,    // "Monday", "Tuesday", etc.
    pub period: String, // "PD", "L1", "L2", etc.
    pub subject: Option<String>,
    pub room: Option<String>,
    pub teacher: Option<String>,
    pub class_code: Option<String>,
}

fn default_fg_color() -> String {
    "#231f20".to_string()
}

impl Config {
    pub fn load(path: &Path) -> Result<Self, ConfigError> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_style_for_room(&self, room_code: &str) -> Option<&Mapping> {
        // Find the longest matching prefix
        self.mappings
            .iter()
            .filter(|m| room_code.starts_with(&m.prefix))
            .max_by_key(|m| m.prefix.len())
    }

    pub fn apply_overrides(&self, weeks: &mut [crate::parser::Week]) {
        for override_rule in &self.overrides {
            // Find the target week (1-based index)
            if override_rule.week == 0 || override_rule.week > weeks.len() {
                eprintln!(
                    "Warning: Override week {} is out of range",
                    override_rule.week
                );
                continue;
            }

            let week = &mut weeks[override_rule.week - 1];

            // Parse day to index
            let day_index = match override_rule.day.to_lowercase().as_str() {
                "monday" | "mon" => 0,
                "tuesday" | "tue" => 1,
                "wednesday" | "wed" => 2,
                "thursday" | "thu" => 3,
                "friday" | "fri" => 4,
                _ => {
                    eprintln!("Warning: Unknown day '{}'", override_rule.day);
                    continue;
                }
            };

            // Parse period to index
            let period_index = match override_rule.period.to_uppercase().as_str() {
                "PD" => 0,
                "L1" => 1,
                "L2" => 2,
                "L3" => 3,
                "L4" => 4,
                "L5" => 5,
                _ => {
                    eprintln!("Warning: Unknown period '{}'", override_rule.period);
                    continue;
                }
            };

            // Find and update the lesson
            if let Some(lesson) = week
                .lessons
                .iter_mut()
                .find(|l| l.day_index == day_index && l.period_index == period_index)
            {
                if let Some(subject) = &override_rule.subject {
                    lesson.subject = subject.clone();
                }
                if let Some(room) = &override_rule.room {
                    lesson.room = room.clone();
                }
                if let Some(teacher) = &override_rule.teacher {
                    lesson.teacher = teacher.clone();
                }
                if let Some(class_code) = &override_rule.class_code {
                    lesson.class_code = class_code.clone();
                }
                println!(
                    "Applied override: Week {}, {}, {}",
                    override_rule.week, override_rule.day, override_rule.period
                );
            } else {
                eprintln!(
                    "Warning: No lesson found for Week {}, {}, {}",
                    override_rule.week, override_rule.day, override_rule.period
                );
            }
        }
    }
}
