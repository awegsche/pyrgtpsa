use std::fmt::Display;

use rgtpsa::tpsa::TPSA;
use pyo3::exceptions::{PyArithmeticError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyFloat, PyList};

/// creates a precompiled TPSA type 
/// This macro basically exposes all TPSA functions to python.
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
                    .map(|x| {
                        x.cast_as::<PyFloat>().expect("Tpsa should be created from floats").value()
                    })
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
                Self{inner: &self.inner + &other.inner}
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
                Self{inner: self.inner.sin()}
            }

            fn cos(&self) -> Self {
                Self{inner: self.inner.cos()}
            }

            fn exp(&self) -> Self {
                Self{inner: self.inner.exp()}
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.inner)
            }
        }
    };
}

/// This macro assembles the python module, put all your Tpsa definitions in here
///
/// # Example
/// ```
/// assemble_module![
///     [Tpsa6D4, 6, 4];
///     [Tpsa6D4, 4, 4];
/// ]
/// ```
/// will create two precompiled Tpsa types, one with 6 variables and order 4 and
/// the second one with 4 variables and order 4
macro_rules! assemble_module {
    [$([$name:ident, $nv:literal, $mo:literal]);+] => {
        $(make_py_tpsa![$name, $nv, $mo]);+;
        
        #[pymodule]
        fn pyrgtpsa(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
            $(m.add_class::<$name>()?);+;

            Ok(())
        }
    };
}

// finally call the assemble macro, which creates the python module
assemble_module![
    [Tpsa6D4, 6, 4];
    [Tpsa4D4, 4, 4];
    [Tpsa2D4, 2, 4];
    [Tpsa6D8, 6, 8];
    [Tpsa4D8, 4, 8];
    [Tpsa2D8, 2, 8]
];

