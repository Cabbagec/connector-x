use crate::errors::ConnectorXPythonError;
use arrow::ffi::{FFI_ArrowArray, FFI_ArrowSchema};
use arrow::record_batch::RecordBatch;
use connectorx::source_router::SourceConn;
use connectorx::{prelude::*, sql::CXQuery};
use fehler::throws;
use libc::uintptr_t;
use pyo3::prelude::*;
use pyo3::{PyAny, Python};

#[throws(ConnectorXPythonError)]
pub fn write_arrow<'a>(
    py: Python<'a>,
    source_conn: &SourceConn,
    origin_query: Option<String>,
    queries: &[CXQuery<String>],
) -> &'a PyAny {
    let destination = get_arrow(source_conn, origin_query, queries)?;
    let rbs = destination.arrow()?;
    let ptrs = to_ptrs(rbs);
    let obj: PyObject = ptrs.into_py(py);
    obj.into_ref(py)
}

pub fn to_ptrs(rbs: Vec<RecordBatch>) -> (Vec<String>, Vec<Vec<(uintptr_t, uintptr_t)>>) {
    if rbs.is_empty() {
        return (vec![], vec![]);
    }

    let mut result = vec![];
    let names = rbs[0]
        .schema()
        .fields()
        .iter()
        .map(|f| f.name().clone())
        .collect();

    for rb in rbs {
        let mut cols = vec![];

        for array in rb.columns() {
            let array_ptr = Box::new(FFI_ArrowArray::empty());
            let schema_ptr = Box::new(FFI_ArrowSchema::empty());
            let array_ptr = Box::into_raw(array_ptr);
            let schema_ptr = Box::into_raw(schema_ptr);
            unsafe {
                arrow::array::export_array_into_raw(array.clone(), array_ptr, schema_ptr)
                    .expect("export_array_into_raw error");
            }
            cols.push((array_ptr as uintptr_t, schema_ptr as uintptr_t));
        }

        result.push(cols);
    }
    (names, result)
}
