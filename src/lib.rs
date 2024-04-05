extern crate pyo3;
extern crate similar;

// use pyo3::exceptions::*;
use pyo3::prelude::*;
// use pyo3::types::*;
use similar::{ChangeTag, TextDiff};

type LineNum = String;
type Diff = ( (LineNum, String), (LineNum, String), Option<bool> );


#[pymodule]
#[pyo3(name = "ocdiff")]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    // Functions
    #[pyfn(m)]
    #[pyo3(name = "diff")]
    fn diff<'a>(
        _py: Python<'a>,
    ) -> PyResult<Vec<Diff>> {

        let a = "Hello World\nThis is the second line.\nThis is the third.";
        let b = "Hallo Welt\nThis is the second line.\nThis is life.\nMoar and more";
        let diff = TextDiff::from_lines(a, b);

        // let mut values: Vec<Diff> = Vec::with_capacity(max_line_length(&a, &b));

        let len = a.lines().count().max(b.lines().count());
        let mut values: Vec<Diff> = (0..len).map(|_| (
            (String::from(""), String::from("")),
            (String::from(""), String::from("")),
            Some(true)
        )).collect();

        for change in diff.iter_all_changes() {
            match change.old_index() {
                Some(old_index) => {
                    values[old_index].0.0 = old_index.to_string();
                    values[old_index].0.1 = String::from(change.value());
                },
                None => {}
            }
            match change.new_index() {
                Some(new_index) => {
                    values[new_index].1.0 = new_index.to_string();
                    values[new_index].1.1 = String::from(change.value());
                },
                None => {}
            }
            // let old_index = change.old_index().unwrap_or(0).to_string();

            // let value = String::from(change.value());
            // let diff: Diff = (
            //     (old_index, value.clone()),
            //     (String::from("1"), value),
            //     Some(true)
            // );
            // values.push(diff);

            // let sign = match change.tag() {
            //     ChangeTag::Delete => "-",
            //     ChangeTag::Insert => "+",
            //     ChangeTag::Equal => " ",
            // };
            // print!("{}{}", sign, change);
        }
        Ok(values)
    }


    Ok(())
}
