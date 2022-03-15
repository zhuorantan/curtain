use std::mem;
use std::ptr;
use std::os::raw::c_int;
use core_foundation::base::*;
use core_foundation::array::*;
use core_foundation::string::*;
use core_foundation::number::*;
use core_foundation::dictionary::*;

mod cgs_internal;

fn get_session_id() -> c_int {
    let dict_ref = unsafe { cgs_internal::CGSCopyCurrentSessionDictionary() };

    let key = CFString::new("kCGSSessionIDKey");
    let mut number_ref: CFTypeRef = std::ptr::null();

    unsafe { CFDictionaryGetValueIfPresent(dict_ref, key.to_void(), &mut number_ref) };
    unsafe { CFRelease(dict_ref.to_void()) };

    let mut number: c_int = 0;
    unsafe { CFNumberGetValue(number_ref as CFNumberRef, kCFNumberIntType, mem::transmute(&mut number)) };

    number
}

fn lock_screen(msg: &str) {
    let session_id = get_session_id();

    let lock_cmd_args = [
        "/System/Library/CoreServices/RemoteManagement/AppleVNCServer.bundle/Contents/Support/LockScreen.app/Contents/MacOS/LockScreen",
        "-session",
        &session_id.to_string(),
        "-msg",
        msg,
    ];

    let lock_cmd_args: Vec<CFString> = lock_cmd_args.iter().map(|x| CFString::new(x)).collect();
    let lock_cmd_args: Vec<CFStringRef> = lock_cmd_args.iter().map(|x| unsafe { std::mem::transmute(x.to_void()) }).collect();

    let lock_cmd_args_array = unsafe { 
        CFArrayCreate(kCFAllocatorDefault,
                      lock_cmd_args.as_ptr() as *const CFTypeRef,
                      lock_cmd_args.len().to_CFIndex(),
                      ptr::null())
    };

    let mut out_session: c_int = 0;
    unsafe { cgs_internal::CGSSessionCreateSessionIDWithOptions(lock_cmd_args[0], lock_cmd_args_array, 3, &mut out_session) };
    unsafe { CFRelease(lock_cmd_args_array.to_void()) };
}

fn main() {
    lock_screen("foobar");
}
