
use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::PyDict;
use time::OffsetDateTime;
use ulid::Ulid;

#[pyfunction]
fn new() -> PyResult<PyUlid> {
    let ulid = Ulid::new();
    Ok(PyUlid::new(ulid))
}

#[pyfunction]
fn batch_new(batch_size: usize) -> PyResult<Vec<PyUlid>> {
    let mut gen = ulid::Generator::new();
    let mut rng = rand::thread_rng();
    let now = OffsetDateTime::now_utc();

    Ok((0..batch_size).map(|_| PyUlid::new(gen.generate_from_datetime_with_source(now, &mut rng).unwrap())).collect())
}

#[pyclass]
struct PyUlid(Ulid);

impl PyUlid {
    fn new(ulid: Ulid) -> Self {
        PyUlid(ulid)
    }
}

#[pymethods]
impl PyUlid {
    pub fn __str__(&self) -> PyResult<String> {
        Ok(self.0.to_string())
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("<ULID('{}')>", self.0.to_string()))
    }

    pub fn bytes(&self) -> PyResult<Vec<u8>> {
        Ok(self.0.0.to_le_bytes().to_vec())
    }

    pub fn timestamp(&self) -> u64 {
        self.0.timestamp_ms()
    }

    pub fn randomness(&self) -> u128 {
        self.0.random()
    }

    pub fn str(&self) -> String {
        self.0.to_string()
    }

    pub fn uuid(&self) ->  PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            let uuid = py.import("uuid")?.getattr("UUID")?;
            locals.set_item("UUID", uuid)?;
            py.eval(format!("UUID(int={})", self.0.0).as_str(), None, Some(locals)).map(|p| p.to_object(py))
        })
    }

}

/// A Python module implemented in Rust.
#[pymodule]
fn ulid_rs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(new, m)?)?;
    m.add_function(wrap_pyfunction!(batch_new, m)?)?;
    Ok(())
}