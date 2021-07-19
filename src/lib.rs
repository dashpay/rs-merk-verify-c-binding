use merk::{execute_proof, Hash, Error};
use merk::proofs::query::Map;
use std::os::raw::{c_uchar};


#[no_mangle]
pub extern "C" fn execute_proof_c(bytes: *const c_uchar) -> Result<(Hash, Map), Error> {
    let execute_proof_result = execute_proof(bytes)?;
    Ok(execute_proof_result)
}