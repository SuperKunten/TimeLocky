mod models;
mod timelocky;

use pyo3::prelude::*;
use pyo3::types::PyBytes;

use crate::models::CreateStreamParams;
use crate::timelocky::*;

/// Build CreateStream instruction data
#[pyfunction]
#[pyo3(signature = (
    start_time,
    net_amount_deposited,
    period,
    amount_per_period,
    cliff,
    cliff_amount,
    cancelable_by_sender,
    cancelable_by_recipient,
    automatic_withdrawal,
    transferable_by_sender,
    transferable_by_recipient,
    can_topup,
    stream_name,
    withdraw_frequency,
    pausable = false,
    can_update_rate = false
))]
fn build_create_stream_data(
    py: Python<'_>,
    start_time: u64,
    net_amount_deposited: u64,
    period: u64,
    amount_per_period: u64,
    cliff: u64,
    cliff_amount: u64,
    cancelable_by_sender: bool,
    cancelable_by_recipient: bool,
    automatic_withdrawal: bool,
    transferable_by_sender: bool,
    transferable_by_recipient: bool,
    can_topup: bool,
    stream_name: &str,
    withdraw_frequency: u64,
    pausable: bool,
    can_update_rate: bool,
) -> PyResult<Py<PyBytes>> {
    let params = CreateStreamParams {
        start_time,
        net_amount_deposited,
        period,
        amount_per_period,
        cliff,
        cliff_amount,
        cancelable_by_sender,
        cancelable_by_recipient,
        automatic_withdrawal,
        transferable_by_sender,
        transferable_by_recipient,
        can_topup,
        stream_name: pad_stream_name(stream_name),
        withdraw_frequency,
        pausable,
        can_update_rate,
    };

    let data = build_create_stream_ix_data(&params);
    Ok(PyBytes::new(py, &data).into())
}

/// Build Withdraw instruction data
#[pyfunction]
fn build_withdraw_data(py: Python<'_>, amount: u64) -> PyResult<Py<PyBytes>> {
    let data = build_withdraw_ix_data(amount);
    Ok(PyBytes::new(py, &data).into())
}

/// Build Cancel instruction data
#[pyfunction]
fn build_cancel_data(py: Python<'_>) -> PyResult<Py<PyBytes>> {
    let data = build_cancel_ix_data();
    Ok(PyBytes::new(py, &data).into())
}

/// Build TransferRecipient instruction data
#[pyfunction]
fn build_transfer_data(py: Python<'_>) -> PyResult<Py<PyBytes>> {
    let data = build_transfer_ix_data();
    Ok(PyBytes::new(py, &data).into())
}

/// Build TopUp instruction data
#[pyfunction]
fn build_topup_data(py: Python<'_>, amount: u64) -> PyResult<Py<PyBytes>> {
    let data = build_topup_ix_data(amount);
    Ok(PyBytes::new(py, &data).into())
}

/// Decode on-chain stream account data, returns JSON string
#[pyfunction]
fn decode_stream(py: Python<'_>, data: &[u8]) -> PyResult<String> {
    match decode_stream_data(data) {
        Ok(stream) => {
            let json = serde_json::to_string_pretty(&stream)
                .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;
            Ok(json)
        }
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}

/// Get the TimeLocky program ID
#[pyfunction]
fn get_program_id() -> &'static str {
    TIMELOCKY_PROGRAM_ID
}

/// Get the TimeLocky treasury address
#[pyfunction]
fn get_treasury_address() -> &'static str {
    TIMELOCKY_TREASURY
}

/// Get SPL Token program ID
#[pyfunction]
fn get_token_program_id() -> &'static str {
    TOKEN_PROGRAM_ID
}

/// Get Associated Token program ID
#[pyfunction]
fn get_associated_token_program_id() -> &'static str {
    ASSOCIATED_TOKEN_PROGRAM_ID
}

/// Get System program ID
#[pyfunction]
fn get_system_program_id() -> &'static str {
    SYSTEM_PROGRAM_ID
}

/// Pad and return a stream name as 64 bytes
#[pyfunction]
fn pad_name(py: Python<'_>, name: &str) -> PyResult<Py<PyBytes>> {
    let padded = pad_stream_name(name);
    Ok(PyBytes::new(py, &padded).into())
}

/// Return the application HMAC secret (compiled into the binary).
#[pyfunction]
fn get_app_secret() -> String {
    // 64-byte hex secret baked into the compiled .pyd – not visible in Python source.
    String::from("b7e9a3f1c4d82056e1f0a39d7c5b824690df13e87a2c4b5f6d0e1a93c8b72d4f")
}

/// Python module entrypoint
#[pymodule]
fn timelocky_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(build_create_stream_data, m)?)?;
    m.add_function(wrap_pyfunction!(build_withdraw_data, m)?)?;
    m.add_function(wrap_pyfunction!(build_cancel_data, m)?)?;
    m.add_function(wrap_pyfunction!(build_transfer_data, m)?)?;
    m.add_function(wrap_pyfunction!(build_topup_data, m)?)?;
    m.add_function(wrap_pyfunction!(decode_stream, m)?)?;
    m.add_function(wrap_pyfunction!(get_program_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_treasury_address, m)?)?;
    m.add_function(wrap_pyfunction!(get_token_program_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_associated_token_program_id, m)?)?;
    m.add_function(wrap_pyfunction!(get_system_program_id, m)?)?;
    m.add_function(wrap_pyfunction!(pad_name, m)?)?;
    m.add_function(wrap_pyfunction!(get_app_secret, m)?)?;
    Ok(())
}
