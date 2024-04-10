#![allow(dead_code)]

extern crate html_escape;
extern crate levenshtein;
extern crate pyo3;
extern crate similar;
extern crate unicode_width;

use html_escape::encode_text;
use levenshtein::levenshtein;
use pyo3::prelude::*;
use similar::{Change, ChangeTag, TextDiff};
use unicode_width::UnicodeWidthStr;

struct Line {
    lineno: usize,
    value: String,
}
struct LineDiff {
    left: Option<Line>,
    right: Option<Line>,
}
enum Part {
    Equal(String),
    Delete(String),
    Insert(String),
}
struct Parts {
    lineno: usize,
    parts: Vec<Part>,
}
struct PartsDiff {
    left: Option<Parts>,
    right: Option<Parts>,
}
impl Part {
    fn width(&self) -> usize {
        match self {
            Part::Equal(s) | Part::Delete(s) | Part::Insert(s) => {
                UnicodeWidthStr::width(s.as_str())
            }
        }
    }
}

enum Side {
    Both(Line, Line),
    Left(Line),
    Right(Line),
}
fn side(change: &Change<&str>) -> Side {
    let value = String::from(change.value().trim_end_matches('\n'));
    match (change.old_index(), change.new_index()) {
        (Some(index_l), Some(index_r)) => Side::Both(
            Line {
                lineno: index_l + 1,
                value: value.clone(),
            },
            Line {
                lineno: index_r + 1,
                value,
            },
        ),
        (Some(index_l), None) => Side::Left(Line {
            lineno: index_l + 1,
            value,
        }),
        (None, Some(index_r)) => Side::Right(Line {
            lineno: index_r + 1,
            value,
        }),
        _ => unreachable!("At least one index should be Some"),
    }
}
// Parts have self-mutating methods to push to them
impl Parts {
    fn push_equal(&mut self, value: &str) {
        match &mut self.parts.last_mut() {
            Some(Part::Equal(s)) => {
                s.push_str(value);
            }
            _ => {
                let part = Part::Equal(String::from(value));
                self.parts.push(part)
            }
        }
    }
    fn push_delete(&mut self, value: &str) {
        match &mut self.parts.last_mut() {
            Some(Part::Delete(s)) => {
                s.push_str(value);
            }
            _ => {
                let part = Part::Delete(String::from(value));
                self.parts.push(part)
            }
        }
    }
    fn push_insert(&mut self, value: &str) {
        match &mut self.parts.last_mut() {
            Some(Part::Insert(s)) => {
                s.push_str(value);
            }
            _ => {
                let part = Part::Insert(String::from(value));
                self.parts.push(part)
            }
        }
    }
    fn width(&self) -> usize {
        self.parts.iter().fold(0, |acc, x| acc + x.width())
    }
}
// Make an inline diff, then convert to side-by-side
fn diff_lines(a: &Line, b: &Line) -> (Option<Parts>, Option<Parts>) {
    let diff = TextDiff::from_chars(&a.value, &b.value);
    let mut x = Parts {
        lineno: a.lineno,
        parts: Vec::new(),
    };
    let mut y = Parts {
        lineno: b.lineno,
        parts: Vec::new(),
    };
    for change in diff.iter_all_changes() {
        let value = change.value();
        match (change.tag(), side(&change)) {
            (ChangeTag::Equal, Side::Both(_, _)) => {
                x.push_equal(value);
                y.push_equal(value);
            }
            (ChangeTag::Delete, Side::Left(_)) => x.push_delete(value),
            (ChangeTag::Delete, Side::Right(_)) => y.push_delete(value),
            (ChangeTag::Insert, Side::Left(_)) => x.push_insert(value),
            (ChangeTag::Insert, Side::Right(_)) => y.push_insert(value),
            _ => unreachable!("Unexpected tag, side"),
        }
    }
    (Some(x), Some(y))
}
// Convert a lines-level diff to a parts-level diff
fn convert_diff(diff: &LineDiff) -> PartsDiff {
    match (&diff.left, &diff.right) {
        (None, Some(right)) => PartsDiff {
            left: None,
            right: Some(Parts {
                lineno: right.lineno,
                parts: vec![Part::Insert(right.value.clone())],
            }),
        },
        (Some(left), None) => PartsDiff {
            left: Some(Parts {
                lineno: left.lineno,
                parts: vec![Part::Delete(left.value.clone())],
            }),
            right: None,
        },
        (Some(left), Some(right)) => {
            let (x, y) = diff_lines(left, right);
            PartsDiff { left: x, right: y }
        }
        _ => panic!("Invalid LineDiff structure"),
    }
}
// Are two lines similar
fn similar(a: &String, b: &String) -> bool {
    let distance = levenshtein(a.as_str(), b.as_str());
    distance < 5 || (distance as f64 / (a.len() + b.len()) as f64) < 0.3
}

