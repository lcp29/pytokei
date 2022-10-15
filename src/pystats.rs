use std::collections::HashMap;
use std::path::PathBuf;
use pyo3::prelude::*;
use tokei::{
    CodeStats,
    Report
};

#[derive(Clone)]
#[pyclass]
pub struct PyCodeStats {
    pub stats: CodeStats
}

#[pymethods]
impl PyCodeStats {
    #[new]
    pub fn py_new() -> Self {
        PyCodeStats{stats: CodeStats::new()}
    }

    #[getter]
    pub fn blanks(&self) -> usize {
        self.stats.blanks
    }

    #[getter]
    pub fn code(&self) -> usize {
        self.stats.code
    }

    #[getter]
    pub fn comments(&self) -> usize {
        self.stats.comments
    }

    pub fn lines(&self) -> usize {
        self.stats.lines()
    }

    pub fn summarise(&self) -> PyCodeStats {
        // The new object is created by copying and inserting the new summarised values
        let mut new_stats = self.clone();
        let summ = self.stats.summarise();
        new_stats.stats.blanks = summ.blanks;
        new_stats.stats.code = summ.code;
        new_stats.stats.comments = summ.comments;
        return new_stats
    }

    pub fn content(&self) -> PyResult<PyObject> {
        // Obtain the inner content as a dict in Python.
        let map = HashMap::from([
            ("blanks", self.blanks()),
            ("code", self.code()),
            ("comments", self.comments()),
            ("lines", self.lines()),
        ]);
        return pyo3::Python::with_gil( |py| {
            Ok( map.to_object( py ) )
        } );
    }

    // fn __repr__(&self) -> PyString {}  // TBD
}


#[pyclass(name="Report")]
pub struct PyReport {
    pub report: Report
}


#[pymethods]
impl PyReport {
    #[new]
    pub fn new(name: &str) -> Self {
        let path = PathBuf::from(name);
        PyReport{report: Report::new(path)}
    }
}