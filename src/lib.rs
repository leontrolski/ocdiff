extern crate pyo3;
extern crate similar;

// use pyo3::exceptions::*;
use pyo3::prelude::*;
// use pyo3::types::*;
use similar::{ChangeTag, TextDiff};

type LineNum = i64;
type Diff = ((LineNum, String), (LineNum, String), bool);

struct LineDiff {
    left_lineno: i64,
    left: PartsDiff,
    right_lineno: i64,
    right: PartsDiff
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
        let diff = TextDiff::from_lines(&a, &b);
        let mut unified = diff.unified_diff();
        unified.context_radius(context_lines.unwrap_or(1000)); // We default to just a large number

        let mut diffs: Vec<Diff> = Vec::new();

        let mut append: bool = false;
        for hunk in unified.iter_hunks() {
            for change in hunk.iter_changes() {
                let value = String::from(change.value().trim_end_matches('\n'));
                match change.tag() {
                    ChangeTag::Equal => {
                        let diff: Diff = (
                            (change.old_index().unwrap() as i64 + 1, value.clone()),
                            (change.new_index().unwrap() as i64 + 1, value.clone()),
                            false,
                        );
                        diffs.push(diff);
                        append = false;
                    }
                    _ => {
                        if append {
                            let last = diffs.len() - 1;
                            if change.old_index().is_some() {
                                diffs[last].0 .0 = change.old_index().unwrap() as i64 + 1;
                                diffs[last].0 .1 = value.clone();
                            } else {
                                diffs[last].1 .0 = change.new_index().unwrap() as i64 + 1;
                                diffs[last].1 .1 = value.clone();
                            }
                            diffs[last].2 = true;
                            append = false;
                        } else {
                            if change.old_index().is_some() {
                                let diff: Diff = (
                                    (change.old_index().unwrap() as i64 + 1, value.clone()),
                                    (-1, String::from("\n")),
                                    true,
                                );
                                diffs.push(diff);
                            } else {
                                let diff: Diff = (
                                    (-1, String::from("\n")),
                                    (change.new_index().unwrap() as i64 + 1, value.clone()),
                                    true,
                                );
                                diffs.push(diff);
                            }
                            append = true;
                        }
                    }
                }
            }
        }
        fn f(diff: &Diff) -> Diff {
            let (x, y) = _diff_lines(diff.0 .1.clone(), diff.1 .1.clone());
            ((diff.0 .0, x.to_string()), (diff.1 .0, y.to_string()), diff.2)
        }
        let diffs_each: Vec<Diff> = diffs.iter().map(f).collect();
        Ok(diffs_each)
    }

    Ok(())
}
