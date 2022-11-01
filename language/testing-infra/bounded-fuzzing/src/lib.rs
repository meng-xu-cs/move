use std::ffi::c_ulong;
use std::slice;

use move_binary_format::CompiledModule;
use move_bytecode_verifier::verifier::verify_module;

/// The actual
#[no_mangle]
pub extern "C" fn process(bytes: *const u8, len: c_ulong) -> bool {
    let binary = unsafe {
        assert!(!bytes.is_null());
        slice::from_raw_parts(bytes, len as usize)
    };
    CompiledModule::deserialize(binary).map_or(false, |m| verify_module(&m).is_ok())
}
