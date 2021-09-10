use merk::execute_proof;
use std::slice;
use std::mem;
use std::ptr;
#[repr(C)]
pub struct Element {
    pub key_length: usize,
    pub key: *mut u8, //32 bytes
    pub bool: bool, //1 byte
    pub value_length: usize, //8 bytes
    pub value: *mut u8, //value_length bytes
}
#[repr(C)]
pub struct ExecuteProofResult {
    pub hash: *mut [u8; 32],  //32 bytes
    pub element_count: usize,  //8 bytes
    pub elements: *mut *mut Element, //sizeof(pointer)
}
#[no_mangle]
pub extern fn execute_proof_c(c_array: *const u8, length: usize) -> *mut ExecuteProofResult {
    let rust_array: &[u8] = unsafe { slice::from_raw_parts(c_array, length as usize) };
    let execute_proof_result = execute_proof(rust_array);
    match execute_proof_result {
        Err(_) => ptr::null_mut(),
        Ok((hash, map)) => {
            let elements_map = map.all().map(|(key, (exists, value))| {
                let mut key_slice = key.to_vec().into_boxed_slice();
                let mut value_slice = value.to_vec().into_boxed_slice();
                let element = Element {
                    key_length: key_slice.len(),
                    key: key_slice.as_mut_ptr(),
                    bool: *exists,
                    value_length: value_slice.len(),
                    value: value_slice.as_mut_ptr()
                };
                Box::into_raw(Box::new(element))
            });
            let a:Vec<*mut Element> = elements_map.collect();
            let mut elements_slice = a.into_boxed_slice();
            let result = ExecuteProofResult {
                hash: Box::into_raw(Box::new(hash)),
                element_count: elements_slice.len(),
                elements: elements_slice.as_mut_ptr()
            };
            mem::forget(elements_slice);
            Box::into_raw(Box::new(result))
        }
    }
}
#[no_mangle]
pub unsafe extern fn destroy_proof_c(proof_result: *mut ExecuteProofResult) {
    let result = Box::from_raw(proof_result);
    let _ = Box::from_raw(result.hash);
    let vec = Vec::from_raw_parts(result.elements, result.element_count, result.element_count);
    for &x in vec.iter() {
        let _ = Box::from_raw(x);
    }
}
