use merk::execute_proof;
use std::slice;
use std::mem;
use std::ptr;

#[repr(C)]
pub struct Element {
    pub key_length: usize,
    pub key: *mut u8,
    pub exists: bool, //1 byte
    pub value_length: usize, //8 bytes
    pub value: *mut u8, //value_length bytes
}

#[repr(C)]
pub struct ExecuteProofResult {
    pub valid: bool,
    pub hash: *mut [u8; 32],  //32 bytes
    pub element_count: usize,  //8 bytes
    pub elements: *mut *mut Element, //sizeof(pointer)
}

#[repr(C)]
pub struct Query {
    pub key_length: usize,
    pub key: *mut u8,
    pub key_end_length: usize, //if range query, not 0
    pub key_end: *mut u8,
}

#[repr(C)]
pub struct Keys {
    pub element_count: usize,  //8 bytes
    pub elements: *mut *mut Query, //sizeof(pointer)
}

fn vec_to_raw_pointer<T>(mut vec: Vec<T>) -> *mut T {
    // Take the pointer
    let pointer = vec.as_mut_ptr();

    // Release the ownership
    // mem::forget releases ownership without deallocating memory.
    // This essentially gives the ownership to the c caller. Rust needs to get
    // the pointer to the struct back in order to properly discard it later.
    mem::forget(vec);

    // Return the pointer
    pointer
}

#[no_mangle]
pub extern fn execute_proof_c(c_array: *const u8, length: usize) -> *mut ExecuteProofResult {
    let rust_array: &[u8] = unsafe {
        slice::from_raw_parts(c_array, length as usize)
    };

    let execute_proof_result = execute_proof(rust_array);

    match execute_proof_result {
        Err(_) => {
            let result = ExecuteProofResult {
                valid: false,
                hash: ptr::null_mut(),
                element_count: 0,
                elements: ptr::null_mut(),
            };

            Box::into_raw(Box::new(result))
        },
        Ok((hash, map)) => {
            let elements: Vec<*mut Element> = map.all().map(|(key, (exists, value))| {
                let element = Element {
                    key_length: key.len(),
                    key: vec_to_raw_pointer(key.clone()),
                    exists: *exists,
                    value_length: value.len(),
                    value: vec_to_raw_pointer(value.clone())
                };

                Box::into_raw(Box::new(element))
            }).collect();

            let result = ExecuteProofResult {
                valid: true,
                hash: Box::into_raw(Box::new(hash)),
                element_count: elements.len(),
                elements: vec_to_raw_pointer(elements),
            };

            Box::into_raw(Box::new(result))
        }
    }
}

#[no_mangle]
pub extern fn execute_proof_query_keys_c(c_array: *const u8, length: usize, query_keys: *const Keys) -> *mut ExecuteProofResult {
    let rust_array: &[u8] = unsafe {
        slice::from_raw_parts(c_array, length as usize)
    };

    let execute_proof_result = execute_proof(rust_array);

    let query_count;
    unsafe {
        query_count = (*query_keys).element_count;
    }

    match execute_proof_result {
        Err(_) => {
            let result = ExecuteProofResult {
                valid: false,
                hash: ptr::null_mut(),
                element_count: 0,
                elements: ptr::null_mut(),
            };

            Box::into_raw(Box::new(result))
        },
        Ok((hash, map)) => {
            let mut elements: Vec<*mut Element> = vec![];
            let mut has_error: bool = false;
            for i in 0..query_count {
                unsafe {
                    let query_element = *((*query_keys).elements.offset(i as isize));
                    let key = std::slice::from_raw_parts((*query_element).key, (*query_element).key_length);
                    // Queries can either be for a key or for a range
                    if (*query_element).key_end_length > 0 {
                        // This is a range query
                        let key_end = std::slice::from_raw_parts((*query_element).key_end, (*query_element).key_end_length);
                        let mut range_elements: Vec<*mut Element> = map.range(key..key_end).map(|result| {
                            match result {
                                Ok(tuple) => {
                                    let element = Element {
                                        key_length: tuple.0.len(),
                                        key: vec_to_raw_pointer(Vec::from(tuple.0.clone())),
                                        exists: true,
                                        value_length: tuple.1.len(),
                                        value: vec_to_raw_pointer(Vec::from(tuple.1.clone()))
                                    };
                                    Box::into_raw(Box::new(element))
                                }
                                Err(_) => {
                                    has_error = true;
                                    ptr::null_mut()
                                }
                            }
                        }).collect();
                        elements.append(&mut range_elements);
                    } else {
                        match map.get(key) {
                            Ok(option) => {
                                match option {
                                    None => {
                                        let element = Element {
                                            key_length: key.len(),
                                            key: vec_to_raw_pointer(Vec::from(key.clone())),
                                            exists: false,
                                            value_length: 0,
                                            value: ptr::null_mut()
                                        };
                                        elements.push(Box::into_raw(Box::new(element)))
                                    }
                                    Some(value) => {
                                        let element = Element {
                                            key_length: key.len(),
                                            key: vec_to_raw_pointer(Vec::from(key.clone())),
                                            exists: true,
                                            value_length: value.len(),
                                            value: vec_to_raw_pointer(Vec::from(value.clone()))
                                        };
                                        elements.push(Box::into_raw(Box::new(element)))
                                    }
                                }
                            }
                            Err(_) => {
                                has_error = true;
                            }
                        }
                    }
                }
            }

            let result = ExecuteProofResult {
                valid: has_error,
                hash: Box::into_raw(Box::new(hash)),
                element_count: elements.len(),
                elements: vec_to_raw_pointer(elements),
            };

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
