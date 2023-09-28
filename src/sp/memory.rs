//! This module contains the functions from the memory management section. This is currently unsupported.
//! For rust memory management, a custom allocator would be needed. This is so far unimplemented.

// Allocates a block of at least size * nmemb. The first size * nmemb bytes are set to zero.
//
// # Arguments
//
// * nmemb - The number of elements to allocate.
// * size - The size of each element.
//
// # Returns
//
// The pointer, if allocation was successful, and NULL otherwise.
// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#OS%20Memory%20Management>
// pub fn sp_calloc(nmemb: usize, size: usize) -> *mut c_void;
// pub fn sp_malloc(size: usize) -> *mut c_void;
// pub fn sp_mutex_lock(pmx: *mut SP_mutex) -> c_int;
// pub fn sp_mutex_unlock(pmx: *mut SP_mutex) -> c_int;
// pub fn sp_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
// pub fn sp_unregister_atom(atom: SP_atom) -> c_int;
// pub fn sp_strdup(str: *const c_char) -> *mut c_void;
// pub fn sp_register_atom(name: *const c_char, atom: *mut SP_atom) -> c_int;
