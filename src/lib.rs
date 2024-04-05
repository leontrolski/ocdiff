extern crate pyo3;

// use pyo3::exceptions::*;
use pyo3::prelude::*;
// use pyo3::types::*;


#[pymodule]
#[pyo3(name = "ocdiff")]
fn init_mod(_py: Python, m: &PyModule) -> PyResult<()> {
    // Functions
    #[pyfn(m)]
    #[pyo3(name = "foo")]
    fn to_object_rs<'a>(
        _py: Python<'a>,
    ) -> PyResult<i64> {
        Ok(32)
    }


    Ok(())
}
