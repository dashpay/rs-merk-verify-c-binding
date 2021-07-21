use merk::execute_proof;
use merk::proofs::query::Map;
use std::slice;

use std::ptr;

#[repr(C)]
pub struct Element {
    pub key: *const u8, //32 bytes
    pub bool: bool, //1 byte
    pub value_length: usize, //8 bytes
    pub value: *const u8, //value_length bytes
}

#[repr(C)]
pub struct ExecuteProofResult {
    pub hash: *const u8,  //32 bytes
    pub element_count: usize,  //8 bytes
    pub elements: *const Element, //sizeof(pointer)
}

#[no_mangle]
pub extern "C" fn execute_proof_c(c_array: *const u8, length: usize) -> *const ExecuteProofResult {
    let rust_array: &[u8] = unsafe { slice::from_raw_parts(c_array, length as usize) };
    let execute_proof_result = execute_proof(rust_array);
    match execute_proof_result {
        Err(_) => ptr::null(),
        Ok((hash, map)) => {
            let elements_map = map.all().map(|key_value_pair| -> Element {
                let element_hash = key_value_pair.0;
                let (exists, value) = key_value_pair.1;
                Element {
                    key: element_hash.as_ptr(),
                    bool: *exists,
                    value_length: 0,
                    value: value.as_ptr(),
                }
            });
            let elements = elements_map.collect();
            let elements_count = elements_map.len();
            let result = ExecuteProofResult {
                hash: hash.as_ptr(),
                element_count: elements_count,
                elements,
            };
            result
        }
    }

}