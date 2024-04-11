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
use unicode_width::UnicodeWidthChar;
use unicode_width::UnicodeWidthStr;

const LINENO_PADDING: usize = 1;

struct Line {
    lineno: usize,
    value: String,
}
struct LineDiff {
    left: Option<Line>,
    right: Option<Line>,
}
#[derive(Clone)]
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
    fn str(&self) -> &String {
        match self {
            Part::Equal(s) | Part::Delete(s) | Part::Insert(s) => s,
        }
    }
    fn split(&self, n: usize) -> (Self, Self) {
        let (x, y) = split_string_at_width(self.str().as_str(), n);
        match self {
            Part::Equal(_) => (Part::Equal(String::from(x)), Part::Equal(String::from(y))),
            Part::Delete(_) => (Part::Delete(String::from(x)), Part::Delete(String::from(y))),
            Part::Insert(_) => (Part::Insert(String::from(x)), Part::Insert(String::from(y))),
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
}

fn split_string_at_width(input: &str, n: usize) -> (&str, &str) {
    if input.width() <= n {
        return (input, "");
    }
    let mut current_width = 0;
    let mut split_i = 0;

    for (i, ch) in input.char_indices() {
        let width = ch.width().unwrap_or(0);
        if current_width + width > n {
            break;
        }
        current_width += width;
        split_i = i + ch.len_utf8();
    }

    let (first, second) = input.split_at(split_i);
    (first, second.trim_start())
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

fn split_parts(parts: &Vec<Part>, n: usize) -> Vec<Vec<Part>> {
    let mut out = vec![vec![]];
    let mut current_width = 0;
    for (i, part) in parts.iter().enumerate() {
        let (head, tail) = part.split(n - current_width);
        if head.str().len() > 0 {
            current_width += head.str().as_str().width();
            out[0].push(head);
        }
        if tail.str().len() > 0 {
            let mut remaining = vec![tail];
            remaining.extend(
                parts[i + 1..]
                    .iter()
                    .map(|part| part.clone())
                    .collect::<Vec<Part>>(),
            );
            out.extend(split_parts(&remaining, n));
            break;
        }
    }
    out
}
fn split_overflow(overflow: &Option<Parts>, n: usize) -> Vec<Option<Parts>> {
    match overflow {
        None => vec![None],
        Some(Parts { parts, lineno }) => split_parts(parts, n)
            .iter()
            .map(|parts| {
                Some(Parts {
                    lineno: lineno.clone(),
                    parts: parts.clone(),
                })
            })
            .collect(),
    }
}
fn split_parts_diff(diff: &Vec<PartsDiff>, max_total_width: usize) -> Vec<PartsDiff> {
    let max_lineno_width = find_max_lineno_width(diff);
    // max_total_width = (n + max_lineno_width + LINENO_PADDING * 2) * 2
    let n = (max_total_width / 2) - max_lineno_width - LINENO_PADDING * 2;

    let mut out: Vec<PartsDiff> = vec![];
    for parts_diff in diff {
        let mut split_left = split_overflow(&parts_diff.left, n);
        let mut split_right = split_overflow(&parts_diff.right, n);
        let n_lines = split_left.len().max(split_right.len());
        split_left.resize_with(n_lines, || None);
        split_right.resize_with(n_lines, || None);

        for (left, right) in split_left.into_iter().zip(split_right.into_iter()) {
            out.push(PartsDiff { left, right });
        }
    }
    out
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

fn find_max_lineno_width(diff: &Vec<PartsDiff>) -> usize {
    let max_lineno_left = diff
        .iter()
        .rev()
        .map(|parts_diff| parts_diff.left.as_ref().map(|parts| parts.lineno))
        .find_map(|x| x)
        .unwrap_or(0);
    let max_lineno_right = diff
        .iter()
        .rev()
        .map(|parts_diff| parts_diff.right.as_ref().map(|parts| parts.lineno))
        .find_map(|x| x)
        .unwrap_or(0);
    max_lineno_left.max(max_lineno_right).to_string().len()
}

fn generate_lineno_str(
    prev_lineno: usize,
    new_lineno: Option<usize>,
    max_lineno_width: usize,
) -> String {
    let lineno_str = new_lineno.map_or(String::from(""), |n| {
        if prev_lineno == n {
            String::from("…")
        } else {
            n.to_string()
        }
    });
    format!(
        "{:^width$}",
        lineno_str,
        width = max_lineno_width + LINENO_PADDING * 2
    )
}

fn generate_html(diff: &Vec<PartsDiff>) -> String {
    let max_lineno_width = find_max_lineno_width(diff);

    let mut html = String::new();
    let mut left = String::new();
    let mut right = String::new();
    html.push_str("<div>");
    html.push_str("<style>");
    html.push_str(
        ".ocdiff-container { display: flex; background-color: #141414; color: #acacac; }",
    );
    html.push_str(".ocdiff-side { overflow-x: auto; margin: 0; }");
    html.push_str(".ocdiff-lineno { color: #3b3b3b; background-color: #00003d; }");
    html.push_str(".ocdiff-delete { color: red; }");
    html.push_str(".ocdiff-insert { color: green; }");
    html.push_str("</style>");
    html.push_str("<div class=\"ocdiff-container\">");

    let mut prev_lineno_left = 0;
    let mut prev_lineno_right = 0;
    for line_diff in diff {
        let new_lineno_left = line_diff.left.as_ref().map(|parts| parts.lineno);
        let new_lineno_right = line_diff.right.as_ref().map(|parts| parts.lineno);

        let lineno_str_left =
            generate_lineno_str(prev_lineno_left, new_lineno_left, max_lineno_width);
        let lineno_str_right =
            generate_lineno_str(prev_lineno_right, new_lineno_right, max_lineno_width);

        left.push_str(format!("<span class=\"ocdiff-lineno\">{}</span>", lineno_str_left).as_str());
        left.push_str(generate_html_parts(&line_diff.left).as_str());
        left.push_str("\n");

        right.push_str(
            format!("<span class=\"ocdiff-lineno\">{}</span>", lineno_str_right).as_str(),
        );
        right.push_str(generate_html_parts(&line_diff.right).as_str());
        right.push_str("\n");

        prev_lineno_left = new_lineno_left.unwrap_or(prev_lineno_left);
        prev_lineno_right = new_lineno_right.unwrap_or(prev_lineno_right);
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
            .map(|part| {
                let (class, text) = match part {
                    Part::Equal(text) => ("ocdiff-equal", text),
                    Part::Delete(text) => ("ocdiff-delete", text),
                    Part::Insert(text) => ("ocdiff-insert", text),
                };
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
    #[pyo3(name = "html_diff", signature = (a, b, *, context_lines=None, max_total_width=None))]
    fn html_diff<'a>(
        _py: Python<'a>,
        a: String,
        b: String,
        context_lines: Option<usize>,
        max_total_width: Option<usize>,
    ) -> PyResult<String> {
        let mut line_parts_diffs = diff_a_and_b(&a, &b, context_lines);
        max_total_width.map(|n| line_parts_diffs = split_parts_diff(&line_parts_diffs, n));
        let html = generate_html(&line_parts_diffs);
        Ok(html)
    }

    Ok(())
}
