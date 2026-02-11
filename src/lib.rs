use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_void};
use xxhash_rust::xxh64::Xxh64;

// --- Core Structures ---

#[repr(C)]
pub struct HashContext {
    hasher: Xxh64,
    file: File,
    total: u64,
    processed: u64,
    buffer_size: usize,
    cancel: Arc<AtomicBool>,
}

pub type CProgressCallback = extern "C" fn(u64, u64, *mut c_void);

// --- FFI Implementation ---

#[no_mangle]
pub extern "C" fn hash_ffi_init(path: *const c_char, buf_size: u32) -> *mut HashContext {
    if path.is_null() { return std::ptr::null_mut(); }
    
    let path_str = match unsafe { CStr::from_ptr(path) }.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let file = match File::open(path_str) {
        Ok(f) => f,
        Err(_) => return std::ptr::null_mut(),
    };

    let total = file.metadata().map(|m| m.len()).unwrap_or(0);

    Box::into_raw(Box::new(HashContext {
        hasher: Xxh64::new(0),
        file,
        total,
        processed: 0,
        buffer_size: if buf_size == 0 { 65536 } else { buf_size as usize },
        cancel: Arc::new(AtomicBool::new(false)),
    }))
}

#[no_mangle]
pub extern "C" fn hash_ffi_process(ctx_ptr: *mut HashContext, cb: Option<CProgressCallback>, data: *mut c_void) -> c_int {
    if ctx_ptr.is_null() { return -1; }
    let ctx = unsafe { &mut *ctx_ptr };
    let mut buffer = vec![0u8; ctx.buffer_size];

    loop {
        if ctx.cancel.load(Ordering::Relaxed) { return -4; }
        
        match ctx.file.read(&mut buffer) {
            Ok(0) => return 0,
            Ok(n) => {
                ctx.hasher.update(&buffer[..n]);
                ctx.processed += n as u64;
                if let Some(callback) = cb { 
                    callback(ctx.processed, ctx.total, data); 
                }
            }
            Err(_) => return -5,
        }
    }
}

#[no_mangle]
pub extern "C" fn hash_ffi_finalize(ctx_ptr: *mut HashContext) -> u64 {
    if ctx_ptr.is_null() { return 0; }
    let ctx = unsafe { Box::from_raw(ctx_ptr) };
    ctx.hasher.digest()
}

#[no_mangle]
pub extern "C" fn hash_ffi_cancel(ctx_ptr: *mut HashContext) {
    if !ctx_ptr.is_null() {
        let ctx = unsafe { &*ctx_ptr };
        ctx.cancel.store(true, Ordering::Relaxed);
    }
}

#[no_mangle]
pub extern "C" fn hash_ffi_cleanup(ctx_ptr: *mut HashContext) {
    if !ctx_ptr.is_null() {
        unsafe { let _ = Box::from_raw(ctx_ptr); }
    }
}