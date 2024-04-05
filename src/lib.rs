extern crate pyo3;
extern crate similar;

// use pyo3::exceptions::*;
use pyo3::prelude::*;
// use pyo3::types::*;
use similar::{ChangeTag, TextDiff};

type LineNum = i64;
type Diff = ((LineNum, String), (LineNum, String), bool);

fn _diff_lines(a: String, b: String) -> (String, String) {
    let diff = TextDiff::from_chars(&a, &b);
    let mut x = String::from("");
    let mut y = String::from("");
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                x.push_str(change.value());
                y.push_str(change.value());
            }
            ChangeTag::Delete => {
                let value = format!("\x00-{}\x01", change.value());
                if change.old_index().is_some() {
                    x.push_str(value.as_str());
                } else {
                    y.push_str(value.as_str())
                }
            }
            ChangeTag::Insert => {
                let value = format!("\x00^{}\x01", change.value());
                if change.old_index().is_some() {
                    x.push_str(value.as_str());
                } else {
                    y.push_str(value.as_str())
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
        Ok(_diff_lines(a, b))
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
            ((diff.0 .0, x), (diff.1 .0, y), diff.2)
        }
        let diffs_each: Vec<Diff> = diffs.iter().map(f).collect();
        Ok(diffs_each)
    }

    Ok(())
}
