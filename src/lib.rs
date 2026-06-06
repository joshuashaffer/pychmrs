use pyo3::prelude::*;

use libchm::{ChmFile, EntrySel};
use std::ops::{Deref, DerefMut};

#[pyclass]
struct ChmFileWrapper {
    inner: ChmFile,
}

impl Deref for ChmFileWrapper {
    type Target = ChmFile;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for ChmFileWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[pymethods]
impl ChmFileWrapper {
    #[new]
    pub fn __new__(obj: String) -> PyResult<Self> {
        Ok(Self {
            inner: ChmFile::open(obj).map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>("failed to open chm file")
            })?,
        })
    }

    pub fn read(&mut self, path: &str) -> PyResult<Vec<u8>> {
        let inner: &mut ChmFile = self;
        let entry = inner
            .find(path)
            .map_err(|_e| PyErr::new::<pyo3::exceptions::PyKeyError, _>("entry not found"))?;
        inner
            .read(&entry)
            .map_err(|_e| PyErr::new::<pyo3::exceptions::PyKeyError, _>("failed to read"))
    }

    pub fn list_paths(&mut self) -> PyResult<Vec<String>> {
        let inner: &mut ChmFile = self;
        let entries = inner
            .entries(EntrySel::all())
            .map_err(|_e| PyErr::new::<pyo3::exceptions::PyIOError, _>("entry not found"))?;
        Ok(entries.iter().map(|e| e.path.clone()).collect())
    }
}
