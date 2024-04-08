extern crate html_escape;
extern crate pyo3;
extern crate similar;
extern crate unicode_width;

use html_escape::encode_text;
use pyo3::prelude::*;
use similar::{ChangeTag, TextDiff};
use unicode_width::UnicodeWidthStr;

#[derive(Clone)]
enum Part {
    Equal(String),
    Delete(String),
    Insert(String),
}
struct PartsDiff {
    parts: Vec<Part>,
}
struct LinePartsDiff {
    left_lineno: i64,
    left: PartsDiff,
    right_lineno: i64,
    right: PartsDiff,
}
struct LineDiff {
    left_lineno: i64,
    left: String,
    right_lineno: i64,
    right: String,
}
impl PartsDiff {
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
        let mut width: usize = 0;
        for part in &self.parts {
            match part {
                Part::Equal(s) => width += UnicodeWidthStr::width(s.as_str()),
                Part::Delete(s) => width += UnicodeWidthStr::width(s.as_str()),
                Part::Insert(s) => width += UnicodeWidthStr::width(s.as_str()),
            }
        }
        width
    }
}
#[pyclass(name = "Part", get_all)]
#[derive(Clone)]
struct PyPart {
    value: String,
    kind: String, // EQUAL|DELETE|INSERT
}
impl PartialEq for PyPart {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.kind == other.kind
    }
}

fn diff_lines(a: String, b: String) -> (PartsDiff, PartsDiff) {
    let diff = TextDiff::from_chars(&a, &b);
    let mut x = PartsDiff { parts: Vec::new() };
    let mut y = PartsDiff { parts: Vec::new() };
    for change in diff.iter_all_changes() {
        let value = change.value();
        match change.tag() {
            ChangeTag::Equal => {
                x.push_equal(value);
                y.push_equal(value);
            }
            ChangeTag::Delete => {
                if change.old_index().is_some() {
                    x.push_delete(value);
                } else {
                    y.push_delete(value)
                }
            }
            ChangeTag::Insert => {
                if change.old_index().is_some() {
                    x.push_insert(value);
                } else {
                    y.push_insert(value)
                }
            }
        }
    }
    (x, y)
}

fn diff_a_and_b(a: String, b: String, context_lines: Option<usize>) -> Vec<LinePartsDiff> {
    let diff = TextDiff::from_lines(&a, &b);
    let mut unified = diff.unified_diff();
    unified.context_radius(context_lines.unwrap_or(1000)); // We default to just a large number

    let mut diffs: Vec<LineDiff> = Vec::new();

    let mut append: bool = false;
    for hunk in unified.iter_hunks() {
        for change in hunk.iter_changes() {
            let value = String::from(change.value().trim_end_matches('\n'));
            match change.tag() {
                ChangeTag::Equal => {
                    let diff = LineDiff {
                        left_lineno: change.old_index().unwrap() as i64 + 1,
                        left: value.clone(),
                        right_lineno: change.new_index().unwrap() as i64 + 1,
                        right: value.clone(),
                    };
                    diffs.push(diff);
                    append = false;
                }
                _ => {
                    if append {
                        let last = diffs.len() - 1;
                        if change.old_index().is_some() {
                            diffs[last].left_lineno = change.old_index().unwrap() as i64 + 1;
                            diffs[last].left = value.clone();
                        } else {
                            diffs[last].right_lineno = change.new_index().unwrap() as i64 + 1;
                            diffs[last].right = value.clone();
                        }
                        append = false;
                    } else {
                        if change.old_index().is_some() {
                            let diff = LineDiff {
                                left_lineno: change.old_index().unwrap() as i64 + 1,
                                left: value.clone(),
                                right_lineno: -1,
                                right: String::from(""),
                            };
                            diffs.push(diff);
                        } else {
                            let diff = LineDiff {
                                left_lineno: -1,
                                left: String::from(""),
                                right_lineno: change.new_index().unwrap() as i64 + 1,
                                right: value.clone(),
                            };
                            diffs.push(diff);
                        }
                        append = true;
                    }
                }
            }
        }
    }
    fn f(diff: &LineDiff) -> LinePartsDiff {
        let (x, y) = diff_lines(diff.left.clone(), diff.right.clone());
        LinePartsDiff {
            left_lineno: diff.left_lineno,
            left: x,
            right_lineno: diff.right_lineno,
            right: y,
        }
    }
    diffs.iter().map(f).collect()
}

fn generate_html(diff: Vec<LinePartsDiff>, column_limit: usize) -> String {
    let mut html = String::new();
    let mut left = String::new();
    let mut right = String::new();
    html.push_str("<div>");
    html.push_str("<style>");
    html.push_str(".ocdiff-container { display: flex; background-color: #141414; color: white; }");
    html.push_str(".ocdiff-side { width: 50%; overflow-x: scroll; margin: 0; padding: 1rem; }");
    html.push_str(".ocdiff-delete { font-weight: bolder; color: red; }");
    html.push_str(".ocdiff-insert { font-weight: bolder; color: green; }");
    html.push_str("</style>");
    html.push_str("<div class=\"ocdiff-container\">");

    for line_diff in diff {
        let _max_width = line_diff.left.width().max(line_diff.right.width());
        let _number_of_lines = (_max_width + (column_limit - 1)) / column_limit;

        left.push_str(generate_html_parts(&line_diff.left.parts).as_str());
        left.push_str("\n");
        right.push_str(generate_html_parts(&line_diff.right.parts).as_str());
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

fn generate_html_parts(parts: &Vec<Part>) -> String {
    let mut html = String::new();

    for part in parts {
        match part {
            Part::Equal(text) => {
                let escaped_text = encode_text(text);
                html.push_str(&format!(
                    "<span class=\"ocdiff-line ocdiff-equal\">{}</span>",
                    escaped_text
                ));
            }
            Part::Delete(text) => {
                let escaped_text = encode_text(text);
                html.push_str(&format!(
                    "<span class=\"ocdiff-line ocdiff-delete\">{}</span>",
                    escaped_text
                ));
            }
            Part::Insert(text) => {
                let escaped_text = encode_text(text);
                html.push_str(&format!(
                    "<span class=\"ocdiff-line ocdiff-insert\">{}</span>",
                    escaped_text
                ));
            }
        }
    }
    html
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
        let line_parts_diffs = diff_a_and_b(a, b, context_lines);
        let html = generate_html(line_parts_diffs, column_limit.unwrap_or(80));
        Ok(html)
    }

    Ok(())
}
