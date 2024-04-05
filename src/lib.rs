extern crate pyo3;
extern crate similar;

// use pyo3::exceptions::*;
use pyo3::prelude::*;
// use pyo3::types::*;
use similar::{ChangeTag, TextDiff};

type Diff = ((i64, String), (i64, String), bool);

struct LineDiff {
    left_lineno: i64,
    left: String,
    right_lineno: i64,
    right: String,
}
struct LinePartsDiff {
    left_lineno: i64,
    left: PartsDiff,
    right_lineno: i64,
    right: PartsDiff,
}
#[derive(Clone)]
enum Part {
    Equal(String),
    Delete(String),
    Insert(String),
}
struct PartsDiff {
    parts: Vec<Part>,
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
    fn to_string(&self) -> String {
        let mut line_string = String::from("");
        for part in &self.parts {
            match part {
                Part::Equal(s) => line_string.push_str(s.as_str()),
                Part::Delete(s) => line_string.push_str(format!("\x00-{}\x01", s).as_str()),
                Part::Insert(s) => line_string.push_str(format!("\x00^{}\x01", s).as_str()),
            }
        }
        line_string
    }
}

fn _diff_lines(a: String, b: String) -> (PartsDiff, PartsDiff) {
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

fn _mdiff(a: String, b: String, context_lines: Option<usize>) -> Vec<LinePartsDiff> {
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
                                right: String::from("\n"),
                            };
                            diffs.push(diff);
                        } else {
                            let diff = LineDiff {
                                left_lineno: -1,
                                left: String::from("\n"),
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
        let (x, y) = _diff_lines(diff.left.clone(), diff.right.clone());
        LinePartsDiff {
            left_lineno: diff.left_lineno,
            left: x,
            right_lineno: diff.right_lineno,
            right: y,
        }
    }
    diffs.iter().map(f).collect()
}

#[pymodule]
#[pyo3(name = "ocdiff")]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    // Attempts to replicated the interface of difflib._mdiff
    #[pyfn(m)]
    #[pyo3(name = "diff_lines")]
    fn diff_lines<'a>(_py: Python<'a>, a: String, b: String) -> PyResult<(String, String)> {
        let (x, y) = _diff_lines(a, b);
        Ok((x.to_string(), y.to_string()))
    }

    #[pyfn(m)]
    #[pyo3(name = "mdiff")]
    fn mdiff<'a>(
        _py: Python<'a>,
        a: String,
        b: String,
        context_lines: Option<usize>,
    ) -> PyResult<Vec<Diff>> {
        let line_parts_diffs = _mdiff(a, b, context_lines);
        let difflib_compatible = line_parts_diffs
            .iter()
            .map(|diff| {
                (
                    (diff.left_lineno, diff.left.to_string()),
                    (diff.right_lineno, diff.right.to_string()),
                    true,
                )
            })
            .collect();
        Ok(difflib_compatible)
    }

    Ok(())
}
