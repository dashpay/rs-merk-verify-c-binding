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

#[cfg(test)]
mod test {
    use crate::{vec_to_raw_pointer, execute_proof_c, execute_proof_query_keys_c, Keys, Query};
    use std::{num::ParseIntError};
    use std::ptr;

    pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }


    #[test]
    fn verify_execute_proof_c() {
        let proof_string = "018604c121970b1dd7ff9682c9584dcb328eb5b6a4ceb3f5c50dc4a30881ec029902fbf32b582538666ca797807cd087df7e16caeffebfc40a94448f62e370ef7bba1003146bcfdf839d58e08ff0128c19d81e450a19b09a2100201c6f400f342d9936f39ad8082344e0aede119f45bdfa4d21811bec243c04200903146f339bc9d3fccc45c09a9678de642d620ba8624c0020575a3be407c7b43aea23a34891a5dd0e4d21f97d1a5a113caf475e0ee5bd80a31001dda3a52ce526f6f3f195ef899d4c20c9c2763e35eeb6ef2047a2f44f6ff9e74f110274d5cda9db960da8eebe9de190de382707eb35196776f133d76a73731ef34702100185f350bf3690cf21c51820811533e3949ba238b2aa55ed57b650eb57211185941102c1a461d30f21383643451cb8a99a03f946e23884799a7a68c172f7c7128ca2071003149c7868515d684da9468d6eaff354902795c1509800202947ab16436bb350747029bb2b317131d564ef524c33f4ad427b406a543c084e03149e3e4dfa8682d9d5f94516ed9d5ca008e18754910020f4e3b50bd0f73395689ceb3d1fc930987580850becb08945885a57311cbb05051001885e7e3dee35bbf4735b252767aafb4ed81332f34c24556cb058d64ce933ed541102fd0ca4e90c8c7e80095b9a02c3fbae2a017f8a27efdb572d3e08144693693e37100314b1386c941706cb5cf5446bd0ba449a4d44beab1600204daae7f0fb8e0b8df9eb54fe33cfe44a11cebdbe7987efebbf9be93b0320d5d50314b32efd4249df7f2197d08035ae1df2bc58d8c1cc0020878122e15567c6d2b3a1bebf86c7fc6b0c401dc80444406e10678f43e91b546b1001eff524fbd1f3b03df81fa6a1c9ebd19f50a99c28fda7ed830e0f53352fe7881c111102e9010df939d28f3fd03cc1c42d1da8095069c9243dc85015cf13648d78dd31581001a620de290694dadc8db87fa26f131b2f51b863e2a45624166305443d94a38c2c0314cfc46d8fcf142fb4a25700983e912bd3bac8d7eb00207376e2b96c26eff8be40be134030b7d31769ef7dd038da93850a9636fac6f1a6100314d3d8aebf222e2350db1a6b119134f24efe0627ce002096b06816790899354d5de7ba3a4bf1f482ff8594ffc35f81aff4b6b9c8fa09ed110314dfd0a3cf05a7cd1547bab95b1d717c05387253190020a561db76723b888a9412d7bae6c721319b0ab04904da4328611f28dad438a42910015d65ec7a5efe14a87fc79b3d74795fab716410a3aaa9278ee3947f2d648da3d51102a725e92fec23c188b4931a3c45430e0917ef2de7cd2fb9306458f5d4d48f0e2f100121af2df2423288f917ea6d4cde07587edead17292bf45e747274946b7007fa8111111111";

        let proof_vec = decode_hex(proof_string);

        let proof_pointer = vec_to_raw_pointer(proof_vec.unwrap());

        let result = execute_proof_c(proof_pointer, 1976 / 2);

        unsafe {
            assert_eq!((*result).element_count, 9);
        }

        let query_key_str = vec!["b2c4d80c2d16da67cb42ad5765513668b4fc524d", "9d1560ea9f6e6d33ac4ebf6e2acbb8cdb5f54a5d",
                                 "d3afa1ca0536ebbf69ba031272e6a80413bdb717", "6e0f6ad70fc0d424e0a91dc67652ddaf9ca41dd7", "d8fc5dd8e33d461954b8a2552e600ab93e70be6f"];

        let queries: Vec<*mut Query> = query_key_str.iter().map(|str| {
            let query_key = decode_hex(str);
            let key = query_key.unwrap();
            let query = Query {
                key_length: key.len(),
                key: vec_to_raw_pointer(key.clone()),
                key_end_length: 0,
                key_end: ptr::null_mut()
            };

            Box::into_raw(Box::new(query))
        }).collect();

        let keys = &Keys {
            element_count: queries.len(),
            elements: vec_to_raw_pointer(queries),
        };

        let result2 = execute_proof_query_keys_c(proof_pointer, 1976 / 2, keys);

        unsafe {
            assert_eq!((*result2).valid, true);
            assert_eq!((*result2).element_count, 5);
            for i in 0..(*result2).element_count {
                let query_element = *((*result2).elements.offset(i as isize));
                assert_eq!((*query_element).exists, false);
            }
        }
    }
}
