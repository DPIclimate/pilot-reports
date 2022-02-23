use pyo3::prelude::*;

pub fn to_csv() {
    // Allows pyo3 to run the python file
    pyo3::prepare_freethreaded_python();

    // Loads the python file content
    let py_transform = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/transform.py"));
    Python::with_gil(|py| {
        let transform = PyModule::from_code(py, &py_transform, "transform.py", "Transform").expect("Unable to find transform.py or object Transform is not found.");
        
        // This gets the object "Transform" and calls it with no parameters "call0()"
        transform.getattr("Transform").expect("Transform object not found.").call0().expect(
            "Failed to run Transform"
        );
    });
}