fn find_hole(diffs: &Vec<LineDiff>, left: bool, value: &String) -> Option<usize> {
    // Iterate backwards finding a hole
    let right = !left;
    let most_recent_hole = diffs
        .iter()
        .enumerate()
        .rev()
        // stop iterating as soon as we encounter a non-hole
        .take_while(|(_, diff)| (left && diff.left.is_none()) || (right && diff.right.is_none()))
        .last()
        .map(|(i, _)| i)
        .unwrap_or(diffs.len());

    // Starting at the most recent hole, iterate forwards to find a hole
    // where the opposite side looks similar.
    for (i, diff) in diffs.iter().enumerate().skip(most_recent_hole) {
        let existing = match (left, &diff.left, &diff.right) {
            (true, Some(_), _) => return None,
            (false, _, Some(_)) => return None,
            (true, _, Some(right)) => right,
            (false, Some(left), _) => left,
            _ => unreachable!("At least one side should be Some"),
        };
        if similar(&existing.value, value) {
            return Some(i);
        }
    }
    None
}

fn diff_a_and_b(a: &String, b: &String, context_lines: Option<usize>) -> Vec<PartsDiff> {
    let diff = TextDiff::from_lines(a, b);
    let mut unified = diff.unified_diff();
    context_lines.map(|c| unified.context_radius(c));

    let mut diffs: Vec<LineDiff> = Vec::new();

    for hunk in unified.iter_hunks() {
        for change in hunk.iter_changes() {
            match side(&change) {
                Side::Both(line_l, line_r) => {
                    let diff = LineDiff {
                        left: Some(line_l),
                        right: Some(line_r),
                    };
                    diffs.push(diff);
                }
                Side::Left(line) => {
                    let hole = find_hole(&diffs, true, &line.value);
                    if hole.is_some() {
                        diffs[hole.unwrap()].left = Some(line);
                    } else {
                        diffs.push(LineDiff {
                            left: Some(line),
                            right: None,
                        });
                    }
                }
                Side::Right(line) => {
                    let hole = find_hole(&diffs, false, &line.value);
                    if hole.is_some() {
                        diffs[hole.unwrap()].right = Some(line);
                    } else {
                        diffs.push(LineDiff {
                            left: None,
                            right: Some(line),
                        });
                    }
                }
            }
        }
    }
    diffs.iter().map(convert_diff).collect()
}

fn generate_html(diff: &Vec<PartsDiff>) -> String {
    let mut html = String::new();
    let mut left = String::new();
    let mut right = String::new();
    html.push_str("<div>");
    html.push_str("<style>");
    html.push_str(
        ".ocdiff-container { display: flex; background-color: #141414; color: #acacac; }",
    );
    html.push_str(".ocdiff-side { width: 50%; overflow-x: auto; margin: 0; padding: 1rem; }");
    html.push_str(".ocdiff-delete { color: red; }");
    html.push_str(".ocdiff-insert { color: green; }");
    html.push_str("</style>");
    html.push_str("<div class=\"ocdiff-container\">");

    for line_diff in diff {
        left.push_str(generate_html_parts(&line_diff.left).as_str());
        left.push_str("\n");
        right.push_str(generate_html_parts(&line_diff.right).as_str());
        right.push_str("\n");
    }

    html.push_str("<pre class=\"ocdiff-side\">");
    html.push_str(left.as_str());
    html.push_str("</pre>");
    html.push_str("<pre class=\"ocdiff-side\">");
    html.push_str(right.as_str());
    html.push_str("</pre>");

    html.push_str("</div>");
    html.push_str("</div>");
    html
}

fn generate_html_parts(parts_diff: &Option<Parts>) -> String {
    match parts_diff {
        None => String::from("<span class=\"ocdiff-line ocdiff-none\"></span>"),
        Some(p) => p
            .parts
            .iter()
            .map(|part| match part {
                Part::Equal(text) => ("ocdiff-equal", text),
                Part::Delete(text) => ("ocdiff-delete", text),
                Part::Insert(text) => ("ocdiff-insert", text),
            })
            .map(|(class, text)| {
                format!(
                    "<span class=\"ocdiff-line {}\">{}</span>",
                    class,
                    encode_text(&text)
                )
            })
            .collect::<String>(),
    }
}

#[pymodule]
#[pyo3(name = "ocdiff")]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "html_diff")]
    fn html_diff<'a>(
        _py: Python<'a>,
        a: String,
        b: String,
        context_lines: Option<usize>,
        column_limit: Option<usize>,
    ) -> PyResult<String> {
        let line_parts_diffs = diff_a_and_b(&a, &b, context_lines);
        column_limit.unwrap_or(80);
        let html = generate_html(&line_parts_diffs);
        Ok(html)
    }

    Ok(())
}
