use pyo3::prelude::*;
use pyo3::py_run;
use pyo3::types::PyDict;
use std::borrow::BorrowMut;
use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Once;
use std::time::Instant;
use time::OffsetDateTime;
use ulid::Ulid;

#[pyfunction]
fn new() -> PyResult<PyUlid> {
    static ONCE: Once = Once::new();
    static CHECK_TIME: AtomicBool = AtomicBool::new(true);
    ONCE.call_once(|| {
        std::thread::spawn(|| {
            thread_local! {
                static LAST: Cell<Instant> = { Cell::new(Instant::now()) };
            }
            LAST.with(|last| loop {
                if last.get().elapsed().as_micros() > 100 {
                    last.set(Instant::now());
                    CHECK_TIME.store(true, Ordering::Relaxed);
                }
                std::thread::sleep(std::time::Duration::from_micros(100));
            });
        });
    });
    thread_local! {
        static GEN_STATE: Cell<Ulid> = { Cell::new(Ulid::new()) };
    }

    GEN_STATE.with(|s| {
        let x = if CHECK_TIME.fetch_and(false, Ordering::Relaxed) {
            s.set(Ulid::new());
            s.get()
        } else {
            let x = s.get().increment().unwrap();
            s.set(x);
            x
        };
        Ok(PyUlid::new(x))
    })
}

#[pyfunction]
fn batch_new(batch_size: usize) -> PyResult<Vec<PyUlid>> {
    let mut gen = ulid::Generator::new();
    let mut rng = rand::thread_rng();
    let now = OffsetDateTime::now_utc();

    Ok((0..batch_size)
        .map(|_| {
            PyUlid::new(
                gen.generate_from_datetime_with_source(now, &mut rng)
                    .unwrap(),
            )
        })
        .collect())
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
        Ok(self.0 .0.to_le_bytes().to_vec())
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

    pub fn uuid(&self) -> PyResult<Py<PyAny>> {
        Python::with_gil(|py| {
            let locals = PyDict::new(py);
            let uuid = py.import("uuid")?.getattr("UUID")?;
            locals.set_item("UUID", uuid)?;
            py.eval(
                format!("UUID(int={})", self.0 .0).as_str(),
                None,
                Some(locals),
            )
            .map(|p| p.to_object(py))
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
