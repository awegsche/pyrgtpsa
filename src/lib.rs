use std::fmt::Display;

use rgtpsa::tpsa::TPSA;
use pyo3::exceptions::PyArithmeticError;
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyList};

macro_rules! make_py_tpsa {
    ($name:ident, $nv:literal, $mo:literal) => {
        #[pyclass]
        #[derive(Clone)]
        struct $name {
            inner: TPSA<$nv, $mo, f64>,
        }

        #[pymethods]
        impl $name {
            #[new]
            fn new(l: &PyList) -> PyResult<Self> {
                let coeffs: Vec<_> = l
                    .iter()
                    .filter_map(|x| x.cast_as::<PyFloat>().ok())
                    .map(|x| x.value())
                    .collect();

                Ok(Self {
                    inner: TPSA::new(&coeffs),
                })
            }

            /// Clones the TPSA
            fn copy(&self) -> Self {
                Self { inner: self.inner.clone() }
            }

            fn __str__(&self) -> String {
                format!("{}", self)
            }

            fn __add__(&self, other: &Self) -> Self {
                $name{inner: &self.inner + &other.inner}
            }

            fn __iadd__(&mut self, other: &Self) {
                self.inner += &other.inner;
            }

            fn __mul__(&self, other: &PyAny) -> PyResult<Self> {
                if let Ok(rhs) = other.extract::<Self>() {
                    return Ok($name{inner: &self.inner * rhs.inner})
                }
                if let Ok(rhs) = other.cast_as::<PyFloat>() {
                    return Ok($name{inner: &self.inner * rhs.value() })
                }
                Err(PyArithmeticError::new_err("TPSA can only be multiplied by another TPSA of the same order or a Float"))
            }

            fn sin(&self) -> Self {
                $name{inner: self.inner.sin()}
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner)
            }
        }
    };
}

make_py_tpsa!(Tpsa6D, 6, 4);
make_py_tpsa!(Tpsa4D, 4, 4);

#[pymodule]
fn pyrgtpsa(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Tpsa6D>()?;
    m.add_class::<Tpsa4D>()?;

    Ok(())
}
