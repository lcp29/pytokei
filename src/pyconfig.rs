use pyo3::prelude::*;
use tokei::Config;


#[pyclass]
pub struct PyConfig {
    pub config: Config
}


#[pymethods]
impl PyConfig {
    #[new]
    pub fn py_new() -> Self {
        PyConfig{config: Config::default()}
    }

    #[getter]
    pub fn columns(&self) -> Option<usize> {
        self.config.columns
    }

    #[getter]
    pub fn hidden(&self) -> Option<bool> {
        self.config.hidden
    }

    //
    #[getter]
    pub fn no_ignore(&self) -> Option<bool> {
        self.config.no_ignore
    }

    #[getter]
    pub fn no_ignore_parent(&self) -> Option<bool> {
        self.config.no_ignore_parent
    }

    #[getter]
    pub fn no_ignore_dot(&self) -> Option<bool> {
        self.config.no_ignore_dot
    }

    #[getter]
    pub fn no_ignore_vcs(&self) -> Option<bool> {
        self.config.no_ignore_vcs
    }

    #[getter]
    pub fn treat_doc_strings_as_comments(&self) -> Option<bool> {
        self.config.treat_doc_strings_as_comments
    }
}