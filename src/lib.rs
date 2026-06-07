use pyo3::prelude::*;

#[pymodule]
pub mod pychmrs {
    use std::ops::{
        Deref,
        DerefMut,
    };

    use libchm::{
        ChmFile,
        EntrySel,
    };
    use pyo3::prelude::*;

    #[pyclass(from_py_object, eq, eq_int)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum PyEntrySel {
        /// Paths starting with `/` but not `/#` or `/$`.
        Normal  = 0x01,
        /// Paths starting with `/#` or `/$`.
        Special = 0x02,
        /// Paths not starting with `/` (internal metadata).
        Meta    = 0x04,
        /// Non-directory entries (path does not end with `/`).
        Files   = 0x08,
        /// Directory entries (path ends with `/`).
        Dirs    = 0x10,
        /// All entries.
        All     = 0x1F,
    }

    impl From<PyEntrySel> for EntrySel {
        fn from(val: PyEntrySel) -> Self {
            match val {
                PyEntrySel::Normal => EntrySel::NORMAL,
                PyEntrySel::Special => EntrySel::SPECIAL,
                PyEntrySel::Meta => EntrySel::META,
                PyEntrySel::Files => EntrySel::FILES,
                PyEntrySel::Dirs => EntrySel::DIRS,
                PyEntrySel::All => EntrySel::ALL,
            }
        }
    }

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
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(
                        "failed to open chm file",
                    )
                })?,
            })
        }

        pub fn read(
            &mut self,
            path: &str,
        ) -> PyResult<Vec<u8>> {
            let inner: &mut ChmFile = self;
            let entry = inner
                .find(path)
                .map_err(|_e| {
                    PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                        "entry not found",
                    )
                })?;
            inner
                .read(&entry)
                .map_err(|_e| {
                    PyErr::new::<pyo3::exceptions::PyIOError, _>(
                        "failed to read",
                    )
                })
        }

        #[pyo3(signature = (prefix = None, entry_sel = PyEntrySel::All))]
        #[pyo3(text_signature = "(self, prefix: str = None, \
                                 entry_sel: PyEntrySel = \
                                 PyEntrySel.All)")]
        pub fn list_paths(
            &mut self,
            prefix: Option<String>,
            entry_sel: PyEntrySel,
        ) -> PyResult<Vec<String>> {
            let inner: &mut ChmFile = self;

            let entries = if let Some(p) = prefix {
                inner.entries_in(p.as_str(), entry_sel.into())
            } else {
                inner.entries(entry_sel.into())
            };

            let entries = entries.map_err(|_e| {
                PyErr::new::<pyo3::exceptions::PyIOError, _>(
                    "entry not found",
                )
            })?;
            Ok(entries
                .iter()
                .map(|e| {
                    e.path
                        .clone()
                })
                .collect())
        }
    }
}
