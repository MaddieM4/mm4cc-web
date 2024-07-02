use gray_matter::{Matter, ParsedEntity, Pod};
use gray_matter::engine::YAML;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize, PartialEq, Debug)]
pub struct Metadata {
    pub title: String,
    pub visibility: String,
    pub template: String,
    pub glu_commands: Vec<String>,
}

#[derive(Serialize, PartialEq, Debug)]
pub struct MarkdownFile {
    pub meta: Metadata,
    pub html: String,
}

fn get_from_hm(hm: &HashMap<String, Pod>, field: &str, default: &str) -> String {
    hm.get(field)
        .and_then(|t| t.as_string().ok())
        .unwrap_or(default.to_string())
}

fn get_glu_commands(hm: &HashMap<String, Pod>) -> Vec<String> {
    if let Some(Pod::Array(v)) = hm.get("glu") {
        // v is a Vec<Pod>
        v.iter()
            .filter_map(|p| p.as_string().ok())
            .collect()
    } else {
        vec![]
    }
}

fn get_meta(result: ParsedEntity) -> Metadata {
    let default_title = "Default title";
    let default_vis = "private";
    let default_template = "index";
    match result.data {
        Some(Pod::Hash(data)) => Metadata {
            title: get_from_hm(&data, "title", default_title),
            visibility: get_from_hm(&data, "visibility", default_vis),
            template: get_from_hm(&data, "template", default_template),
            glu_commands: get_glu_commands(&data),
        },
        _ => Metadata {
            title: default_title.into(),
            visibility: default_vis.into(),
            template: default_template.into(),
            glu_commands: vec![],
        }
    }
}

pub fn parse(input: &str) -> MarkdownFile {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(input);
    let md_html = markdown::to_html(&result.content);
    return MarkdownFile {
        meta: get_meta(result),
        html: md_html,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn minimal() {
        let input = "Hello world!";
        let parsed = parse(&input);
        assert_eq!(parsed, MarkdownFile {
            meta: Metadata {
                title: "Default title".into(),
                visibility: "private".into(),
                template: "index".into(),
                glu_commands: vec![],
            },
            html: "<p>Hello world!</p>".into(),
        });
    }

    #[test]
    fn with_formatting() {
        let input = indoc! {"
            This _text_ is **formatted.**
            It's also multi-line.

            This should definitely be a fresh paragraph.
        "};
        let parsed = parse(&input);
        assert_eq!(parsed, MarkdownFile {
            meta: Metadata {
                title: "Default title".into(),
                visibility: "private".into(),
                template: "index".into(),
                glu_commands: vec![],
            },
            html: indoc! {"
                <p>This <em>text</em> is <strong>formatted.</strong>
                It's also multi-line.</p>
                <p>This should definitely be a fresh paragraph.</p>"
            }.into(),
        });
    }

    #[test]
    fn with_frontmatter() {
        let input = indoc! {"
            ---
            title: A simple little title.
            template: some_template
            ---

            Normal *contents*.
        "};
        let parsed = parse(&input);
        assert_eq!(parsed, MarkdownFile {
            meta: Metadata {
                title: "A simple little title.".into(),
                visibility: "private".into(),
                template: "some_template".into(),
                glu_commands: vec![],
            },
            html: "<p>Normal <em>contents</em>.</p>".into(),
        });
    }

    #[test]
    fn glu_commands() {
        let input = indoc! {"
            ---
            title: A simple little title.
            glu:
             - bash
             - node hello.js
             - 5
             - '16'
             - null
             - tree
            ---

            Normal *contents*.
        "};
        let parsed = parse(&input);
        assert_eq!(parsed, MarkdownFile {
            meta: Metadata {
                title: "A simple little title.".into(),
                visibility: "private".into(),
                template: "index".into(),
                glu_commands: vec![
                    "bash".to_string(),
                    "node hello.js".to_string(),
                    "16".to_string(),
                    "tree".to_string(),
                ],
            },
            html: "<p>Normal <em>contents</em>.</p>".into(),
        });
    }
}
