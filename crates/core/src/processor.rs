use regex::Regex;
use roxmltree::Document;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProcessorError {
    #[error("XML parsing error: {0}")]
    Xml(#[from] roxmltree::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

pub struct MapHighlight {
    pub id: String,
    pub color: String,
}

pub fn process_map(path: &Path, highlights: &[MapHighlight]) -> Result<String, ProcessorError> {
    let content = fs::read_to_string(path)?;
    let doc = Document::parse(&content)?;

    // We will collect replacements: (start_index, end_index, new_text)
    let mut replacements: Vec<(usize, usize, String)> = Vec::new();
    let fill_re = Regex::new(r#"fill\s*=\s*(?:"[^"]*"|'[^']*')"#)?;

    for highlight in highlights {
        // Find the node by id or data-name
        let node = doc.descendants().find(|n| {
            n.attribute("id") == Some(&highlight.id)
                || n.attribute("data-name") == Some(&highlight.id)
        });

        if let Some(group_node) = node {
            // Iterate over all descendants to find shapes with fill attributes
            for child in group_node.descendants() {
                // We only care about elements that have a 'fill' attribute
                if child.has_attribute("fill") {
                    let range = child.range();
                    // Find the end of the start tag.
                    if let Some(start_tag_end) = content[range.start..].find('>') {
                        let start_tag_str = &content[range.start..range.start + start_tag_end + 1];

                        if let Some(mat) = fill_re.find(start_tag_str) {
                            let absolute_start = range.start + mat.start();
                            let absolute_end = range.start + mat.end();
                            replacements.push((
                                absolute_start,
                                absolute_end,
                                format!("fill=\"{}\"", highlight.color),
                            ));
                        }
                    }
                }
            }
        }
    }

    // Apply replacements in reverse order
    replacements.sort_by(|a, b| b.0.cmp(&a.0));

    // Deduplicate based on start index to avoid conflicting writes if regions overlap
    replacements.dedup_by_key(|k| k.0);

    let mut result = content;
    for (start, end, text) in replacements {
        // Ensure we don't panic if indices are out of bounds
        if start <= end && end <= result.len() {
            result.replace_range(start..end, &text);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn process_map_replaces_fill() {
        let temp_dir = env::temp_dir();
        let file = temp_dir.join("test_map.svg");
        let content = r###"<?xml version="1.0"?>
<svg>
    <g id="Maths_Rooms">
        <path fill="#000000" d="M0" />
    </g>
    <g id="Other">
        <rect fill="#ffffff" />
    </g>
</svg>"###;

        std::fs::write(&file, content).unwrap();

        let highlights = vec![MapHighlight {
            id: "Maths_Rooms".into(),
            color: "#ff0000".into(),
        }];
        let out = process_map(&file, &highlights).unwrap();
        assert!(out.contains("fill=\"#ff0000\""));
    }
}
